/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This model is responsible for managing the resource in the database
/// Resource: AGENCY
///
use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, QueryDsl, Selectable};
use chrono::NaiveDateTime;

use crate::models::schema::agencies;
use crate::error_handler::CustomError;
use crate::repository::database::Database;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::agencies)]
pub struct Agencies {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub bank_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable, Queryable, Selectable, PartialEq)]
#[diesel(table_name=crate::models::schema::agencies)]
pub struct Agency {
    pub code: String,
    pub name: String,
    pub bank_id: i32,
}

/// Implements methods responsible for manipulating the resource
impl Agencies {

    /// Retrieves a list of available resources
    pub fn get_agencies(bank_id: i32) -> Result<Vec<Self>, CustomError> {
        let agencies = agencies::table
            .filter(agencies::bank_id.eq(bank_id))
            .load::<Agencies>(&mut Database::new().pool.get().unwrap())?;
        Ok(agencies)
    }

    /// Retrieves details of a specific resource 
    pub fn get_agency(bank_id: i32, id: i32) -> Result<Vec<Self>, CustomError> {
        let agencies = agencies::table
            .filter(agencies::bank_id.eq(bank_id))
            .find(id)
            .load::<Agencies>(&mut Database::new().pool.get().unwrap())?;
        Ok(agencies)
    }

    /// Creates a new resource 
    pub fn create_agency(agency: Agency) -> Result<Self, CustomError> {
        let agency = Agency::from(agency);
        let agency = diesel::insert_into(agencies::table)
            .values(agency)
            .get_result(&mut Database::new().pool.get().unwrap())?;
        Ok(agency)
    }

    /// Updates an existing resource
    pub fn update_agency(bank_id: i32, id: i32, agency: Agency) -> Result<Self, CustomError> {

        let agency = diesel::update(agencies::table
            .filter(agencies::bank_id.eq(bank_id))
            .filter(agencies::id.eq(id)))
            .set(agency)
            .get_result(&mut Database::new().pool.get().unwrap());
        
        match agency {
            Ok(agency) => Ok(agency),
            Err(diesel::NotFound) => Err(CustomError::new(404, "Not found".to_string())),
            Err(_) => Err(CustomError::new(500, "Internal error".to_string()))
        }
    }

    /// Deletes a resource
    pub fn delete_agency(bank_id: i32, id: i32) -> Result<Option<usize> ,CustomError> {
        let res = diesel::delete(agencies::table
            .filter(agencies::bank_id.eq(bank_id))
            .filter(agencies::id.eq(id)))
            .execute(&mut Database::new().pool.get().unwrap())?;
        if res == 0 {
            Err(CustomError::new(404,"Not found".to_string()))
        } else{
            Ok(Some(res))
        }
    }

}