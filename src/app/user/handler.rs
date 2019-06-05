use crate::db;
use crate::current_user::CurrentUser;
use crate::output::{Output, Code};
use super::output::{OPerson};

#[get("/user")]
pub fn get_me(conn: db::Conn, current_user: CurrentUser) -> Output<OPerson> {
    let person = match db::person::find(&conn, current_user.id) {
        Ok(person) => person,
        Err(e) => return Output::new(e, Code::ResourceNotFound),
    }
}

#[get("/user/<id>")]
pub fn get(conn: db::Conn, current_user: CurrentUser, id: i32) -> Output<OPerson> {

}
