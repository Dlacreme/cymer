use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signup {
    email: String,
    password: String,
}
