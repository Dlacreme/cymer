use crate::schema::employee;
use serde_derive::{Serialize, Deserialize};
use crate::model::person::Person;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Person)]
#[table_name = "employee"]
pub struct Employee {
    pub id: i32,
    pub person_id: i32,
    pub company_id: i32,
    pub is_disabled: bool,
}
