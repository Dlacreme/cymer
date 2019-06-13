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
pub fn get_active_employees(conn: db::Conn, current_user: CurrentUser) -> CR<Vec<Employee>> {
    if current_user.active_company_id.is_none() {
        return CR::new(crate::msg::NO_ACTIVE_COMPANY, Code::ResourceNotFound);
    }
    get_company_employees(&conn, current_user.active_company_id.unwrap())
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
pub fn set_active(conn: db::Conn, current_user: CurrentUser, id: i32) -> CR<Company> {
    match db::person::update_active_company(&conn, current_user.id, id) {
        Ok(_) => get_company(&conn, id),
        Err(e) => CR::new(e, Code::ServerError),
    }
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
    conn: db::Conn,
    current_user: CurrentUser,
    input: Json<CompanyToUpdate>,
) -> CR<Company> {
    if current_user.active_company_id.is_none() {
        return CR::new(crate::msg::NO_ACTIVE_COMPANY, Code::ResourceNotFound);
    }
    update_company(
        &conn,
        current_user.active_company_id.unwrap(),
        input.into_inner(),
    )
}

#[put("/<id>", format = "application/json", data = "<input>")]
pub fn update(
    conn: db::Conn,
    _current_user: CurrentUser,
    id: i32,
    input: Json<CompanyToUpdate>,
) -> CR<Company> {
    update_company(&conn, id, input.into_inner())
}

#[delete("/<id>")]
pub fn disable(_conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    println!("DELETE {}", id);
    CR::not_implemented()
}

pub fn get_company(conn: &db::Conn, company_id: i32) -> CR<Company> {
    CR::data_query(Company::from_db(&conn, company_id))
}

pub fn update_company(conn: &db::Conn, company_id: i32, input: CompanyToUpdate) -> CR<Company> {
    // CR::data_query(Company::from_db(&conn, company_id))
    match db::company::update(conn, company_id, input) {
        Ok(co) => CR::data(Company::from_model(&co)),
        Err(e) => CR::new(e, Code::ServerError),
    }
}

pub fn get_company_employees(conn: &db::Conn, company_id: i32) -> CR<Vec<Employee>> {
    CR::data_query(db::employee::get_from_company(conn, company_id))
}