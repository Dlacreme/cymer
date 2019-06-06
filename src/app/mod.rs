use rocket;

mod session;
mod user;
mod company;
mod employee;

pub fn routes(rocket: rocket::Rocket) -> rocket::Rocket {
    // Public routes only
    rocket.mount("/", routes![
        // Session
        session::login,
        session::signup,
    ]).mount("/user", routes![
        user::get_me,
        user::get,
        user::update_me,
        user::update,
        user::get_notification,
        user::read_notification,
    ]).mount("/company", routes![
        company::get_active,
        company::get,
        company::get_active_employees,
        company::get_employees,
        company::get_available,
        company::set_active,
        company::create,
        company::update_active,
        company::update,
        company::disable,
    ]).mount("/employee", routes![
        employee::invite,
        employee::disable,
    ])
}
