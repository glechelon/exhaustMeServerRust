use crate::domain::user::User;
use crate::repositories::user_repository::UserRepository;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse, Responder};

#[get("/")]
pub async fn get_users(user_reposirory: Data<UserRepository>) -> impl Responder {
    let user = user_reposirory.get_users().await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[get("/{id}/")]
pub async fn get_user(
    Path(id): Path<String>,
    user_reposirory: Data<UserRepository>,
) -> impl Responder {
    let user = user_reposirory.get_user(&id).await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[post("/")]
pub async fn create_user(
    user: Json<User>,
    user_reposirory: Data<UserRepository>,
) -> impl Responder {
    let res = user_reposirory.create_user(&user).await;
    if res.is_ok() {
        return HttpResponse::Created();
    }
    HttpResponse::InternalServerError()
}

#[put("/{id}/")]
pub async fn modify_user(
    Path(id): Path<String>,
    user: Json<User>,
    user_reposirory: Data<UserRepository>,
) -> impl Responder {
    let res = user_reposirory.update_user(&id, &user).await;
    if res.is_ok() {
        return HttpResponse::Ok();
    }
    HttpResponse::InternalServerError()
}

#[delete("/{id}/")]
pub async fn delete_user(
    Path(id): Path<String>,
    user_reposirory: Data<UserRepository>,
) -> impl Responder {
    let res = user_reposirory.delete_user(&id).await;
    if res.is_ok() {
        return HttpResponse::NoContent();
    }
    HttpResponse::InternalServerError()
}
