use serde_derive::{Serialize, Deserialize};
use rocket::{self, Request, Outcome};
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use crate::service::jwt::{self, Payload};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: i32,
    pub active_company_id: Option<i32>,
}

impl CurrentUser {
    pub fn load(token_payload: Payload) -> CurrentUser {
        Self {
            id: token_payload.person_id,
            active_company_id: token_payload.active_company_id,
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
                    Ok(token) => return Outcome::Success(CurrentUser::load(token)),
                    Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
