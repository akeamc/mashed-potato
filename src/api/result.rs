use actix_web::{http::StatusCode, ResponseError};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum APIError {
    #[fail(display = "{}", _0)]
    ReqwestError(#[cause] reqwest::Error),

    #[fail(display = "{}", _0)]
    NotFound(String),
}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ReqwestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

pub type APIResult<T> = Result<T, APIError>;
