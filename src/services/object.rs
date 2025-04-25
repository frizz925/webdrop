use std::{
    error::Error as StdError,
    fmt::Display,
    io::{Error as IoError, ErrorKind},
    result::Result as StdResult,
};

use tokio::io::AsyncRead;

use crate::{
    models::object::{Object, ObjectId, Upload},
    repositories::object::ObjectRepository,
};

#[derive(Debug)]
pub enum ObjectError {
    NotFound,
    Other(Box<dyn StdError>),
}

impl Display for ObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotFound => "Object not found".to_owned(),
            Self::Other(e) => e.to_string(),
        };
        f.write_str(&s)
    }
}

impl StdError for ObjectError {}

pub type Result<T> = StdResult<T, ObjectError>;

pub struct ObjectService<R> {
    repository: R,
}

impl<S: ObjectRepository> ObjectService<S> {
    pub fn new(repository: S) -> Self {
        Self { repository }
    }

    pub async fn put(&self, upload: Upload) -> Result<Object> {
        normalize_result(self.repository.put(upload).await)
    }

    pub async fn upload<R>(&self, upload: Upload, reader: R) -> Result<Object>
    where
        R: AsyncRead + Unpin + Send + Sync,
    {
        normalize_result(self.repository.upload(upload, reader).await)
    }

    pub async fn get(&self, oid: &ObjectId) -> Result<Object> {
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

fn normalize_error(err: Box<dyn StdError>) -> ObjectError {
    if let Some(e) = err.downcast_ref::<IoError>() {
        if e.kind() == ErrorKind::NotFound {
            return ObjectError::NotFound;
        }
    }
    ObjectError::Other(err)
}
