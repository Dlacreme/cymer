use serde_derive::{Serialize, Deserialize};
use crate::schema::profile;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "profile"]
pub struct Profile {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone_number: String,
}
