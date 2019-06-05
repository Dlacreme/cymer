use serde_derive::{Serialize, Deserialize};
use rocket::{self, Request, Outcome};
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use crate::service::jwt;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    id: i32,
}

impl CurrentUser {
    pub fn load(id: i32) -> CurrentUser {
        Self {
            id,
        }
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();
    fn from_request(rq: &'a Request<'r>) -> request::Outcome<CurrentUser, Self::Error> {
        match rq.headers().get_one("Authorization") {
            Some(token) => {
                let token_items: Vec<&str> = token.trim().split_whitespace().collect::<Vec<&str>>();
                if token_items.len() != 2 {
                    return Outcome::Failure((Status::Unauthorized, ()));
                }
                match jwt::deserialize(String::from(token_items[1])) {
                    Ok(token) => Outcome::Success(CurrentUser::load(token.person_id)),
                    Err(_) => Outcome::Failure((Status::BadRequest, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
