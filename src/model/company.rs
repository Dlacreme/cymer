use crate::schema::company;
use serde_derive::{Serialize, Deserialize};
use crate::model::person::Person;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Person, foreign_key="created_by_id")]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub label: String,
    pub created_by_id: i32,
    pub is_disabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
