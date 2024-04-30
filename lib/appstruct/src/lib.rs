use hyper::http::StatusCode;
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum WebError {
    #[error("I/O error: {0} {1:?}")]
    Io(String, io::ErrorKind),

    #[error("HTTP error: {0}")]
    Http(#[from] hyper::Error),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Invalid header value")]
    HeaderValue(#[from] hyper::header::InvalidHeaderValue),

    #[error("UTF8 Error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Error not found")]
    NotFound,

    #[error("Unknown error")]
    Unknown,

    #[error("Authorization Error: {0}")]
    AuthorizationError(String),

    #[error("Some String Error: {0}")]
    StringError(String),

    #[error("I'm a teapot")]
    Teapot,

    #[error("Error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{message}\nCaused By: {caused_by}")]
    AnyhowError { message: String, caused_by: String },

    #[error("Redis Error: {0}")]
    RedisError(#[from] redis::RedisError),
}

impl From<io::Error> for WebError {
    fn from(err: io::Error) -> WebError {
        WebError::Io(err.to_string(), err.kind())
    }
}

impl From<anyhow::Error> for WebError {
    fn from(err: anyhow::Error) -> WebError {
        let caused_by = match err.source() {
            Some(source) => source.to_string(),
            None => String::from("Unknown"),
        };

        WebError::AnyhowError {
            message: err.to_string(),
            caused_by,
        }
    }
}

impl WebError {
    pub fn status(&self) -> StatusCode {
        match *self {
            WebError::Io(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::Http(_) => StatusCode::BAD_REQUEST,
            WebError::ParseIntError(_) => StatusCode::BAD_REQUEST,
            WebError::HeaderValue(_) => StatusCode::BAD_REQUEST,
            WebError::Utf8Error(_) => StatusCode::BAD_REQUEST,
            WebError::NotFound => StatusCode::NOT_FOUND,
            WebError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::AuthorizationError(_) => StatusCode::UNAUTHORIZED,
            WebError::StringError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::Teapot => StatusCode::IM_A_TEAPOT,
            WebError::JsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::AnyhowError {
                message: _,
                caused_by: _,
            } => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<String> for WebError {
    fn from(s: String) -> Self {
        WebError::StringError(s)
    }
}

impl axum::response::IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let body = maud::html! {
            h1 { "Error" }
            pre { (self) }
        };

        let code = self.status();

        // its often easiest to implement `IntoResponse` by calling other implementations
        (code, body).into_response()
    }
}
