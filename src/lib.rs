use std::sync::Arc;

use models::session::SessionId;
use repositories::{object::ObjectFsRepository, session::SessionFsRepository};
use services::{object::ObjectService, session::SessionService, websocket::WebSocketService};

pub mod controllers;
pub mod models;
pub mod registries;
pub mod repositories;
pub mod services;
pub mod utils;

pub type ConcreteSessionRepository = SessionFsRepository;
pub type ConcreteSessionService = SessionService<ConcreteSessionRepository>;

pub type ConcreteObjectRepository = ObjectFsRepository;
pub type ConcreteObjectService = ObjectService<ConcreteObjectRepository, ConcreteSessionRepository>;

pub type ConcreteWebSocketService = WebSocketService<ConcreteSessionRepository>;

pub(crate) type WebSocketServiceFactory = fn(&SessionId) -> Arc<ConcreteWebSocketService>;
pub(crate) type ObjectServiceFactory = fn(&SessionId) -> Arc<ConcreteObjectService>;

pub const STORAGE_DIR: &str = "storage";
