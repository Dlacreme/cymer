use rocket;

mod session;
mod user;

pub fn routes(rocket: rocket::Rocket) -> rocket::Rocket {
    // Public routes only
    rocket.mount("/", routes![
        // Session
        session::me,
        session::login,
        session::signup,
    ]).mount("/user", routes![
        user::get_me,
        user::get,
        user::update_me,
        user::update,
        user::get_notification,
        user::read_notification,
    ])
}
