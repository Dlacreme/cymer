use diesel;
use diesel::prelude::*;
use crate::schema::person as sPerson;
use crate::model::person::Person;

pub fn get_by_credentials(co: &super::Conn, email: &str, password: &str) -> QueryResult<Person> {
    sPerson::table.find(1).get_result::<Person>(co)
}
