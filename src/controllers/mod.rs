mod api;
mod main;
mod object;
mod websocket;

use std::error::Error;

use axum::http::{HeaderMap, StatusCode};
use base64::{
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD},
    Engine,
};
use serde::Deserialize;

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

pub trait AuthKeyExtractor {
    fn extract_auth_key(&self) -> Result<Vec<u8>, StatusCode>;
}

impl AuthKeyExtractor for HeaderMap {
    fn extract_auth_key(&self) -> Result<Vec<u8>, StatusCode> {
        if let Some(encoded) = self.get("X-Auth-Key") {
            BASE64_STANDARD
                .decode(encoded)
                .map_err(|_| StatusCode::BAD_REQUEST)
        } else {
            Ok(Vec::default())
        }
    }
}

#[derive(Deserialize)]
pub struct AuthParams {
    auth: Option<String>,
}

impl AuthKeyExtractor for AuthParams {
    fn extract_auth_key(&self) -> Result<Vec<u8>, StatusCode> {
        if let Some(encoded) = self.auth.as_deref() {
            BASE64_URL_SAFE_NO_PAD
                .decode(encoded)
                .map_err(|_| StatusCode::BAD_REQUEST)
        } else {
            Ok(Vec::default())
        }
    }
}
