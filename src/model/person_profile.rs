use crate::schema::person_profile;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "person_profile"]
pub struct PersonProfile {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone_number: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "person_profile"]
pub struct InsertablePersonProfile {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone_number: String,
    pub updated_at: chrono::NaiveDateTime,
}

impl InsertablePersonProfile {
    pub fn new(email: &str) -> Self {
        Self {
            email: String::from(email),
            firstname: String::from(""),
            lastname: String::from(""),
            phone_number: String::from(""),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
