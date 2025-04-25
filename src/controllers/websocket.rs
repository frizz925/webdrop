use axum::Router;

pub struct WebSocketController {}

impl WebSocketController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_router(self) -> Router {
        Router::new()
    }
}
