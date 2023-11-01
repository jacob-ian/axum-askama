use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub enum Error {
    NotFound(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

//TODO: Use error page HTML template
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        return (self.get_status_code()).into_response();
    }
}
