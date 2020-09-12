use crate::domain::user::User;
use actix_web::{HttpResponse, Responder};

use actix_web::get;

#[get("/users")]
pub async fn get_users() -> impl Responder {
    return HttpResponse::Ok().json(User {
        identifier: String::from("test"),
        name: String::from("test"),
    });
}
