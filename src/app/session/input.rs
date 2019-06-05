use serde_derive::{Serialize, Deserialize};
use crate::parser::validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct ILogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ISignup {
    pub email: String,
    pub password: String,
}

impl ILogin {
    pub fn validate(&self) -> Result<(), &str> {
        validator::is_email(&self.email)?;

        Result::Ok(())
    }
}

impl ISignup {
    pub fn validate(&self) -> Result<(), &str> {
        validator::is_email(&self.email)?;
        validator::is_valid_password(&self.password)?;
        Result::Ok(())
    }
}
