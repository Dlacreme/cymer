use rocket_contrib::json::{Json};
use crate::db;
use crate::current_user::CurrentUser;
use crate::cr::{CR, Code};
use crate::view_model::user::User;

pub mod input;

#[get("/")]
pub fn get_me(conn: db::Conn, current_user: CurrentUser) -> CR<User> {
    CR::data_query(User::from_db(&conn, current_user.id))
}

#[get("/<id>")]
pub fn get(conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<User> {
    CR::data_query(User::from_db(&conn, id))
}

#[put("/", format = "application/json", data="<input>")]
pub fn update_me(_conn: db::Conn, _current_user: CurrentUser, input: Json<input::Update>) -> CR<String> {
    println!("UPDATE ME {:?}", input);
    CR::not_implemented()
}

#[put("/<id>", format = "application/json", data="<input>")]
pub fn update(_conn: db::Conn, _current_user: CurrentUser, id: i32, input: Json<input::Update>) -> CR<String> {
    println!("UPDATE {} {:?} ", id, input);
    CR::not_implemented()
}

#[get("/notification")]
pub fn get_notification(_conn: db::Conn, _current_user: CurrentUser) -> CR<String> {
    println!("List notification");
    CR::not_implemented()
}

#[delete("/notification")]
pub fn read_notification(_conn: db::Conn, _current_user: CurrentUser) -> CR<String> {
    println!("READ NOTIFICATION");
    CR::not_implemented()
}
