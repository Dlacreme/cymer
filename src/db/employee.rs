use crate::model::employee::{Employee, InsertableEmployee};
use crate::model::employee_access::EmployeeAccess;
use crate::model::employee_role::{from_enum, EmployeeRoleEnum};
use crate::schema::employee as sEmployee;
use crate::schema::employee_access as sEmployeeAccess;
use diesel;
use diesel::prelude::*;

pub fn find(co: &PgConnection, id: i32) -> QueryResult<Employee> {
    sEmployee::table.find(id).get_result(co)
}

pub fn disable(co: &PgConnection, id: i32) -> QueryResult<Employee> {
    diesel::update(sEmployee::table.filter(sEmployee::id.eq(id)))
        .set(sEmployee::is_disabled.eq(true))
        .get_result(co)
}

pub fn get_from_company(co: &PgConnection, company_id: i32) -> QueryResult<Vec<Employee>> {
    sEmployee::table
        .filter(sEmployee::company_id.eq(company_id))
        .load(co)
}

pub fn create(
    co: &PgConnection,
    person_id: i32,
    company_id: i32,
    role: EmployeeRoleEnum,
) -> QueryResult<Employee> {
    let employee = InsertableEmployee::new(person_id, company_id);
    let employee_row: Employee = diesel::insert_into(sEmployee::table)
        .values(&employee)
        .get_result(co)?;
    let employee_access = EmployeeAccess::new(employee_row.id, from_enum(role));
    diesel::insert_into(sEmployeeAccess::table)
        .values(&employee_access)
        .get_result::<EmployeeAccess>(co)?;
    QueryResult::Ok(employee_row)
}
