use crate::model::company::{Company, InsertableCompany};
use crate::schema::company as sCompany;
use diesel;
use diesel::prelude::*;

pub fn find(co: &PgConnection, id: i32) -> QueryResult<Company> {
    sCompany::table.find(id).get_result(co)
}

pub fn create(co: &PgConnection, label: &str, created_by_id: i32) -> QueryResult<Company> {
    let company = InsertableCompany::new(label, created_by_id);
    diesel::insert_into(sCompany::table)
        .values(&company)
        .get_result::<Company>(co)
}
