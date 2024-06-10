/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This handler invokes the correlated model function to manipulate a resource in the database.
/// Resource: BANK
///
use actix_web::{web, get, post, put, delete, HttpResponse};
use crate::error_handler::CustomError;
use crate::models::bank::{Bank, Banks};

/// Retrieve a list of resources
#[get("/banks")]
async fn get_banks() -> Result<HttpResponse, CustomError> {
    let banks = Banks::get_banks()?;
    Ok(HttpResponse::Ok().json(banks))
}

/// Retrieve details of a specific resource
#[get("/banks/{id}")]
async fn get_bank(path:web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let bank_id = path.into_inner();
    let banks = Banks::get_bank(bank_id)?;
    Ok(HttpResponse::Ok().json(banks))
}

/// Request to create a new resource
#[post("/banks")]
async fn create_bank(bank:web::Json<Bank>) -> Result<HttpResponse, CustomError> {
    let bank = Banks::create_bank(bank.into_inner())?;
    let bank_uri = format!("/banks/{}", bank.id);
    Ok(HttpResponse::Created().append_header(("Location", bank_uri)).finish())
}

/// Request to update an existing resource
#[put("/banks/{id}")]
async fn update_bank(path: web::Path<i32>, bank:web::Json<Bank>) -> Result<HttpResponse, CustomError>  {
    let bank = Banks::update_bank(path.into_inner(), bank.into_inner())?;
    Ok(HttpResponse::Ok().json(bank))
}

/// Request to delete a resource
#[delete("/banks/{id}")]
async fn delete_bank(path:web::Path<i32>) -> Result<HttpResponse, CustomError> {
    Banks::delete_bank(path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

// Initialize all routes of the resource
pub fn init_routes(config:&mut web::ServiceConfig){
    config
        .service(get_banks)
        .service(get_bank)
        .service(create_bank)
        .service(update_bank)
        .service(delete_bank)
    ;
}