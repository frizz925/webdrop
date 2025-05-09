use std::sync::Arc;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::{ConcreteSessionService, ObjectServiceFactory, WebSocketServiceFactory};

use super::{
    api::ApiController, object::ObjectController, websocket::WebSocketController, PUBLIC_PATH,
};

pub struct MainController {
    session: Arc<ConcreteSessionService>,
    websocket: WebSocketServiceFactory,
    object: ObjectServiceFactory,
}

impl MainController {
    pub fn new(
        session: ConcreteSessionService,
        websocket: WebSocketServiceFactory,
        object: ObjectServiceFactory,
    ) -> Self {
        Self {
            session: Arc::new(session),
            websocket,
            object,
        }
    }

    pub fn into_router(self) -> Router {
        let index = format!("{PUBLIC_PATH}/index.html");
        let ws = WebSocketController::new(self.websocket);
        let api = ApiController::new(self.session, self.object);
        let object = ObjectController::new(self.object);
        Router::new()
            .nest("/ws", ws.into_router())
            .nest("/api", api.into_router())
            .nest("/objects", object.into_router())
            .route_service("/session/{sid}", ServeFile::new(index))
            .fallback_service(ServeDir::new(PUBLIC_PATH))
    }
}
