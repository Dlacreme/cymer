use rocket;

pub mod current_user;
mod session;

pub fn routes(rocket: rocket::Rocket) -> rocket::Rocket {
    // Public routes only
    rocket.mount("/", routes![
        // Session
        session::handler::me,
        session::handler::login,
        session::handler::signup,
    ])
}
