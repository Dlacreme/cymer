use crate::cr::{Code, CR};
use crate::current_user::CurrentUser;
use crate::db;
use crate::view_model::company::{Company, CompanyToCreate, CompanyToUpdate};
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_active(conn: db::Conn, current_user: CurrentUser) -> CR<Company> {
    if current_user.active_company_id.is_none() {
        return CR::new(crate::msg::NO_ACTIVE_COMPANY, Code::ResourceNotFound);
    }
    get_company(&conn, current_user.active_company_id.unwrap())
}

#[get("/<id>")]
pub fn get(conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<Company> {
    get_company(&conn, id)
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

#[post("/", format = "application/json", data = "<input>")]
pub fn create(
    conn: db::Conn,
    current_user: CurrentUser,
    input: Json<CompanyToCreate>,
) -> CR<Company> {
    let data = input.into_inner();
    match db::company::create(&conn, data.label.as_str(), current_user.id) {
        Ok(co) => CR::data(Company::from_model(&co)),
        Err(e) => CR::new(e, Code::ServerError),
    }
}

#[put("/", format = "application/json", data = "<input>")]
pub fn update_active(
    _conn: db::Conn,
    _current_user: CurrentUser,
    input: Json<CompanyToUpdate>,
) -> CR<String> {
    println!("UPDATE ME {:?}", input);
    CR::not_implemented()
}

#[put("/<id>", format = "application/json", data = "<input>")]
pub fn update(
    _conn: db::Conn,
    _current_user: CurrentUser,
    id: i32,
    input: Json<CompanyToUpdate>,
) -> CR<String> {
    println!("UPDATE {} {:?} ", id, input);
    CR::not_implemented()
}

#[delete("/<id>")]
pub fn disable(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("DELETE {}", id);
    CR::not_implemented()
}

pub fn get_company(conn: &db::Conn, company_id: i32) -> CR<Company> {
    CR::data_query(Company::from_db(&conn, company_id))
}
