use diesel;
use diesel::prelude::*;
use crate::view_model::user::UserToUpdate;
use crate::schema::person_profile as sProfile;
use crate::model::person_profile::{PersonProfile, InsertablePersonProfile};

pub fn find(co: &PgConnection, id: i32) -> QueryResult<PersonProfile> {
    sProfile::table.find(id).get_result::<PersonProfile>(co)
}

pub fn create(co: &PgConnection, email: &str) -> QueryResult<PersonProfile> {
    let profile = InsertablePersonProfile::new(email);
    diesel::insert_into(sProfile::table)
        .values(&profile)
        .get_result::<PersonProfile>(co)
}

pub fn update(co: &PgConnection, id: i32, user: UserToUpdate) -> QueryResult<PersonProfile> {
    let mut initial_row = find(co, id)?;
    initial_row.updated_at = chrono::Utc::now().naive_utc();
    if user.email.is_some() {
        initial_row.email = user.email.unwrap();
    }
    if user.firstname.is_some() {
        initial_row.firstname = user.firstname.unwrap();
    }
    if user.lastname.is_some() {
        initial_row.lastname = user.lastname.unwrap();
    }
    if user.phone_number.is_some() {
        initial_row.email = user.phone_number.unwrap();
    }
    println!("UPDATE {:?}", initial_row);
    diesel::update(sProfile::table.filter(sProfile::id.eq(id)))
        .set(&initial_row).get_result(co)
}
