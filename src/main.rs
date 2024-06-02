use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Serialize;
use dotenv::dotenv;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
#[macro_use]
extern crate diesel_migrations;

mod handlers;
mod models;
mod repository;
mod error_handler;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

type DB=diesel::pg::Pg;
const MIGRATIONS:EmbeddedMigrations=embed_migrations!();

fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _=connection.run_pending_migrations(MIGRATIONS);
}   

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL").expect("LOG_LEVEL must be set.");
    let host = std::env::var("HOST").expect("HOST must be set.");
    let port = std::env::var("PORT").expect("PORT must be set.");
    let events_db = repository::database::Database::new();

    run_migrations(&mut events_db.pool.get().unwrap());

    let app_data = web::Data::new(events_db);
    
    env_logger::Builder::from_env(Env::default().default_filter_or(format!("{}",log_level))).init();
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(health)
            .service(web::scope("/api")
                .configure(handlers::bank::init_routes)
                .configure(handlers::agency::init_routes)
            )
            .wrap(actix_web::middleware::Logger::default())

    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await

}