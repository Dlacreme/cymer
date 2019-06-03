use rocket;

mod session;

pub fn routes(rocket: rocket::Rocket) -> rocket::Rocket {
    // Public routes only
    rocket.mount("/", routes![
        // Session
        session::handler::login,
        session::handler::signup,
    ])
}
