use super::input::{Login, Signup};
use rocket_contrib::json::{Json};
use crate::output::{self, Output};
use crate::db;
use crate::msg;
use crate::code;

#[post("/login", format = "application/json", data="<login>")]
pub fn login(conn: db::Conn, login: Json<Login>) -> Output<String> {
    match login.validate() {
        Ok(_) => (),
        Err(s) => return Output::error(s, output::Error::InvalidQuery(code::INVALID_INPUT))
    };
    let person = match db::person::get_by_credentials(&conn, login.email.as_str(), login.password.as_str()) {
        Ok(person) => person,
        Err(e) => return Output::error(e.to_string(), output::Error::NotFound(code::RESOURCE_NOT_FOUND)),
    };
    println!("Person found: {:?}", person);
    Output::message("Hello World")
}

#[post("/signup", format = "application/json", data="<signup>")]
pub fn signup(conn: db::Conn, signup: Json<Signup>) -> Output<String> {
    match signup.validate() {
        Ok(_) => (),
        Err(s) => return Output::error(s, output::Error::InvalidQuery(code::INVALID_INPUT))
    };
    match db::person::get_by_email(&conn, signup.email.as_str()) {
        Ok(_) => return Output::error(msg::USER_EXISTING, output::Error::InvalidQuery(code::RESOURCE_ALREADY_EXISTING)),
        Err(_) => {
            match db::person::create(&conn, signup.email.as_str(), signup.password.as_str()) {
                Ok(person) =>  return Output::message("PERSON CREATED"),
                Err(e) => return Output::error(e.to_string(), output::Error::ServerError(code::DATABASE_ERROR)),
            }
        },
    }
}
