use super::input::{Login, Signup};
use rocket_contrib::json::{Json};
use crate::output::{self, Output};
use crate::db;

#[post("/login", format = "application/json", data="<login>")]
pub fn login(connection: db::Conn, login: Json<Login>) -> Output<String> {
    let person = match db::person::get_by_credentials(&connection, login.email.as_str(), login.get_password().as_str()) {
        Ok(person) => person,
        Err(_) => return Output::error(crate::msg::ENTITY_NOT_FOUND, output::Error::NotFound),
    };
    println!("Person found: {:?}", person);
    Output::message("Hello World")
}

#[post("/signup", format = "application/json", data="<signup>")]
pub fn signup(signup: Json<Signup>) -> Output<String> {
    println!("{:?}", signup);

    Output::message("Hello World")
}
