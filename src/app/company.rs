use rocket_contrib::json::{Json};
use crate::db;
use crate::current_user::CurrentUser;
use crate::cr::{CR};
use crate::view_model::company::{CompanyToUpdate};

#[get("/")]
pub fn get_active(_conn: db::Conn, _current_user: CurrentUser) -> CR<String> {
    CR::not_implemented()
}

#[get("/<id>")]
pub fn get(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("GET COMPANY {}", id);
    CR::not_implemented()
}

#[get("/employees")]
pub fn get_active_employees(_conn: db::Conn, _current_user: CurrentUser) -> CR<String> {
    CR::not_implemented()
}

#[get("/<id>/employees")]
pub fn get_employees(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("GET EMPLOYEES COMPANY {}", id);
    CR::not_implemented()
}

#[get("/list")]
pub fn get_available(_conn: db::Conn, _current_user: CurrentUser) -> CR<String> {
    CR::not_implemented()
}

#[post("/<id>/use")]
pub fn set_active(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("SET ACTIVE {}", id);
    CR::not_implemented()
}

#[post("/", format = "application/json", data="<input>")]
pub fn create(_conn: db::Conn, _current_user: CurrentUser, input: Json<CompanyToUpdate>) -> CR<String> {
    println!("CREATE {:?}", input);
    CR::not_implemented()
}

#[put("/", format = "application/json", data="<input>")]
pub fn update_active(_conn: db::Conn, _current_user: CurrentUser, input: Json<CompanyToUpdate>) -> CR<String> {
    println!("UPDATE ME {:?}", input);
    CR::not_implemented()
}

#[put("/<id>", format = "application/json", data="<input>")]
pub fn update(_conn: db::Conn, _current_user: CurrentUser, id: i32, input: Json<CompanyToUpdate>) -> CR<String> {
    println!("UPDATE {} {:?} ", id, input);
    CR::not_implemented()
}

#[delete("/<id>")]
pub fn disable(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("DELETE {}", id);
    CR::not_implemented()
}