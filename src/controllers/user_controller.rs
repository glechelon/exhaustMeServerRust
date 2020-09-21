use crate::domain::user::User;
use crate::repositories::user_repository;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use sqlx::{MySql, Pool};

#[get("/")]
pub async fn get_users(db: Data<Pool<MySql>>) -> impl Responder {
    let user = user_repository::get_users(&db).await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[get("/{id}/")]
pub async fn get_user(Path(id): Path<String>, db: Data<Pool<MySql>>) -> impl Responder {
    let user = user_repository::get_user(&db, &id).await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[post("/")]
pub async fn create_user(user: Json<User>, db: Data<Pool<MySql>>) -> impl Responder {
    let res = user_repository::create_user(&db, &user).await;
    if res.is_ok() {
        return HttpResponse::Created();
    }
    HttpResponse::InternalServerError()
}

#[put("/{id}/")]
pub async fn modify_user(
    Path(id): Path<String>,
    user: Json<User>,
    db: Data<Pool<MySql>>,
) -> impl Responder {
    let res = user_repository::update_user(&db, &id, &user).await;
    if res.is_ok() {
        return HttpResponse::Ok();
    }
    HttpResponse::InternalServerError()
}

#[delete("/{id}/")]
pub async fn delete_user(Path(id): Path<String>, db: Data<Pool<MySql>>) -> impl Responder {
    let res = user_repository::delete_user(&db, &id).await;
    if res.is_ok() {
        return HttpResponse::NoContent();
    }
    HttpResponse::InternalServerError()
}
