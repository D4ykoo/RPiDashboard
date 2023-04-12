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

struct User {
    username: String,
    password: String
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

fn connect() -> Result<Connection, rusqlite::Error>{
    return Connection::open("dashboard.db");
}


fn exists(name: &str, conn: &Connection) -> Result<bool>{
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
    stmt.execute([name.to_string()])?;

    return Ok(true)
}

pub fn add_user(name: &str, pw: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();
    
    if !exists(name, &conn).unwrap() {return Err(QueryReturnedNoRows)};

    let hashed_pw = hash_password(pw);

    let mut stmt = conn.prepare("INSERT INTO users (username, password) values (?1, ?2)",)?;
    stmt.execute([name.to_string(), hashed_pw.to_string()])?;

    return Ok(true)

}

pub fn check_creds(name: &str, pw: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();
    let hashed_pw = hash_password(pw);

    if !exists(name, &conn).unwrap() {return Err(QueryReturnedNoRows)};
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
    stmt.execute([name.to_string()])?;

    let users = stmt.query_row((), |row| {
        Ok(User {
            username: row.get(1)?,
            password: row.get(2)?,
        })
    });

    if let Ok(user) = users{
        if user.password == hashed_pw{ return Ok(true)}
    }

    return Ok(true)
}

pub fn delete_user(name: &str) -> Result<bool, rusqlite::Error>{
    let conn = connect().unwrap();
    
    if !exists(name, &conn).unwrap() {return Err(QueryReturnedNoRows)};

    let mut stmt = conn.prepare("DELETE FROM users WHERE username = ?1")?;
    stmt.execute([name.to_string()])?;

    return Ok(true);
}

fn hash_password(password: &str) -> &str {
    let mut hashed = String::from("");
    password
}
