use serde_derive::{Serialize, Deserialize};
use crate::schema::access;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "access"]
pub struct Access {
    pub id: i32,
    pub label: String,
}
