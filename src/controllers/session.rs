use std::{
    io::{Error as IoError, ErrorKind},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use tokio_util::io::ReaderStream;
use tower_http::services::ServeFile;
use uuid::Uuid;

use crate::{
    models::session::SessionId,
    repositories::{session::Error as SessionError, SessionRepository},
};

use super::ConcreteSessionRepository;

pub struct SessionController {
    repository: Arc<ConcreteSessionRepository>,
}

impl SessionController {
    pub fn new(repository: Arc<ConcreteSessionRepository>) -> Self {
        Self { repository }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        Router::new()
            .route("/{sid}", get_service(ServeFile::new("web/index.html"))) // TODO: Implement upload handler
            .route("/{sid}/{fid}", get(download_handler))
            .with_state(state)
    }
}

async fn download_handler(
    State(controller): State<Arc<SessionController>>,
    Path((sid, fid)): Path<(SessionId, Uuid)>,
) -> Result<Body, (StatusCode, String)> {
    let repository = &controller.repository;
    match repository.download(sid, fid).await {
        Ok(reader) => Ok(Body::from_stream(ReaderStream::new(reader))),
        Err(e) => {
            let result = if let Some(inner) = e.downcast_ref::<IoError>() {
                if inner.kind() == ErrorKind::NotFound {
                    Ok(StatusCode::NOT_FOUND)
                } else {
                    Err(e)
                }
            } else if e.is::<SessionError>() {
                Ok(StatusCode::NOT_FOUND)
            } else {
                Err(e)
            };
            let resp = result.map_or_else(
                |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                |c| (c, String::default()),
            );
            Err(resp)
        }
    }
}
