use serde_derive::{Serialize, Deserialize};
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signup {
    pub email: String,
    password: String,
}

impl Login {

    pub fn get_password(&self) -> String {
        get_password_hash(&self.password)
    }

}

impl Signup {

    pub fn get_password(&self) -> String {
        get_password_hash(&self.password)
    }

}

fn get_password_hash(pass: &String) -> String {
    hash(pass, DEFAULT_COST).unwrap()
}
