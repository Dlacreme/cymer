use serde_derive::{Serialize, Deserialize};
use crate::parser::validator;
use crate::model::person;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signup {
    pub email: String,
    pub password: String,
}

impl Login {
    pub fn validate(&self) -> Result<(), &str> {
        validator::is_email(&self.email)?;

        Result::Ok(())
    }
}

impl Signup {
    pub fn validate(&self) -> Result<(), &str> {
        validator::is_email(&self.email)?;
        validator::is_valid_password(&self.password)?;
        Result::Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    user: person::Person,
}

impl LoginResult {
    pub fn new(token: String, person: person::Person) -> Self {
        Self {
            token: token,
            user: person,
        }
    }
}