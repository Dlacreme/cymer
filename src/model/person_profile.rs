use serde_derive::{Serialize, Deserialize};
use crate::schema::person_profile;

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
}

impl InsertablePersonProfile {

    pub fn new(email: &str) -> Self {
        Self {
            email: String::from(email),
            firstname: String::from(""),
            lastname: String::from(""),
            phone_number: String::from(""),
        }
    }

}