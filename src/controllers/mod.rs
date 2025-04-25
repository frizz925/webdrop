mod api;
mod main;
mod object;

use std::error::Error;

use axum::http::StatusCode;

use crate::services::{object::ObjectError, session::SessionError};

pub use main::MainController;

pub(super) const PUBLIC_PATH: &str = "web/build";

pub(super) trait StatusCodeError: Error {
    fn into_status_code(self) -> StatusCode;
}

impl StatusCodeError for ObjectError {
    fn into_status_code(self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl StatusCodeError for SessionError {
    fn into_status_code(self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
