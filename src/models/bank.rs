use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable,AsChangeset};
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::banks)]
pub struct Bank {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::banks)]
pub struct NewBank {
    pub code: String,
    pub name: String,
}