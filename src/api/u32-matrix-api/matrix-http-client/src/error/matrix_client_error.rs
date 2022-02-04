use crate::abstraction::GetError;
use crate::error::HttpResponseError;
use actix_web::client::{PayloadError, SendRequestError};
use std::error::Error;
use std::fmt::{Display, Formatter};

// TODO: Reduce the number of variants in this type as it is growing.
#[derive(Debug)]
pub enum MatrixClientError {
    External(Box<dyn Error>),
    SendRequestError(SendRequestError),
    PayloadErr(PayloadError),
    JsonDeserializationError(serde_json::Error),
    HttpResponseError(HttpResponseError),
    ContentTypeMissingError,
    ContentTypeInvalidError(String),
    Unknown,
}

impl Display for MatrixClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatrixClientError")
    }
}

impl Error for MatrixClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MatrixClientError::SendRequestError(e) => Some(e),
            MatrixClientError::PayloadErr(e) => Some(e),
            MatrixClientError::JsonDeserializationError(e) => Some(e),
            MatrixClientError::HttpResponseError(e) => Some(e),
            _ => None,
        }
    }
}

impl GetError<HttpResponseError> for MatrixClientError {
    fn get_error(&self) -> Option<&HttpResponseError> {
        match self {
            MatrixClientError::HttpResponseError(e) => Some(&e),
            _ => None,
        }
    }
}
