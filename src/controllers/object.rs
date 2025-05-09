use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{multipart::MultipartError, DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{event, Level};

use crate::{
    models::{
        object::{FileContent, Object, ObjectId, Upload},
        session::SessionId,
    },
    repositories::object::ObjectRepository,
    services::object::{ObjectError, ObjectService},
    ObjectServiceFactory,
};

pub struct ObjectController {
    factory: ObjectServiceFactory,
}

impl ObjectController {
    pub fn new(factory: ObjectServiceFactory) -> Self {
        Self { factory }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        Router::new()
            .route("/{sid}", post(upload_handler))
            .route("/{sid}/{oid}/{name}", get(download_handler))
            .with_state(state)
            .layer(DefaultBodyLimit::disable())
    }
}

async fn download_handler(
    State(controller): State<Arc<ObjectController>>,
    Path((sid, oid, name)): Path<(SessionId, ObjectId, String)>,
) -> Result<Body, (StatusCode, String)> {
    let service = (controller.factory)(&sid);
    match service.download(&oid, &name).await {
        Ok(reader) => Ok(Body::from_stream(ReaderStream::new(reader))),
        Err(e) => {
            let code = match e {
                ObjectError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err((code, e.to_string()))
        }
    }
}

async fn upload_handler(
    State(controller): State<Arc<ObjectController>>,
    Path(sid): Path<SessionId>,
    multipart: Multipart,
) -> Result<Json<Object>, StatusCode> {
    let service = (controller.factory)(&sid);
    let result = do_upload(service, multipart).await.map_err(|err| {
        if let Some(e) = err.downcast_ref::<MultipartError>() {
            event!(Level::ERROR, "Multipart error: {e}");
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    });
    match result {
        Ok(Some(obj)) => Ok(Json(obj)),
        Ok(None) => Err(StatusCode::BAD_REQUEST),
        Err(code) => Err(code),
    }
}

async fn do_upload<R: ObjectRepository>(
    service: Arc<ObjectService<R>>,
    mut multipart: Multipart,
) -> Result<Option<Object>, Box<dyn Error>> {
    while let Some(field) = multipart.next_field().await? {
        if let (Some(name), Some(filename)) = (field.name(), field.file_name()) {
            if name != "file" {
                continue;
            }
            let mime = field.content_type().unwrap_or("application/octet-stream");
            let content = FileContent::new(filename.to_owned());
            let upload = Upload::new(mime.to_owned(), content);
            let stream = field.map_err(|err| IoError::new(ErrorKind::Other, err));
            let reader = StreamReader::new(stream);
            let obj = service.upload(upload, reader).await?;
            return Ok(Some(obj));
        }
    }
    Ok(None)
}
