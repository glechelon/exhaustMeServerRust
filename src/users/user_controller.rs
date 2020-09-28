use super::user::User;
use crate::app::app_state::{AppState};
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use log::{info, warn, error};

#[get("/")]
pub async fn get_users(app_state: Data<AppState>) -> impl Responder {
    let users = app_state.user_state.user_repository.get_users().await;
    if users.is_ok() {
        return HttpResponse::Ok().json(users.unwrap());
    }
    error!("{:?}", users.err());
    HttpResponse::NotFound().finish()
}

#[get("/{id}/")]
pub async fn get_user(Path(id): Path<String>, app_state: Data<AppState>) -> impl Responder {
    let user = app_state.user_state.user_repository.get_user(&id).await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    error!("{:?}", user.err());
    HttpResponse::NotFound().finish()
}

#[post("/")]
pub async fn create_user(user: Json<User>, app_state: Data<AppState>) -> impl Responder {
    let res = app_state
        .user_state
        .user_repository
        .create_user(&user)
        .await;
    if res.is_ok() {
        return HttpResponse::Created();
    }
    HttpResponse::InternalServerError()
}

#[put("/{id}/")]
pub async fn modify_user(
    Path(id): Path<i64>,
    user: Json<User>,
    app_state: Data<AppState>,
) -> impl Responder {
    let res = app_state
        .user_state
        .user_repository
        .update_user(&id, &user)
        .await;
    if res.is_ok() {
        return HttpResponse::Ok();
    }
    HttpResponse::InternalServerError()
}

#[delete("/{id}/")]
pub async fn delete_user(Path(id): Path<i64>, app_state: Data<AppState>) -> impl Responder {
    let res = app_state.user_state.user_repository.delete_user(&id).await;
    if res.is_ok() {
        return HttpResponse::NoContent();
    }
    HttpResponse::InternalServerError()
}
