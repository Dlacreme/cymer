use crate::schema::person;
use serde_derive::{Serialize, Deserialize};
use crate::model::profile::Profile;
use crate::model::access::Access;

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Access)]
#[belongs_to(Profile)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub access_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub profile_id: i32,
    pub active_company_id: Option<i32>,
    pub notif_counter: i32,
}

#[derive(Insertable)]
#[table_name = "person"]
struct InsertablePerson {
    pub access_id: i32,
    pub email: String,
    pub password: String,
}

impl InsertablePerson {
    fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            email: person.email,
            password: person.password,
            access_id: person.access_id,

        }
    }
}