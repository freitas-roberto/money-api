/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This handler invokes the correlated model function to manipulate a resource in the database.
/// Resource: AGENCY

///
use actix_web::{web, get, post, put, delete, HttpResponse};
use crate::error_handler::CustomError;
use crate::models::agency::{Agency, Agencies};

/// Retrieve a list of resources
#[get("/banks/{bank_id}/agencies")]
async fn get_agencies(path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let agencies = Agencies::get_agencies(path.into_inner())?;
    Ok(HttpResponse::Ok().json(agencies))
}

/// Retrieve details of a specific resource
#[get("/banks/{bank_id}/agencies/{id}")]
async fn get_agency(path:web::Path<(i32,i32)>) -> Result<HttpResponse, CustomError> {
    let (bank_id, id) = path.into_inner();
    let agencies = Agencies::get_agency(bank_id, id)?;
    Ok(HttpResponse::Ok().json(agencies))
}

/// Request to create a new resource
#[post("/banks/{bank_id}/agencies")]
async fn create_agency(path: web::Path<i32>,mut agency: web::Json<Agency>) -> Result<HttpResponse, CustomError> { 
    let bank_id = path.into_inner();
    agency.bank_id = bank_id;
    let agency = Agencies::create_agency(agency.into_inner())?;
    let agency_uri = format!("/banks/{}/agencies/{}", bank_id, agency.id);
    Ok(HttpResponse::Created().append_header(("Location", agency_uri)).finish())
}

/// Request to update an existing resource
#[put("/banks/{bank_id}/agencies/{id}")]
async fn update_agency(path: web::Path<(i32, i32)>, agency:web::Json<Agency>) -> Result<HttpResponse, CustomError> {
    let (bank_id, id) = path.into_inner();
    let agency = Agencies::update_agency(bank_id, id, agency.into_inner())?;
    Ok(HttpResponse::Ok().json(agency))
}

/// Request to delete a resource
#[delete("/banks/{bank_id}/agencies/{id}")]
async fn delete_agency(path: web::Path<(i32, i32)>) -> Result<HttpResponse, CustomError> {
    let (bank_id, id) = path.into_inner();
    Agencies::delete_agency(bank_id, id)?;
    Ok(HttpResponse::NoContent().finish())
}

// Initialize all routes of the resource
pub fn init_routes(config:&mut web::ServiceConfig) {
    config
        .service(get_agencies)
        .service(get_agency)
        .service(create_agency)
        .service(update_agency)
        .service(delete_agency)
    ;
}