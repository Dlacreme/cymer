use crate::schema::person;
use serde_derive::{Serialize, Deserialize};
use crate::model::person_profile::PersonProfile;
use crate::model::access::Access;
use bcrypt::hash;

const HASH_COMPLEX: u32 = 4;
const DEFAUT_ACCESS_ID: i32 = 1;

#[derive(Debug, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Access)]
#[belongs_to(PersonProfile)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub access_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub person_profile_id: i32,
    pub active_company_id: Option<i32>,
    pub notif_counter: i32,
}

#[derive(Insertable)]
#[table_name = "person"]
pub struct InsertablePerson {
    pub access_id: i32,
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
            access_id: 1,
            created_at: chrono::Utc::now().naive_utc(),
            notif_counter: 0,
            person_profile_id: person_profile_id,
        }
    }
}
