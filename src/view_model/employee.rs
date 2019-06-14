use crate::model::employee_role::{to_enum, EmployeeRoleEnum};
use crate::schema_manual::employee;
use diesel;
use diesel::prelude::*;
use diesel::{sql_query, PgConnection, QueryResult};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    person_id: i32,
    employee_id: i32,
    access: Vec<EmployeeRoleEnum>,
}

impl Employee {
    pub fn list_from_db(co: &PgConnection, company_id: i32) -> QueryResult<Vec<Self>> {
        let emps: Vec<EmployeeDto> = sql_query(format!(
            "                                                                               \
             SELECT  e.id as employee_id,                                                   \
             e.person_id,                                                                   \
             e.company_id,                                                                  \
             e.is_disabled,                                                                 \
             array_to_string(array_agg(ea.employee_role_id), ',') as employee_role_id       \
             FROM employee e                                                                \
             INNER JOIN employee_access ea                                                  \
             ON e.id = ea.employee_id                                                       \
             WHERE e.company_id = {}                                                        \
             AND e.is_disabled = 'f'                                                        \
             GROUP BY e.id                                                                  \
             ",
            company_id
        ))
        .load(co)?;

        let mut res: Vec<Self> = Vec::new();
        for emp in emps {
            let access: Vec<EmployeeRoleEnum> = emp
                .employee_role_id
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .map(|y| to_enum(y))
                .collect();
            res.push(Self {
                person_id: emp.person_id,
                employee_id: emp.employee_id,
                access: access,
            })
        }

        QueryResult::Ok(res)
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "employee"]
struct EmployeeDto {
    person_id: i32,
    employee_id: i32,
    company_id: i32,
    is_disabled: bool,
    employee_role_id: String,
}
