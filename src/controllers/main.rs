use std::sync::Arc;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::{ConcreteSessionService, ObjectServiceFactory};

use super::{api::ApiController, object::ObjectController, PUBLIC_PATH};

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
        let index = format!("{PUBLIC_PATH}/index.html");
        let api = ApiController::new(self.session, self.object);
        let object = ObjectController::new(self.object);
        Router::new()
            .nest("/api", api.into_router())
            .nest("/objects", object.into_router())
            .route_service("/{sid}", ServeFile::new(index))
            .fallback_service(ServeDir::new(PUBLIC_PATH))
    }
}
