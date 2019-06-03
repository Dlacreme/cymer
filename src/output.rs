use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    ServerError,
    InvalidQuery,
    NotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output<T> {
    message: String,
    data: Option<T>,
    error: Option<Error>,
}

impl<T> Output<T> {

    pub fn message<D>(message: D) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            error: None,
        }
    }

    pub fn data<D>(message: D, data: Option<T>) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: data,
            error: None,
        }
    }

    pub fn error<D>(message: D, error: Option<Error>) -> Self
    where D: std::fmt::Display {
        Self {
            message: format!("{}", message),
            data: None,
            error: error,
        }
    }

}
