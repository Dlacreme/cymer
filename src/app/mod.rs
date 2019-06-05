use rocket;

mod session;
mod user;

pub fn routes(rocket: rocket::Rocket) -> rocket::Rocket {
    // Public routes only
    rocket.mount("/", routes![
        // Session
        session::handler::me,
        session::handler::login,
        session::handler::signup,
    ]).mount("/user", routes![
        user::handler::get_me,
        user::handler::get,
        user::handler::update_me,
        user::handler::update,
        user::handler::get_notification,
        user::handler::read_notification,
    ])
}
