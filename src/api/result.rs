use failure::Fail;

#[derive(Fail, Debug)]
pub enum APIError {
    #[fail(display = "{}", _0)]
    ReqwestError(#[cause] reqwest::Error),
}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

pub type APIResult<T> = Result<T, APIError>;
