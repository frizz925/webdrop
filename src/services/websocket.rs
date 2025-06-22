use std::{error::Error, fmt::Display, result::Result as StdResult, sync::Arc};

use crate::{
    models::{event::Event, session::SessionId},
    repositories::session::SessionRepository,
    utils::sync::{PubSub, Subscriber},
};

pub struct WebSocketService<R> {
    pubsub: PubSub<Event>,
    repository: Arc<R>,
}

#[derive(Debug)]
pub enum WebSocketError {
    Other(Box<dyn Error>),
}

impl Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Other(e) => e.to_string(),
        };
        f.write_str(&s)
    }
}

impl Error for WebSocketError {}

pub type Result<T> = StdResult<T, WebSocketError>;

impl<R: SessionRepository> WebSocketService<R> {
    pub fn new(backlog: usize, repository: Arc<R>) -> Self {
        Self {
            pubsub: PubSub::new(backlog),
            repository,
        }
    }

    pub fn subscribe(&self) -> Arc<Subscriber<Event>> {
        self.pubsub.subscribe()
    }

    pub fn publish(&self, event: Event) {
        self.pubsub.publish(&event);
    }

    pub async fn auth(&self, sid: &SessionId, auth_key: &[u8]) -> Result<bool> {
        self.repository
            .auth(sid, auth_key)
            .await
            .map_err(|e| WebSocketError::Other(e))
    }
}
