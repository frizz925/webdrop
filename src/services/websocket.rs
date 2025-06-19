use std::{error::Error, fmt::Display, result::Result as StdResult, sync::Arc};

use futures::TryFutureExt;

use crate::{
    models::event::Event,
    repositories::object::ObjectRepository,
    utils::sync::{PubSub, Subscriber},
};

pub struct WebSocketService<R> {
    pubsub: PubSub<Event>,
    repository: Arc<R>,
}

#[derive(Debug)]
pub struct WebSocketError(Box<dyn Error>);

impl Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for WebSocketError {}

pub type Result<T> = StdResult<T, WebSocketError>;

impl<R: ObjectRepository> WebSocketService<R> {
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

    pub async fn auth(&self, auth_key: &str) -> Result<bool> {
        self.repository
            .auth(auth_key)
            .map_err(|e| WebSocketError(e))
            .await
    }
}
