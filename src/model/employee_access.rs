use serde_derive::{Serialize, Deserialize};
use crate::schema::employee_access;
use crate::model::employee::Employee;
use crate::model::employee_role::EmployeeRole;

#[derive(Queryable, Associations, Insertable, Serialize, Deserialize)]
#[belongs_to(Employee)]
#[belongs_to(EmployeeRole)]
#[table_name = "employee_access"]
pub struct EmployeeAccess {
    pub employee_id: i32,
    pub employee_role_id: i32,
}

impl EmployeeAccess {
    pub fn new(employee_id: i32, role_id: i32) -> Self {
        Self {
            employee_id,
            employee_role_id: role_id,
        }
    }
}
