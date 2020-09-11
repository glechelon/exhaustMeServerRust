mod controllers;
mod domain;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(controllers::user_controller::get_users)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}