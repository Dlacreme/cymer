use serde_derive::{Serialize, Deserialize};
use crate::schema::employee_access;
use crate::model::employee::Employee;
use crate::model::employee_role::EmployeeRole;

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Employee)]
#[belongs_to(EmployeeRole)]
#[table_name = "employee_access"]
pub struct PersoneAccess {
    pub employee_id: i32,
    pub employee_role_id: String,
}
