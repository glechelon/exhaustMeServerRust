use actix_web::{middleware, web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;

mod controllers;
mod domain;
mod repositories;

const URL_DB: &str = "mysql://guillaume:enzo1542@localhost:3306/EXHAUST_ME";

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&URL_DB)
        .await?;

    let user_repository = repositories::user_repository::UserRepository {
        db: db_pool.clone(),
    };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Always,
            ))
            .wrap(middleware::Logger::default())
            .data(user_repository.clone())
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
