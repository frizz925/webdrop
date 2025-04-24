use std::sync::Arc;

use axum::Router;
use tower_http::services::ServeDir;

use crate::{ConcreteSessionService, ObjectServiceFactory};

use super::{api::ApiController, session::SessionController, PUBLIC_PATH};

pub struct MainController {
    session: Arc<ConcreteSessionService>,
    object: ObjectServiceFactory,
}

impl MainController {
    pub fn new(session: ConcreteSessionService, object: ObjectServiceFactory) -> Self {
        Self {
            session: Arc::new(session),
            object,
        }
    }

    pub fn into_router(self) -> Router {
        let session = SessionController::new(self.object);
        let api = ApiController::new(self.session, self.object);
        Router::new()
            .nest("/s", session.into_router())
            .nest("/api", api.into_router())
            .fallback_service(ServeDir::new(PUBLIC_PATH))
    }
}
