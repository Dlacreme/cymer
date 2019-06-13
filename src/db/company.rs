use crate::db;
use crate::model::company::{Company, InsertableCompany};
use crate::model::employee_role::EmployeeRoleEnum;
use crate::schema::company as sCompany;
use diesel;
use diesel::prelude::*;

pub fn find(co: &PgConnection, id: i32) -> QueryResult<Company> {
    sCompany::table.find(id).get_result(co)
}

pub fn create(co: &PgConnection, label: &str, created_by_id: i32) -> QueryResult<Company> {
    let company = InsertableCompany::new(label, created_by_id);
    let company_row = diesel::insert_into(sCompany::table)
        .values(&company)
        .get_result::<Company>(co)?;
    db::employee::create(co, created_by_id, company_row.id, EmployeeRoleEnum::Admin)?;
    db::person::update_active_company(co, created_by_id, company_row.id)?;
    QueryResult::Ok(company_row)
}

pub fn update(
    co: &PgConnection,
    id: i32,
    input: crate::view_model::company::CompanyToUpdate,
) -> QueryResult<Company> {
    let mut initial_row = find(co, id)?;
    initial_row.updated_at = chrono::Utc::now().naive_utc();
    if input.label.is_some() {
        initial_row.label = input.label.unwrap();
    }
    diesel::update(sCompany::table.filter(sCompany::id.eq(id)))
        .set(&initial_row)
        .get_result(co)
}
