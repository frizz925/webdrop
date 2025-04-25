use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{
    models::session::SessionId, services::websocket::WebSocketService, ConcreteObjectService,
};

type SessionRegistry<T> = LazyLock<RwLock<HashMap<SessionId, Arc<T>>>>;

pub static WEBSOCKET_SERVICES: SessionRegistry<WebSocketService> = register();
pub static OBJECT_SERVICES: SessionRegistry<ConcreteObjectService> = register();

const fn register<T>() -> SessionRegistry<T> {
    LazyLock::new(|| RwLock::new(HashMap::new()))
}
