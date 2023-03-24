//! Webserver

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;
use actix_web::http::header;


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
        )
    })
    .bind(("127.0.0.1", 4300))?
    .run()
    .await
}
