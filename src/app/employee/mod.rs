use rocket_contrib::json::{Json};
use crate::db;
use crate::current_user::CurrentUser;
use crate::cr::{CR};

pub mod input;

#[post("/invite", format = "application/json", data="<input>")]
pub fn invite(_conn: db::Conn, _current_user: CurrentUser, input: Json<input::Invite>) -> CR<String> {
    println!("INVITE {:?}", input);
    CR::not_implemented()
}

#[delete("/<id>")]
pub fn disable(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("DELETE EMPLOYEE {}", id);
    CR::not_implemented()
}