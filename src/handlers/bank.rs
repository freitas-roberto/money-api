use actix_web::{web,get,post,delete,put,HttpResponse};
use crate::{models::bank::{NewBank},repository::database::Database};

#[get("/banks")]
async fn get_banks(db:web::Data<Database>)->HttpResponse{
    let banks = db.get_banks();
    HttpResponse::Ok().json(banks)
}

#[get("/banks/{id}")]
async fn get_bank(db:web::Data<Database>, path:web::Path<i32>)->HttpResponse{
    let bank = db.get_bank(path.into_inner());
    match bank {
        Some(bank)=>HttpResponse::Ok().json(bank),
        None=>HttpResponse::NotFound().body("Not Found")
    }
}

#[post("/banks")]
async fn create_bank(db:web::Data<Database>, bank:web::Json<NewBank>)->HttpResponse{
    let bank = db.create_bank(bank.into_inner());
    match bank {
        Ok(bank)=>HttpResponse::Ok().json(bank),
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[put("/banks/{id}")]
async fn update_bank(db:web::Data<Database>, id: web::Path<i32>, bank:web::Json<NewBank>)->HttpResponse{
    let bank = db.update_bank(id.into_inner(), bank.into_inner());
    match bank {
        Ok(bank)=>HttpResponse::Ok().json(bank),
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

#[delete("/banks/{id}")]
async fn delete_bank(db:web::Data<Database>, path:web::Path<i32>)->HttpResponse{
    let bank = db.delete_bank(path.into_inner());
    match bank {
        Ok(bank)=>HttpResponse::Ok().json(bank),
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

pub fn init_routes(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(get_banks)
            .service(get_bank)
            .service(create_bank)
            .service(update_bank)
            .service(delete_bank)
    );
}