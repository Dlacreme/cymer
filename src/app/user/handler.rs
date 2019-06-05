use crate::db;
use crate::current_user::CurrentUser;
use crate::output::{Output};
use super::output::{OPerson};

#[get("/user")]
pub fn get_me(_conn: db::Conn, _current_user: CurrentUser) -> Output<OPerson> {
    // let person = match db::person::find(&conn, current_user.id) {
    //     Ok(person) => person,
    //     Err(e) => return Output::new(e, Code::ResourceNotFound),
    // }
    Output::not_implemented()
}

#[get("/user/<id>")]
pub fn get(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> Output<OPerson> {
    println!("GET USER {}", id);
    Output::not_implemented()
}
