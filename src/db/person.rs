use super::person_profile;
use crate::model::person::{InsertablePerson, Person};
use crate::model::person_role::{from_enum, PersonRoleEnum};
use crate::schema::person as sPerson;
use bcrypt::verify;
use diesel;
use diesel::prelude::*;

pub fn find(co: &PgConnection, id: i32) -> QueryResult<Person> {
    sPerson::table.find(id).get_result(co)
}

pub fn get_by_email(co: &PgConnection, email: &str) -> QueryResult<Person> {
    sPerson::table.filter(sPerson::email.eq(email)).first(co)
}

pub fn get_by_credentials(
    co: &PgConnection,
    email: &str,
    password: &str,
) -> Result<Person, diesel::result::Error> {
    let ps: Person = sPerson::table
        .filter(sPerson::email.eq(email))
        // .filter(sPerson::email.eq(email).and(sPerson::password.eq(password)))
        .first(co)?;

    match verify(&ps.password, password) {
        Ok(_) => Result::Ok(ps),
        Err(_) => Result::Err(diesel::result::Error::NotFound),
    }
}

pub fn create(co: &PgConnection, email: &str, password: &str) -> QueryResult<Person> {
    let profile = person_profile::create(co, email)?;
    let person = InsertablePerson::new(email, password, profile.id);
    diesel::insert_into(sPerson::table)
        .values(&person)
        .get_result::<Person>(co)
}

pub fn update_role(co: &PgConnection, person_id: i32, role: PersonRoleEnum) -> QueryResult<Person> {
    diesel::update(sPerson::table.filter(sPerson::id.eq(person_id)))
        .set(sPerson::person_role_id.eq(from_enum(role)))
        .get_result(co)
}

pub fn update_active_company(
    co: &PgConnection,
    person_id: i32,
    company_id: i32,
) -> QueryResult<Person> {
    diesel::update(sPerson::table.filter(sPerson::id.eq(person_id)))
        .set(sPerson::active_company_id.eq(company_id))
        .get_result(co)
}
