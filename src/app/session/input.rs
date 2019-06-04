use serde_derive::{Serialize, Deserialize};
use crate::parser::validator;

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
