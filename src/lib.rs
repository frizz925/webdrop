use models::session::SessionId;
use repositories::{object::ObjectFsRepository, session::SessionFsRepository};
use services::{object::ObjectService, session::SessionService};

pub mod controllers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

pub type ConcreteServiceRepository = SessionFsRepository;
pub type ConcreteSessionService = SessionService<ConcreteServiceRepository>;

pub type ConcreteObjectRepository = ObjectFsRepository;
pub type ConcreteObjectService = ObjectService<ConcreteObjectRepository>;
pub type ObjectServiceFactory = fn(&SessionId) -> ConcreteObjectService;
