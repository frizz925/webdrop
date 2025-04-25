use std::sync::Arc;

use crate::{
    models::event::Event,
    utils::sync::{Channel, Dispatcher},
};

pub struct WebSocketService {
    dispatcher: Dispatcher<Event>,
}

impl WebSocketService {
    pub fn new(backlog: usize) -> Self {
        Self {
            dispatcher: Dispatcher::new(backlog),
        }
    }

    pub fn subscribe(&self) -> Arc<Channel<Event>> {
        self.dispatcher.subscribe()
    }

    pub fn dispatch(&self, event: Event) {
        self.dispatcher.send(&event);
    }
}
