//! Webserver
extern crate argon2;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;


use database::*;
pub mod database;


#[derive(Debug, PartialEq, Eq)]
struct User {
    username: String,
    password: String,
}


async fn create_user() -> impl Responder{
    println!("stuff");
    // database::create_user(&request.username, &request.email,&request.password);
    HttpResponse::Ok().body("true")
}

async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("true")
}

fn get_user(){
    // TODO: DB call to select user + specific token
}

fn get_user_token(){
    // TODO: receive unique token and return
}

async fn login() -> impl Responder{
    HttpResponse::Ok().body("true")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("INFO: Server started");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // TODO: WARNING: not for production!!!
            .service(
            web::scope("/api")
                .route("/login", web::post().to(login))
                .route("/create", web::post().to(create_user))
        )
    })
    .bind(("127.0.0.1", 4300))?
    .run()
    .await
}
