use serde_derive::{Serialize, Deserialize};

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
    pub fn validate(&self) -> Option<String> {
        None
    }
}

impl Signup {
    pub fn validate(&self) -> Option<String> {
        None
    }
}
