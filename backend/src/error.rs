use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("status: {status}\nmessage: {message}\nerror: {error}\n")]
pub struct Error {
    status: StatusCode,
    message: &'static str,
    error: anyhow::Error,
}

impl Error {
    fn new(status: StatusCode, message: &'static str, error: impl Into<anyhow::Error>) -> Error {
        Error {
            status,
            message,
            error: error.into(),
        }
    }

    pub fn bad_request(message: &'static str, error: impl Into<anyhow::Error>) -> Error {
        Error::new(StatusCode::BAD_REQUEST, message, error)
    }

    pub fn internal_server_error(message: &'static str, error: impl Into<anyhow::Error>) -> Error {
        Error::new(StatusCode::INTERNAL_SERVER_ERROR, message, error)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::internal_server_error("Error happened", error)
    }
}
