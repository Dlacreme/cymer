use crate::model::person;

pub struct Person {
    pub id: i32,
    pub access_id: i32,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub person_profile_id: i32,
    pub active_company_id: Option<i32>,
    pub notif_counter: i32,
}

impl Person {
    pub fn from_person(person: &Person) -> Self {
        Self {
            id, person.id,
            access_id: person.access_id,
            email: person.email,
            created_at: person.created_at,
            person_profile_id: person.created_at,
            active_company_id: person.active_company_id,
            notif_counter: person.notif_counter,
        }
    }
}