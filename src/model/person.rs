use crate::schema::person;
use serde_derive::{Serialize, Deserialize};
use crate::model::person_profile::PersonProfile;
use crate::model::person_role::PersonRole;
use bcrypt::hash;

const HASH_COMPLEX: u32 = 4;
const USER_ROLE_ID: i32 = 1;
// const ADMIN_ACCESS_ID: i32 = 2;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(PersonRole)]
#[belongs_to(PersonProfile)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub person_role_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub person_profile_id: i32,
    pub active_company_id: Option<i32>,
    pub notif_counter: i32,
    pub is_disabled: bool,
}

#[derive(Insertable)]
#[table_name = "person"]
pub struct InsertablePerson {
    pub person_role_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub notif_counter: i32,
    pub person_profile_id: i32,
}

impl InsertablePerson {
    pub fn new(email: &str, password: &str, person_profile_id: i32) -> Self {
        Self {
            email: String::from(email),
            password: hash(password, HASH_COMPLEX).unwrap(),
            person_role_id: USER_ROLE_ID,
            created_at: chrono::Utc::now().naive_utc(),
            notif_counter: 0,
            person_profile_id: person_profile_id,
        }
    }
}
