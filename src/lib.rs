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

pub type ConcreteServiceRepository = SessionFsRepository;
pub type ConcreteSessionService = SessionService<ConcreteServiceRepository>;

pub type ConcreteObjectRepository = ObjectFsRepository;
pub type ConcreteObjectService = ObjectService<ConcreteObjectRepository>;

pub(crate) type WebSocketServiceFactory = fn(&SessionId) -> Arc<WebSocketService>;
pub(crate) type ObjectServiceFactory = fn(&SessionId) -> Arc<ConcreteObjectService>;
