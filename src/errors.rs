use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub enum Error {
    NotFound(String),
    InternalServerError(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(self) -> String {
        match self {
            Error::NotFound(m) => m,
            Error::InternalServerError(m) => m,
        }
    }
}

//TODO: Use error page HTML template
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        return (self.get_status_code(), self.get_message()).into_response();
    }
}

impl From<askama::Error> for Error {
    fn from(value: askama::Error) -> Self {
        return Error::InternalServerError(value.to_string());
    }
}
