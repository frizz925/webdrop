mod main;
mod session;

pub use main::MainController;

use crate::repositories::session::SessionFsRepository;

pub(super) type ConcreteSessionRepository = SessionFsRepository;
