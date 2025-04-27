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
        object::{Object, ObjectId, Upload},
    },
    repositories::object::ObjectRepository,
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

pub struct ObjectService<R> {
    repository: R,
    websocket: Arc<WebSocketService>,
}

impl<S: ObjectRepository> ObjectService<S> {
    pub fn new(repository: S, websocket: Arc<WebSocketService>) -> Self {
        Self {
            repository,
            websocket,
        }
    }

    pub async fn put(&self, upload: Upload) -> Result<Object> {
        normalize_result(
            self.repository
                .put(upload)
                .await
                .map(|obj| self.publish_object_created(obj)),
        )
    }

    pub async fn upload<R>(&self, upload: Upload, reader: R) -> Result<Object>
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
        normalize_result(self.repository.delete(oid).await.map(|_| {
            let event = Event::new(EventName::ObjectDeleted, *oid);
            self.websocket.publish(event);
        }))
    }

    fn publish_object_created(&self, obj: Object) -> Object {
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
