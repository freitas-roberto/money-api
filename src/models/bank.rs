/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This model is responsible for managing the resource in the database
/// Resource: BANK
/// 
use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, QueryDsl, Selectable};
use chrono::NaiveDateTime;

use crate::models::schema::banks;
use crate::error_handler::CustomError;
use crate::repository::database::Database;

// Bank structure
#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::banks)]
pub struct Banks {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable, Queryable, Selectable)]
#[diesel(table_name=crate::models::schema::banks)]
pub struct Bank {
    pub code: String,
    pub name: String,
}

/// Implements methods responsible for manipulating the resource
impl Banks {

    /// Retrieves a list of available resources
    pub fn get_banks() -> Result<Vec<Self>, CustomError> {
        let banks = banks::table.load::<Banks>(&mut Database::new().pool.get().unwrap())?;
        Ok(banks)
    }

    /// Retrieves details of a specific resource 
    pub fn get_bank(id: i32) -> Result<Self, CustomError> {
        let banks = banks::table
            .filter(banks::id.eq(id))
            .first::<Banks>(&mut Database::new().pool.get().unwrap())?;
        Ok(banks)
    }

    /// Creates a new resource
    pub fn create_bank(bank: Bank) -> Result<Self, CustomError> {
        let bank = Bank::from(bank);
        let bank = diesel::insert_into(banks::table)
            .values(bank)
            .get_result(&mut Database::new().pool.get().unwrap())?;
        Ok(bank)
    }

    /// Updates an existing resource
    pub fn update_bank(id: i32, bank: Bank) -> Result<Self, CustomError> {
        let bank = diesel::update(banks::table)
            .filter(banks::id.eq(id))
            .set(bank)
            .get_result(&mut Database::new().pool.get().unwrap());

        match bank {
            Ok(bank) => Ok(bank),
            Err(diesel::NotFound) => Err(CustomError::new(404,"Not found".to_string())),
            Err(_) => Err(CustomError::new(500,"Internal error".to_string()))
        }
    }

    /// Deletes a resource
    pub fn delete_bank(id: i32) -> Result<Option<usize>, CustomError> {
        let res = diesel::delete(banks::table
            .filter(banks::id.eq(id)))
            .execute(&mut Database::new().pool.get().unwrap())?;
        if res == 0 {
            Err(CustomError::new(404,"Not found".to_string()))
        } else {
            Ok(Some(res))
        }
    }    

}