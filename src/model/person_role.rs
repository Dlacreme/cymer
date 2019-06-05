use serde_derive::{Serialize, Deserialize};
use crate::schema::person_role;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "person_role"]
pub struct PersonRole {
    pub id: i32,
    pub label: String,
}
