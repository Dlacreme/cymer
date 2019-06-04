use std::io::Cursor;
use serde_derive::{Serialize, Deserialize};
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket::http::{ContentType, Status};

#[derive(Debug, Serialize, Deserialize)]
pub enum Code {
    Success,
    ServerError,
    DatabaseError,
    Unauthorized,
    InvalidInput,
    ResourceNotFound,
    ResourceAlreadyExisting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output<T: serde::Serialize> {
    message: String,
    data: Option<T>,
    code: Code,
}

impl<T: serde::Serialize> Output<T> {

    pub fn new<D>(message: D, code: Code) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            code: code,
        }
    }

    pub fn message<D>(message: D) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            code: Code::Success,
        }
    }

    pub fn data<D>(message: D, data: T) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: Some(data),
            code: Code::Success,
        }
    }

}

impl<T: serde::Serialize> Responder<'static> for Output<T> {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        match serde_json::to_string(&self) {
            Ok(json_str) => Response::build()
                .header(ContentType::new("application", "json"))
                .status(match self.code {
                    Code::Success => Status::Ok,
                    Code::ServerError => Status::InternalServerError,
                    Code::DatabaseError => Status::InternalServerError,
                    Code::InvalidInput => Status::BadRequest,
                    Code::ResourceNotFound => Status::NotFound,
                    Code::ResourceAlreadyExisting => Status::Forbidden,
                    Code::Unauthorized => Status::Unauthorized,
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
