/// Author: Roberto Freitas
/// Version: 1.0.0
/// 
/// Objective: This model is responsible for managing the resource in the database
/// Resource: USER
/// 
use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, ExpressionMethods, Insertable, Queryable, RunQueryDsl, QueryDsl, Selectable};
use chrono::NaiveDateTime;

use crate::models::schema::users;
use crate::error_handler::CustomError;
use crate::repository::database::Database;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable, Queryable, Selectable)]
#[diesel(table_name=crate::models::schema::users)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordChange {
    pub old_password: String,
    pub new_password: String,
    pub new_password_check: String,
}

/// Implements methods responsible for manipulating the resource
impl Users {

    /// Retrieves a list of available resources
    pub fn get_users() -> Result<Vec<UserResponse>, CustomError> {
        let users = users::table.load::<Users>(&mut Database::new().pool.get().unwrap())?;
        let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
                id: user.id,
                username: user.username,
                is_active: user.is_active,
        })
        .collect();
        Ok(user_responses)
    }

    /// Retrieves details of a specific resource 
    pub fn get_user(id: i32) -> Result<UserResponse, CustomError> {
        let users = users::table
            .filter(users::id.eq(id))
            .first::<Users>(&mut Database::new().pool.get().unwrap())?;
        let user_response = UserResponse { 
            id: users.id,
            username: users.username,
            is_active: users.is_active,
        };
        Ok(user_response)       
    }

    /// Creates a new resource 
    pub fn create_user(user: User) -> Result<Self, CustomError> {
        let mut user = User::from(user);

        user.password = Self::hash_password(&user.password)?;
        
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut Database::new().pool.get().unwrap())?;
        Ok(user)
    }

    /// Updates the user’s password
    pub fn update_password(username: &str, old_password: &str, new_password: &str, new_password_check: &str) -> Result<Self, CustomError> {
        // find the user in a database
        let mut user = users::table
            .filter(users::username.eq(username))
            .first::<Users>(&mut Database::new().pool.get().unwrap())?;
        
        // Verifies that the password entered matches the stored password
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password)?;
        let old_password_matches = argon2.verify_password(old_password.as_bytes(), &parsed_hash).is_ok();
        if !old_password_matches {
            return Err(CustomError::new(400, "Old password does not match".to_string()));
        }
        
        // Check new password
        if new_password != new_password_check {
            return Err(CustomError::new(400, "New password and confirmation do not match".to_string()));
        }
        
        // Encrypt the new password
        user.password = Self::hash_password(new_password)?;
        
        // Update user in database
        let user = diesel::update(users::table.filter(users::username.eq(username)))
            .set(user)
            .get_result(&mut Database::new().pool.get().unwrap())?;
        
        Ok(user)
    }
    
    /// Receives the password and generate a hash for it
    fn hash_password(password: &str) -> Result<String, CustomError> {
        let salt = SaltString::generate(&mut OsRng);
        
        // Use Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
        Ok(password_hash)
    }

    /// Check if username and password is valid
    pub fn verify_user(username: &str, password: &str) -> Result<bool, CustomError> {
        
        // find user
        let user = users::table
            .filter(users::username.eq(username))
            .first::<Users>(&mut Database::new().pool.get().unwrap())?;
        
        // Check if the password is valid
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password)?;
        let matches = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
        Ok(matches)
    }

}