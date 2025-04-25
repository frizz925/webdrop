use std::{
    error::Error as StdError,
    fmt::Display,
    io::{Error as IoError, ErrorKind},
    result::Result as StdResult,
};

use tokio::io::AsyncRead;

use crate::{
    models::object::{ObjectId, ObjectResult, Upload},
    repositories::object::ObjectRepository,
};

#[derive(Debug)]
pub enum Error {
    ObjectNotFound,
    Other(Box<dyn StdError>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ObjectNotFound => "Object not found".to_owned(),
            Self::Other(e) => e.to_string(),
        };
        f.write_str(&s)
    }
}

impl StdError for Error {}

pub type Result<T> = StdResult<T, Error>;

pub struct ObjectService<R> {
    repository: R,
}

impl<R: ObjectRepository> ObjectService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn put(&self, upload: Upload) -> Result<ObjectResult> {
        normalize_result(self.repository.put(upload).await)
    }

    pub async fn upload(&self, upload: Upload, reader: R) -> Result<ObjectResult>
    where
        R: AsyncRead + Unpin + Send + Sync + 'static,
    {
        normalize_result(self.repository.upload(upload, reader).await)
    }

    pub async fn get(&self, oid: &ObjectId) -> Result<ObjectResult> {
        normalize_result(self.repository.get(oid).await)
    }

    pub async fn download(
        &self,
        oid: &ObjectId,
        name: &str,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>> {
        normalize_result(self.repository.download(oid, name).await)
    }

    pub async fn delete(&self, oid: &ObjectId) -> Result<()> {
        normalize_result(self.repository.delete(oid).await)
    }
}

fn normalize_result<T>(res: StdResult<T, Box<dyn StdError>>) -> Result<T> {
    res.map_err(normalize_error)
}

fn normalize_error(err: Box<dyn StdError>) -> Error {
    if let Some(e) = err.downcast_ref::<IoError>() {
        if e.kind() == ErrorKind::NotFound {
            return Error::ObjectNotFound;
        }
    }
    Error::Other(err)
}
