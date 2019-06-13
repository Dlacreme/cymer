use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    person_id: i32,
    employee_id: i32,}
    