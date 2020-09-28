use actix_web::{middleware, web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::time::Duration;
use users::user_controller;
use users::users_state::{UserState};
use app::app_state::{AppState};
extern crate env_logger;

mod users;
mod app;

const URL_DB: &str = "mysql://exhaustMeServer:pwdExhaustMeServer@localhost:3306/EXHAUST_ME";


#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    let app_state = init_application().await;
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::default())
            .data(app_state.clone())
            .service(
                web::scope("/users")
                    .service(user_controller::get_users)
                    .service(user_controller::get_user)
                    .service(user_controller::create_user)
                    .service(user_controller::modify_user)
                    .service(user_controller::delete_user),
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await?;

    Ok(())
}

async fn create_connection_pool() -> Box<Pool<MySql>> {
    //Creation of a connection pool to the database
    let db_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(&URL_DB)
        .await
        .unwrap();
    //Trying to acquire a connection from the pool in order to test that
    db_pool
        .try_acquire()
        .expect("Imossible to establish a connection to the database.");
    return Box::new(db_pool);
}

async fn init_application() -> AppState {
    let db_pool_pointer = create_connection_pool().await;
    //Create application state
    let user_repository = users::user_repository::UserRepository {
        db_pool: db_pool_pointer,
    };

    let user_state: UserState = UserState {
        user_repository: user_repository,
    };
    return AppState {
        user_state: user_state,
    };
}
