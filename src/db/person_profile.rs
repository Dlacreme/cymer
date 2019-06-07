use diesel;
use diesel::prelude::*;
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

// pub fn update(co: &PgConnection, )