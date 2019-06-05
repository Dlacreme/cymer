use std::io::Cursor;
use serde_derive::{Serialize, Deserialize};
use rocket::request::Request;
use rocket::response::{Response, Responder};
use rocket::http::{ContentType, Status};
use diesel::QueryResult;
use crate::msg;

#[derive(Debug, Serialize, Deserialize)]
pub enum Code {
    Success,
    ResourceCreated,
    ServerError,
    DatabaseError,
    Unauthorized,
    InvalidPassword,
    InvalidInput,
    ResourceNotFound,
    ResourceAlreadyExisting,
    NotImplemented,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CR<T: serde::Serialize> {
    message: String,
    data: Option<T>,
    code: Code,
}

impl<T: serde::Serialize> CR<T> {

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

    pub fn data_code<D>(message: D, data: T, code: Code) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: Some(data),
            code: code,
        }
    }

    pub fn from_query_result<D>(data: QueryResult<T>) -> Self
    where D: std::fmt::Display {
        match data {
            Ok(d) => Self {
                message: format!("{}", msg::OK),
                data: Some(d),
                code: Code::Success
            },
            Err(e) => Self {
                message: format!("{}", e.to_string()),
                data: None,
                code: Code::ResourceNotFound,
            }
        }
    }

    pub fn not_implemented() -> Self {
        Self {
            message: format!("{}", msg::NOT_IMPLEMENTED),
            data: None,
            code: Code::NotImplemented
        }
    }

}

impl<T: serde::Serialize> Responder<'static> for CR<T> {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        match serde_json::to_string(&self) {
            Ok(json_str) => Response::build()
                .header(ContentType::new("application", "json"))
                .status(match self.code {
                    Code::Success => Status::Ok,
                    Code::ResourceCreated => Status::Created,
                    Code::ServerError => Status::InternalServerError,
                    Code::DatabaseError => Status::InternalServerError,
                    Code::InvalidInput => Status::BadRequest,
                    Code::ResourceNotFound => Status::NotFound,
                    Code::ResourceAlreadyExisting => Status::Forbidden,
                    Code::Unauthorized => Status::Unauthorized,
                    Code::InvalidPassword => Status::Unauthorized,
                    Code::NotImplemented => Status::NotImplemented,
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
