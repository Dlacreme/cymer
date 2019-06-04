use super::input::{Login, Signup};
use rocket_contrib::json::{Json};
use crate::output::{self, Output};
use crate::db;
use crate::msg;
use crate::model::person::Person;
use crate::service::jwt;

#[post("/login", format = "application/json", data="<login>")]
pub fn login(conn: db::Conn, login: Json<Login>) -> Output<super::output::Login> {
    match login.validate() {
        Ok(_) => (),
        Err(s) => return Output::new(s, output::Code::InvalidInput)
    };
    let person = match db::person::get_by_credentials(&conn, login.email.as_str(), login.password.as_str()) {
        Ok(person) => person,
        Err(e) => return Output::new(e.to_string(), output::Code::ResourceNotFound),
    };
    login_person(conn, person)
}

#[post("/signup", format = "application/json", data="<signup>")]
pub fn signup(conn: db::Conn, signup: Json<Signup>) -> Output<super::output::Login> {
    match signup.validate() {
        Ok(_) => (),
        Err(s) => return Output::new(s, output::Code::InvalidInput)
    };
    match db::person::get_by_email(&conn, signup.email.as_str()) {
        Ok(_) => return Output::new(msg::USER_EXISTING, output::Code::ResourceAlreadyExisting),
        Err(_) => {
            match db::person::create(&conn, signup.email.as_str(), signup.password.as_str()) {
                Ok(person) => return login_person(conn, person),
                Err(e) => return Output::new(e.to_string(), output::Code::DatabaseError),
            }
        },
    }
}

fn login_person(code: db::Conn, person: Person) -> Output<super::output::Login> {
    let token = match jwt::serialize(jwt::Payload::from_person(&person)) {
        Ok(token) => token,
        Err(_) => {
            // Should add log
            return Output::new(msg::SERVER_ERROR, output::Code::ServerError)
        }
    };
    Output::data_code(msg::OK, super::output::Login::new(token, person), output::Code::ResourceCreated)
}
