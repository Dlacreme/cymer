use crate::cr::{Code, CR};
use crate::current_user::{CurrentAdmin, CurrentUser};
use crate::db;
use crate::model::person::Person;
use crate::view_model::user::{User, UserToUpdate};
use diesel::QueryResult;
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_me(conn: db::Conn, current_user: CurrentUser) -> CR<User> {
    get_user(&conn, current_user.id)
}

#[get("/<id>")]
pub fn get(conn: db::Conn, _current_admin: CurrentAdmin, id: i32) -> CR<User> {
    get_user(&conn, id)
}

#[put("/", format = "application/json", data = "<input>")]
pub fn update_me(conn: db::Conn, current_user: CurrentUser, input: Json<UserToUpdate>) -> CR<User> {
    match update_user(&conn, current_user.id, input.into_inner()) {
        Ok(up) => up,
        Err(e) => return CR::new(e, Code::ResourceNotFound),
    };
    get_user(&conn, current_user.id)
}

#[put("/<id>", format = "application/json", data = "<input>")]
pub fn update(
    conn: db::Conn,
    _current_user: CurrentAdmin,
    id: i32,
    input: Json<UserToUpdate>,
) -> CR<User> {
    match update_user(&conn, id, input.into_inner()) {
        Ok(up) => up,
        Err(e) => return CR::new(e, Code::ResourceNotFound),
    };
    get_user(&conn, id)
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

fn get_user(conn: &db::Conn, person_id: i32) -> CR<User> {
    CR::data_query(User::from_db(&conn, person_id))
}

fn update_user(conn: &db::Conn, person_id: i32, input: UserToUpdate) -> QueryResult<Person> {
    let person = match input.role.is_some() {
        true => db::person::update_role(conn, person_id, input.role.unwrap().clone())?,
        false => db::person::find(conn, person_id)?,
    };
    db::person_profile::update(conn, person.person_profile_id, input)?;
    QueryResult::Ok(person)
}
