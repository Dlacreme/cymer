use crate::cr::{Code, CR};
use crate::current_user::CurrentUser;
use crate::db;
use crate::model::employee_role::EmployeeRoleEnum;
use crate::view_model::employee::Invite;
use rocket_contrib::json::Json;

#[post("/invite", format = "application/json", data = "<input>")]
pub fn invite(conn: db::Conn, current_user: CurrentUser, input: Json<Invite>) -> CR<String> {
    if current_user.active_company_id.is_none() {
        return CR::new(crate::msg::NO_ACTIVE_COMPANY, Code::ResourceNotFound);
    }
    match input.validate() {
        Ok(_) => (),
        Err(s) => return CR::new(s, Code::InvalidInput),
    }
    let person = match db::person::create(&conn, input.email.as_str(), "") {
        Ok(ps) => ps,
        Err(e) => return CR::new(e, Code::InvalidInput),
    };
    match db::employee::create(
        &conn,
        person.id,
        current_user.active_company_id.unwrap(),
        EmployeeRoleEnum::Admin,
    ) {
        Ok(_) => (),
        Err(e) => return CR::new(e, Code::InvalidInput),
    };
    match crate::service::reset_password::invite(&conn, person.id) {
        Ok(_) => CR::new(crate::msg::OK, Code::Success),
        Err(_) => CR::new(crate::msg::SERVER_ERROR, Code::ServerError),
    }
}

#[delete("/<id>")]
pub fn disable(conn: db::Conn, _current_user: CurrentUser, id: i32) -> CR<String> {
    match db::employee::disable(&conn, id) {
        Ok(_) => CR::new(crate::msg::RESOURCE_DISABLED, Code::Success),
        Err(_) => CR::new(crate::msg::ENTITY_NOT_FOUND, Code::ResourceNotFound),
    }
}
