use crate::schema::employee;
use serde_derive::{Serialize, Deserialize};
use crate::model::person::Person;

#[derive(Debug, Queryable, Associations, AsChangeset, Serialize, Deserialize)]
#[belongs_to(Person)]
#[table_name = "employee"]
pub struct Employee {
    pub id: i32,
    pub person_id: i32,
    pub company_id: i32,
    pub is_disabled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "employee"]
pub struct InsertableEmployee {
    pub person_id: i32,
    pub company_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl InsertableEmployee {
    pub fn new(person_id: i32, company_id: i32) -> Self {
        Self {
            person_id: person_id,
            company_id: company_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}