use super::input::{Login, Signup};
use rocket_contrib::json::{Json};
use crate::output::Output;

#[post("/login", format = "application/json", data="<login>")]
pub fn login(login: Json<Login>) -> Output<String> {
    println!("{:?}", login);

    Output::message("Hello World")
}

#[post("/signup", format = "application/json", data="<signup>")]
pub fn signup(signup: Json<Signup>) -> &'static str {
    println!("{:?}", signup);
    "Sign up"
}
