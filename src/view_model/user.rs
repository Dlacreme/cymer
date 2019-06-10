use serde_derive::{Serialize, Deserialize};
use diesel::{PgConnection, QueryResult};
use crate::parser::validator;
use crate::model::person_role::{PersonRoleEnum, to_enum};
use crate::db;
use super::company::Company;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub role: PersonRoleEnum,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
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
            firstname: profile.firstname,
            lastname: profile.lastname,
            phone_number: profile.phone_number,
            member_since: person.created_at,
            notif_counter: person.notif_counter,
            active_company: company,
        })
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToUpdate {
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<PersonRoleEnum>,
}

impl UserToUpdate {
    pub fn validate(&self) -> Result<(), &str> {
        if self.email.is_some() {
            validator::is_email(self.email.as_ref().unwrap())?;
        }
        if self.firstname.is_some() {
            validator::is_alpha(self.firstname.as_ref().unwrap())?;
        }
        if self.email.is_some() {
            validator::is_alpha(self.lastname.as_ref().unwrap())?;
        }
        if self.email.is_some() {
            validator::is_num(self.phone_number.as_ref().unwrap())?;
        }

        Result::Ok(())
    }
}
