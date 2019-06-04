use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::ops::Deref;

pub mod person;

// Extracted from Diesel doc - https://rocket.rs/v0.4/guide/state/#databases
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
impl Deref for Conn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}