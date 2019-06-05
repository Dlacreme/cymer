use serde_derive::{Serialize, Deserialize};
use crate::schema::employee_role;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "employee_role"]
pub struct EmployeeRole {
    pub id: i32,
    pub label: String,
}
