//! Webserver
extern crate argon2;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use actix_cors::Cors;


use database::*;
pub mod database;
pub mod tokenservice;


// static conn : Connection = database::connect();

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}


async fn create_user(request: web::Json<User>) -> impl Responder{
    println!("create user");
    let res = database::add_user(&request.username, &request.password);
    match res {
        Ok(r) => HttpResponse::Ok().body("true"),
        Err(_) => HttpResponse::BadRequest().body("false"),
    };
    // database::create_user(&request.username, &request.email,&request.password);
    HttpResponse::Ok().body("true")
}

async fn del_user(request: web::Json<User>) -> impl Responder {
    let res = database::delete_user(&request.username);

    match res {
        Ok(r) => HttpResponse::Ok().body("true"),
        Err(_) => HttpResponse::BadRequest().body("false"),
    };

    HttpResponse::Ok().body("true")

}

fn get_user(){
    // TODO: DB call to select user + specific token
}

fn get_user_token(){
    // TODO: receive unique token and return
}

async fn login(request: web::Json<User>) -> impl Responder{
    let res = database::check_creds(&request.username, &request.password);

    println!("{}", res.as_ref().unwrap());
    let resp_message: bool;
    match res {
        Ok(r) => resp_message = r,
        Err(_) => resp_message = false,
    };

    if resp_message {
        HttpResponse::Ok().body("true")
    } else {
        HttpResponse::Ok().body("false")

    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let res = database::create_db_tables();
    println!("INFO: Server started");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // TODO: WARNING: not for production!!!
            .service(
            web::scope("/api")
                .route("/login", web::post().to(login))
                .route("/create", web::post().to(create_user))
                .route("/delete", web::post().to(del_user))
        )
    })
    .bind(("127.0.0.1", 4300))?
    .run()
    .await
}