use actix_web::guard::Connect;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};

use dotenvy::dotenv;
use std::env;
use std::env::args;
use rusqlite::Error::QueryReturnedNoRows;
use argon2::{self, Config, Version, Variant};

#[derive(Debug)]
struct User {
    username: String,
    password: String
}

#[derive(Debug)]
struct ExistUser{
    username: String,
}

pub fn create_db_tables() -> Result<()>{
    let conn = Connection::open("dashboard.db")?;
    conn.execute_batch(
        "create table if not exists users (
             id integer primary key,
             username text not null unique,
             password text not null
         )",
        
    )?;
    conn.execute_batch(
        "create table if not exists information (
             id integer primary key,
             title text not null,
             description text not null,
             link text not null,
             image text not null
         )"
    )?;
    Ok(())
}

pub fn connect() -> Result<Connection, rusqlite::Error>{
    return Connection::open("dashboard.db");
}


fn exists(name: &str, conn: &Connection) -> Result<bool>{
    let mut stmt = conn.prepare("SELECT username FROM users WHERE username = ?1")?;

    let users = stmt.query_row([name.to_string()], |row| {
        Ok(ExistUser{
            username: row.get(1)?
        }
        )
    });

    if users.is_ok(){
        return Ok(true)
    } else {
        return Ok(false)
    }
}

pub fn add_user(name: &str, pw: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();
    let res = exists(name, &conn).unwrap();

    if res {
        println!("user already exists");
        return Err(rusqlite::Error::ExecuteReturnedResults)
    };

    let hashed_pw = hash_password(pw);

    let mut stmt = conn.prepare("INSERT INTO users (username, password) values (?1, ?2)",)?;
    stmt.execute([name.to_string(), hashed_pw.to_string()])?;
    println!("user created");
    return Ok(true)

}

pub fn check_creds(name: &str, pw: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();

    if exists(name, &conn).unwrap() {return Ok(false)};

    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
    // stmt.execute([name.to_string()])?;

    let users = stmt.query_row([name.to_string()], |row| {
        Ok(User {
            username: row.get(1)?,
            password: row.get(2)?,
        })
    });

    if let Ok(user) = users{
        return Ok(check_password(&user.password, pw));
    } else {
        println!("{}", ("shitty"));
        return Ok(false)
    }
}

pub fn delete_user(name: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();
    
    if !exists(name, &conn).unwrap() {return Err(QueryReturnedNoRows)};

    let mut stmt = conn.prepare("DELETE FROM users WHERE username = ?1")?;
    stmt.execute([name.to_string()])?;

    return Ok(true);
}

fn argon_conf()-> argon2::Config<'static> {
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        thread_mode: argon2::ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32
    };

    return config;
}

fn check_password(password: &str, pw: &str) -> bool {
    let hash = hash_password(pw);
    if password.contains(&hash){return true};
    return false;
}

fn hash_password(password: &str) -> String {
    let salt = b"othersalt";
    let config = argon_conf();
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    hash
}
