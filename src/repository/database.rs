use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::bank::{Bank, NewBank};
use crate::models::schema::banks::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database{
    pub pool: DBPool
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create a database pool.");

        Database { pool : result }
    }

    pub fn get_banks(&self) -> Vec<Bank> {
        banks
            .load::<Bank>(&mut self.pool.get().unwrap())
            .expect("Failed to get banks")
    }

    pub fn get_bank(&self, find_id: i32) -> Option<Bank> {
        banks
            .find(find_id)
            .first::<Bank>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn create_bank(&self, bank: NewBank) -> Result<Bank, diesel::result::Error> {
        diesel::insert_into(banks)
            .values(&bank)
            .get_result(&mut self.pool.get().unwrap())
    }

    pub fn update_bank(&self, find_id: i32, bank: NewBank) -> Result<Bank, diesel::result::Error>{
        diesel::update(banks
            .filter(id.eq(find_id)))
            .set(&bank)
            .get_result(&mut self.pool.get().unwrap())
    }

    pub fn delete_bank(&self, find_id: i32) -> Result<usize,diesel::result::Error> {
        diesel::delete(banks
            .filter(id.eq(find_id)))
            .execute(&mut self.pool.get().unwrap())
    }

}