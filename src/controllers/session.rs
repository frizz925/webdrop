use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use tokio_util::io::ReaderStream;
use tower_http::services::ServeFile;

use crate::{
    models::{object::ObjectId, session::SessionId},
    services::object::Error as ObjectError,
    ObjectServiceFactory,
};

use super::PUBLIC_PATH;

pub struct SessionController {
    object: ObjectServiceFactory,
}

impl SessionController {
    pub fn new(object: ObjectServiceFactory) -> Self {
        Self { object }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        let index = format!("{PUBLIC_PATH}/index.html");
        Router::new()
            .route("/{sid}", get_service(ServeFile::new(index))) // TODO: Implement upload handler
            .route("/{sid}/{oid}/{name}", get(download_handler))
            .with_state(state)
    }
}

async fn download_handler(
    State(controller): State<Arc<SessionController>>,
    Path((sid, oid, name)): Path<(SessionId, ObjectId, String)>,
) -> Result<Body, (StatusCode, String)> {
    let service = (controller.object)(&sid);
    match service.download(&oid, &name).await {
        Ok(reader) => Ok(Body::from_stream(ReaderStream::new(reader))),
        Err(e) => {
            let code = match e {
                ObjectError::ObjectNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err((code, e.to_string()))
        }
    }
}
