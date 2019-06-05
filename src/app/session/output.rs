use serde_derive::{Serialize, Deserialize};
use crate::model::person;

#[derive(Debug, Serialize, Deserialize)]
pub struct OLogin {
    token: String,
    user: person::Person,
}

impl OLogin {
    pub fn new(token: String, person: person::Person) -> Self {
        Self {
            token: token,
            user: person,
        }
    }
}