use actix_web::{middleware, web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

extern crate env_logger;

mod controllers;
mod domain;
mod repositories;

const URL_DB: &str = "mysql://exhaustMeServer:pwdExhaustMeServer@localhost:3306/EXHAUST_ME";

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(&URL_DB)
        .await?;

    println!("{:?}", db_pool);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Always,
            ))
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(
                web::scope("/users")
                    .service(controllers::user_controller::get_users)
                    .service(controllers::user_controller::get_user)
                    .service(controllers::user_controller::create_user)
                    .service(controllers::user_controller::modify_user)
                    .service(controllers::user_controller::delete_user),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await?;
    Ok(())
}

