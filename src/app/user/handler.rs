use rocket_contrib::json::{Json};
use crate::db;
use crate::current_user::CurrentUser;
use crate::output::{Output};
use super::output::{OPerson};
use super::input::{IUpdate};

#[get("/")]
pub fn get_me(_conn: db::Conn, _current_user: CurrentUser) -> Output<OPerson> {
    // let person = match db::person::find(&conn, current_user.id) {
    //     Ok(person) => person,
    //     Err(e) => return Output::new(e, Code::ResourceNotFound),
    // }
    Output::not_implemented()
}

#[get("/<id>")]
pub fn get(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> Output<OPerson> {
    println!("GET USER {}", id);
    Output::not_implemented()
}

#[put("/", format = "application/json", data="<input>")]
pub fn update_me(_conn: db::Conn, _current_user: CurrentUser, input: Json<IUpdate>) -> Output<OPerson> {
    println!("UPDATE ME {:?}", input);
    Output::not_implemented()
}

#[put("/<id>", format = "application/json", data="<input>")]
pub fn update(_conn: db::Conn, _current_user: CurrentUser, id: i32, input: Json<IUpdate>) -> Output<OPerson> {
    println!("UPDATE {} {:?} ", id, input);
    Output::not_implemented()
}

#[get("/notification")]
pub fn get_notification(_conn: db::Conn, _current_user: CurrentUser) -> Output<String> {
    println!("List notification");
    Output::not_implemented()
}

#[delete("/notification")]
pub fn read_notification(_conn: db::Conn, _current_user: CurrentUser) -> Output<String> {
    println!("READ NOTIFICATION");
    Output::not_implemented()
}
