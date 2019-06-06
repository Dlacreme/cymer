use rocket_contrib::json::{Json};
use crate::cr::{self, CR};
use crate::db;
use crate::msg;
use crate::model::person::Person;
use crate::service::jwt;
use crate::current_user::CurrentUser;

pub mod input;
pub mod output;

#[post("/login", format = "application/json", data="<login>")]
pub fn login(conn: db::Conn, login: Json<input::Login>) -> CR<output::Login> {
    match login.validate() {
        Ok(_) => (),
        Err(s) => return CR::new(s, cr::Code::InvalidInput)
    };
    let person = match db::person::get_by_email(&conn, login.email.as_str()) {
        Ok(person) => {
            match bcrypt::verify(&login.password, person.password.as_str()) {
                Ok(is_valid) => {
                    if is_valid == false {
                        return CR::new(msg::LOGIN_FAILED, cr::Code::InvalidPassword);
                    }
                    person
                },
                Err(_) => return CR::new(msg::LOGIN_FAILED, cr::Code::InvalidPassword),
            }
        },
        Err(e) => return CR::new(e, cr::Code::ResourceNotFound),
    };
    login_person(conn, person)
}

#[post("/signup", format = "application/json", data="<signup>")]
pub fn signup(conn: db::Conn, signup: Json<input::Signup>) -> CR<output::Login> {
    match signup.validate() {
        Ok(_) => (),
        Err(s) => return CR::new(s, cr::Code::InvalidInput)
    };
    match db::person::get_by_email(&conn, signup.email.as_str()) {
        Ok(_) => return CR::new(msg::USER_EXISTING, cr::Code::ResourceAlreadyExisting),
        Err(_) => {
            match db::person::create(&conn, signup.email.as_str(), signup.password.as_str()) {
                Ok(person) => return login_person(conn, person),
                Err(e) => return CR::new(e, cr::Code::DatabaseError),
            }
        },
    }
}

fn login_person(_conn: db::Conn, mut person: Person) -> CR<output::Login> {
    let token = match jwt::serialize(jwt::Payload::from_person(&person)) {
        Ok(token) => token,
        Err(_) => {
            // Should add log
            return CR::new(msg::SERVER_ERROR, cr::Code::ServerError)
        }
    };
    person.password = String::from("PRIVATE");
    CR::data_code(msg::OK, output::Login::new(token, person), cr::Code::ResourceCreated)
}
