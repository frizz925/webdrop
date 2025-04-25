use std::{
    error::Error as StdError,
    fmt::Display,
    io::{Error as IoError, ErrorKind},
    result::Result as StdResult,
};

use crate::{
    models::session::{Session, SessionId},
    repositories::session::SessionRepository,
};

#[derive(Debug)]
pub enum Error {
    SessionNotFound,
    Other(Box<dyn StdError>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::SessionNotFound => "Session not found".to_owned(),
            Self::Other(e) => e.to_string(),
        };
        f.write_str(&s)
    }
}

impl StdError for Error {}

pub type Result<T> = StdResult<T, Error>;

pub struct SessionService<R> {
    repository: R,
}

impl<R: SessionRepository> SessionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self) -> Result<Session> {
        normalize_result(self.repository.create().await)
    }

    pub async fn get(&self, sid: &SessionId) -> Result<Session> {
        normalize_result(self.repository.get(sid).await)
    }

    pub async fn exists(&self, sid: &SessionId) -> Result<bool> {
        normalize_result(self.repository.exists(sid).await)
    }

    pub async fn delete(&self, sid: &SessionId) -> Result<()> {
        normalize_result(self.repository.delete(sid).await)
    }
}

fn normalize_result<T>(res: StdResult<T, Box<dyn StdError>>) -> Result<T> {
    res.map_err(normalize_error)
}

fn normalize_error(err: Box<dyn StdError>) -> Error {
    if let Some(e) = err.downcast_ref::<IoError>() {
        if e.kind() == ErrorKind::NotFound {
            return Error::SessionNotFound;
        }
    }
    Error::Other(err)
}
