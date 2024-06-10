/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This handler invokes the correlated model function to manipulate a resource in the database.
/// Resource: USER

///
use actix_web::{web, get, post, put, HttpResponse};
use crate::error_handler::CustomError;
use crate::models::user::{User, Users, PasswordChange};

/// Retrieve a list of resources
#[get("/users")]
async fn get_users() -> Result<HttpResponse, CustomError> {
    let users = Users::get_users()?;
    Ok(HttpResponse::Ok().json(users))
}

/// Retrieve details of a specific resource
#[get("/users/{id}")]
async fn get_user(path:web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let user_id = path.into_inner();
    let users = Users::get_user(user_id)?;
    Ok(HttpResponse::Ok().json(users))
}

/// Request to create a new resource
#[post("/users")]
async fn create_user(user:web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::create_user(user.into_inner())?;
    let user_uri = format!("/users/{}", user.username);
    Ok(HttpResponse::Created().append_header(("Location", user_uri)).finish())
}

/// Request to update an password
#[put("/users/{username}/password")]
async fn update_password(path: web::Path<String>, password_change: web::Json<PasswordChange>) -> Result<HttpResponse, CustomError> {
    let username = path.into_inner();
    let PasswordChange { old_password, new_password, new_password_check } = password_change.into_inner();
    let user = Users::update_password(&username, &old_password, &new_password, &new_password_check)?;
    Ok(HttpResponse::Ok().json(user))
}

// Initialize all routes of the resource
pub fn init_routes(config:&mut web::ServiceConfig) {
    config
        .service(get_users)
        .service(get_user)
        .service(create_user)
        .service(update_password)
    ;
}