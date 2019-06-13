use serde_derive::{Serialize, Deserialize};
use crate::model::employee_role::EmployeeRoleEnum;
use diesel::{PgConnection, QueryResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    person_id: i32,
    employee_id: i32,
    access: Vec<EmployeeRoleEnum>,
}

impl Employee {
    pub fn list_from_db(co: &PgConnection, company_id: i32) -> QueryResult<Vec<Self>> {

    }
}