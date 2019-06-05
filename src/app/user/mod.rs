use rocket_contrib::json::{Json};
use crate::db;
use crate::current_user::CurrentUser;
use crate::cr::{CR};

pub mod output;
pub mod input;

#[get("/")]
pub fn get_me(_conn: db::Conn, _current_user: CurrentUser) -> CR<output::OPerson> {
    // let person = match db::person::find(&conn, current_user.id) {
    //     Ok(person) => person,
    //     Err(e) => return Output::new(e, Code::ResourceNotFound),
    // }
    CR::not_implemented()
}

#[get("/<id>")]
pub fn get(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<output::OPerson> {
    println!("GET USER {}", id);
    CR::not_implemented()
}

#[put("/", format = "application/json", data="<input>")]
pub fn update_me(_conn: db::Conn, _current_user: CurrentUser, input: Json<input::IUpdate>) -> CR<output::OPerson> {
    println!("UPDATE ME {:?}", input);
    CR::not_implemented()
}

#[put("/<id>", format = "application/json", data="<input>")]
pub fn update(_conn: db::Conn, _current_user: CurrentUser, id: i32, input: Json<input::IUpdate>) -> CR<output::OPerson> {
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
