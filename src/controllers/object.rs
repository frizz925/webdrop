use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{multipart::MultipartError, DefaultBodyLimit, Multipart, Path, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{event, Level};

use crate::{
    controllers::{AuthKeyExtractor, AuthParams},
    models::{
        object::{FileContent, ObjectDao, ObjectId, Upload},
        session::SessionId,
    },
    repositories::{object::ObjectRepository, session::SessionRepository},
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
    Query(params): Query<AuthParams>,
) -> Result<Body, StatusCode> {
    let service = (controller.factory)(&sid);
    check_query_auth_key(&service, &params).await?;
    match service.download(&oid, &name).await {
        Ok(reader) => Ok(Body::from_stream(ReaderStream::new(reader))),
        Err(err) => match err {
            ObjectError::Other(e) => {
                event!(Level::ERROR, "Download erorr: {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _ => Err(StatusCode::NOT_FOUND),
        },
    }
}

async fn upload_handler(
    State(controller): State<Arc<ObjectController>>,
    Path(sid): Path<SessionId>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Result<Json<ObjectDao>, StatusCode> {
    let service = (controller.factory)(&sid);
    check_header_auth_key(&service, &headers).await?;
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

async fn do_upload<O: ObjectRepository, S: SessionRepository>(
    service: Arc<ObjectService<O, S>>,
    mut multipart: Multipart,
) -> Result<Option<ObjectDao>, Box<dyn Error>> {
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

async fn check_query_auth_key<O: ObjectRepository, S>(
    service: &Arc<ObjectService<O, S>>,
    params: &AuthParams,
) -> Result<(), StatusCode> {
    let auth_key = params.extract_auth_key()?;
    check_auth_key(service, &auth_key).await
}

async fn check_header_auth_key<O: ObjectRepository, S>(
    service: &Arc<ObjectService<O, S>>,
    headers: &HeaderMap,
) -> Result<(), StatusCode> {
    let auth_key = headers.extract_auth_key()?;
    check_auth_key(service, &auth_key).await
}

async fn check_auth_key<O: ObjectRepository, S>(
    service: &Arc<ObjectService<O, S>>,
    auth_key: &[u8],
) -> Result<(), StatusCode> {
    if service.auth(auth_key).await.map_err(|err| match err {
        ObjectError::NotFound => StatusCode::NOT_FOUND,
        ObjectError::Other(e) => {
            event!(Level::ERROR, "Authentication error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })? {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
