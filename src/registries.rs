use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{
    models::session::SessionId, ConcreteObjectRepository, ConcreteObjectService,
    ConcreteWebSocketService,
};

type SessionRegistry<T> = LazyLock<RwLock<HashMap<SessionId, Arc<T>>>>;

pub static WEBSOCKET_SERVICES: SessionRegistry<ConcreteWebSocketService> = register();
pub static OBJECT_SERVICES: SessionRegistry<ConcreteObjectService> = register();

pub static OBJECT_REPOSITORIES: SessionRegistry<ConcreteObjectRepository> = register();

const fn register<T>() -> SessionRegistry<T> {
    LazyLock::new(|| RwLock::new(HashMap::new()))
}
