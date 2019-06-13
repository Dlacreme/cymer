use crate::schema::employee_role;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EmployeeRoleEnum {
    Admin = 1,
    Manager = 2,
    Employee = 3,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "employee_role"]
pub struct EmployeeRole {
    pub id: i32,
    pub label: String,
}

pub fn to_enum(id: i32) -> EmployeeRoleEnum {
    match id {
        1 => EmployeeRoleEnum::Admin,
        2 => EmployeeRoleEnum::Manager,
        3 => EmployeeRoleEnum::Employee,
        _ => unreachable!(),
    }
}

pub fn from_enum(role: EmployeeRoleEnum) -> i32 {
    match role {
        EmployeeRoleEnum::Admin => 1,
        EmployeeRoleEnum::Manager => 2,
        EmployeeRoleEnum::Employee => 3,
    }
}
