use std::sync::Arc;

use crate::{
    models::event::Event,
    utils::sync::{PubSub, Subscriber},
};

pub struct WebSocketService {
    pubsub: PubSub<Event>,
}

impl WebSocketService {
    pub fn new(backlog: usize) -> Self {
        Self {
            pubsub: PubSub::new(backlog),
        }
    }

    pub fn subscribe(&self) -> Arc<Subscriber<Event>> {
        self.pubsub.subscribe()
    }

    pub fn publish(&self, event: Event) {
        self.pubsub.publish(&event);
    }
}
