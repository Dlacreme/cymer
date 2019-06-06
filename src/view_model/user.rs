use serde_derive::{Serialize, Deserialize};
use diesel::{PgConnection, sql_query, RunQueryDsl, QueryResult};
use crate::model::person_role::{PersonRoleEnum, to_enum};
use crate::db;
use super::company::Company;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub role: PersonRoleEnum,
    pub email: String,
    pub name: String,
    pub phone_number: String,
    pub member_since: chrono::NaiveDateTime,
    pub notif_counter: i32,
    pub active_company: Option<Company>,
}

impl User {

    pub fn from_db(conn: &PgConnection, person_id: i32) -> QueryResult<User> {
        let person = db::person::find(conn, person_id)?;
        let profile = db::person_profile::find(conn, person.person_profile_id)?;
        let company = match person.active_company_id {
            Some(id) => Some(Company::from_db(conn, id)?),
            None => None,
        };
        QueryResult::Ok(Self {
            id: person.id,
            role: to_enum(person.person_role_id),
            email: profile.email,
            name: format!("{} {}", profile.firstname, profile.lastname),
            phone_number: profile.phone_number,
            member_since: person.created_at,
            notif_counter: person.notif_counter,
            active_company: company,
        })
    }

}
