use std::io::Cursor;
use serde_derive::{Serialize, Deserialize};
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket::http::{ContentType, Status};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    ServerError(String),
    InvalidQuery(String),
    NotFound(String),
    Unauthorized(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output<T: serde::Serialize> {
    message: String,
    data: Option<T>,
    error: Option<Error>,
}

impl<T: serde::Serialize> Output<T> {

    pub fn message<D>(message: D) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            error: None,
        }
    }

    pub fn data<D>(message: D, data: T) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: Some(data),
            error: None,
        }
    }

    pub fn error<D>(message: D, error: Error) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            error: Some(error),
        }
    }

}

impl<T: serde::Serialize> Responder<'static> for Output<T> {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        match serde_json::to_string(&self) {
            Ok(json_str) => Response::build()
                .header(ContentType::new("application", "json"))
                .status(match self.error {
                    None => Status::Ok,
                    Some(v) => match v {
                        Error::ServerError(_) => Status::InternalServerError,
                        Error::InvalidQuery(_) => Status::BadRequest,
                        Error::NotFound(_) => Status::NotFound,
                        Error::Unauthorized(_) => Status::Unauthorized,
                    }
                })
                .sized_body(Cursor::new(json_str))
                .ok(),
            Err(_) => Response::build()
                .header(ContentType::Plain)
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(String::from("Server error -- could not build output object")))
                .ok(),
        }
    }
}
