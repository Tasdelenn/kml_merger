// error.rs
// Custom error types for the KML Merger application

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    XmlError(quick_xml::Error),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "I/O error: {}", err),
            AppError::XmlError(err) => write!(f, "XML parsing error: {}", err),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<quick_xml::Error> for AppError {
    fn from(err: quick_xml::Error) -> Self {
        AppError::XmlError(err)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
