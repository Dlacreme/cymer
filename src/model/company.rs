use crate::model::person::Person;
use crate::schema::company;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Person, foreign_key = "created_by_id")]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub label: String,
    pub created_by_id: i32,
    pub is_disabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "company"]
pub struct InsertableCompany {
    pub label: String,
    pub created_by_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl InsertableCompany {
    pub fn new(label: &str, created_by_id: i32) -> Self {
        Self {
            label: String::from(label),
            created_by_id: created_by_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
