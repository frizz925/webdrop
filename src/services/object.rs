use std::{
    error::Error as StdError,
    fmt::Display,
    io::{Error as IoError, ErrorKind},
    result::Result as StdResult,
    sync::Arc,
};

use tokio::io::AsyncRead;

use crate::{
    models::{
        event::{Event, EventName},
        object::{ObjectDao, ObjectId, Upload},
    },
    repositories::{object::ObjectRepository, session::SessionRepository},
};

use super::websocket::WebSocketService;

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

pub struct ObjectService<O, S> {
    repository: Arc<O>,
    websocket: Arc<WebSocketService<S>>,
}

impl<O, S> ObjectService<O, S> {
    pub fn new(repository: Arc<O>, websocket: Arc<WebSocketService<S>>) -> Self {
        Self {
            repository,
            websocket,
        }
    }
}

impl<O: ObjectRepository, S> ObjectService<O, S> {
    pub async fn list(&self) -> Result<Vec<ObjectDao>> {
        normalize_result(self.repository.list().await)
    }

    pub async fn get(&self, oid: &ObjectId) -> Result<ObjectDao> {
        normalize_result(self.repository.get(oid).await)
    }

    pub async fn download(
        &self,
        oid: &ObjectId,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>> {
        normalize_result(self.repository.download(oid).await)
    }

    pub async fn auth(&self, auth_key: &[u8]) -> Result<bool> {
        self.repository
            .auth(auth_key)
            .await
            .map_err(|e| ObjectError::Other(e))
    }
}

impl<O: ObjectRepository, S: SessionRepository> ObjectService<O, S> {
    pub async fn put(&self, upload: Upload) -> Result<ObjectDao> {
        normalize_result(
            self.repository
                .put(upload)
                .await
                .map(|obj| self.publish_object_created(obj)),
        )
    }

    pub async fn upload<R>(&self, upload: Upload, reader: R) -> Result<ObjectDao>
    where
        R: AsyncRead + Unpin + Send + Sync,
    {
        normalize_result(
            self.repository
                .upload(upload, reader)
                .await
                .map(|obj| self.publish_object_created(obj)),
        )
    }

    pub async fn delete(&self, oid: &ObjectId) -> Result<()> {
        normalize_result(self.repository.delete(oid).await.map(|_| {
            let event = Event::new(EventName::ObjectDeleted, *oid);
            self.websocket.publish(event);
        }))
    }

    fn publish_object_created(&self, obj: ObjectDao) -> ObjectDao {
        let event = Event::new(EventName::ObjectCreated, obj.id);
        self.websocket.publish(event);
        obj
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
