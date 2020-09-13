use crate::repositories::user_repository::UserRepository;
use actix_web::{HttpResponse, Responder, web};

use actix_web::get;

#[get("/{id}/")]
pub async fn get_user(web::Path(id): web::Path<String>, user_reposirory: web::Data<UserRepository>) -> impl Responder {
    println!("get one");
    let user = user_reposirory.get_user(&id).await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    println!("Error");
    return HttpResponse::NotFound().finish();
}


#[get("/")]
pub async fn get_users(user_reposirory: web::Data<UserRepository>) -> impl Responder {
    println!("get all");
    let user = user_reposirory.get_users().await;
    if user.is_ok() {
        return HttpResponse::Ok().json(user.unwrap());
    }
    println!("Error");
    return HttpResponse::NotFound().finish();
}
