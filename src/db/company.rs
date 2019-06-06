use diesel;
use diesel::prelude::*;
use crate::model::company::{Company};
use crate::schema::company as sCompany;

pub fn find(co: &PgConnection, id: i32) -> QueryResult<Company> {
    sCompany::table.find(id).get_result(co)
}
