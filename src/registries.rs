use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{
    models::session::SessionId, ConcreteObjectRepository, ConcreteObjectService,
    ConcreteSessionRepository, ConcreteWebSocketService, STORAGE_DIR,
};

type SessionRegistry<T> = LazyLock<RwLock<HashMap<SessionId, Arc<T>>>>;

pub static SESSION_REPOSITORY: LazyLock<Arc<ConcreteSessionRepository>> =
    LazyLock::new(|| Arc::new(ConcreteSessionRepository::new(STORAGE_DIR)));
pub static OBJECT_REPOSITORIES: SessionRegistry<ConcreteObjectRepository> = register();
pub static OBJECT_SERVICES: SessionRegistry<ConcreteObjectService> = register();
pub static WEBSOCKET_SERVICES: SessionRegistry<ConcreteWebSocketService> = register();

const fn register<T>() -> SessionRegistry<T> {
    LazyLock::new(|| RwLock::new(HashMap::new()))
}
