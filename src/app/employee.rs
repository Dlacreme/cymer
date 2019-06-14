use crate::cr::{CR, Code};
use crate::current_user::CurrentUser;
use crate::db;
use crate::view_model::employee::Invite;
use rocket_contrib::json::Json;

#[post("/invite", format = "application/json", data = "<input>")]
pub fn invite(_conn: db::Conn, _current_user: CurrentUser, input: Json<Invite>) -> CR<String> {
    println!("INVITE {:?}", input);
    CR::not_implemented()
}

#[delete("/<id>")]
pub fn disable(conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    match db::employee::disable(&conn, id) {
        Ok(_) => CR::new(crate::msg::RESOURCE_DISABLED, Code::Success),
        Err(_) => CR::new(crate::msg::ENTITY_NOT_FOUND, Code::ResourceNotFound),
    }
}
