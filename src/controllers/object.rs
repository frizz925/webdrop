use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{multipart::MultipartError, DefaultBodyLimit, Multipart, Path, Query, State},
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures::TryStreamExt;
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{event, Level};

use crate::{
    controllers::{AuthKeyExtractor, AuthParams},
    models::{
        object::{ObjectDto, ObjectId, Upload},
        session::SessionId,
    },
    repositories::{object::ObjectRepository, session::SessionRepository},
    services::{
        object::{ObjectError, ObjectService},
        session::{SessionError, SessionService},
    },
    ConcreteSessionService, ObjectServiceFactory,
};

pub struct ObjectController {
    factory: ObjectServiceFactory,
    session: Arc<ConcreteSessionService>,
}

impl ObjectController {
    pub fn new(factory: ObjectServiceFactory, session: Arc<ConcreteSessionService>) -> Self {
        Self { factory, session }
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
    Path((sid, oid, filename)): Path<(SessionId, ObjectId, String)>,
    Query(params): Query<AuthParams>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = (controller.factory)(&sid);
    check_object_auth_key(&service, &oid, &params).await?;

    match service.download(&oid).await {
        Ok(reader) => {
            let mime = mime_guess::from_path(&filename).first_or_octet_stream();
            let body = Body::from_stream(ReaderStream::new(reader));
            Response::builder()
                .header(header::CONTENT_TYPE, mime.essence_str())
                .body(body)
                .map_err(|e| {
                    event!(Level::ERROR, "Download response error: {e}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })
        }
        Err(err) => match err {
            ObjectError::Other(e) => {
                event!(Level::ERROR, "Download error: {e}");
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
) -> Result<Json<ObjectDto>, StatusCode> {
    check_session_auth_key(&controller.session, &sid, &headers).await?;

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

async fn do_upload<O: ObjectRepository, S: SessionRepository>(
    service: Arc<ObjectService<O, S>>,
    mut multipart: Multipart,
) -> Result<Option<ObjectDto>, Box<dyn Error>> {
    let mut opt_upload: Option<Upload> = None;
    while let Some(field) = multipart.next_field().await? {
        match field.name().unwrap_or_default() {
            "meta" => {
                let content = field.text().await?;
                let upload = serde_json::from_str(&content)?;
                opt_upload = Some(upload);
            }
            "file" if opt_upload.is_some() => {
                let upload = opt_upload.unwrap();
                let stream = field.map_err(|err| IoError::new(ErrorKind::Other, err));
                let reader = StreamReader::new(stream);
                let obj = service.upload(upload, reader).await?;
                return Ok(Some(obj.into()));
            }
            _ => continue,
        }
    }
    Ok(None)
}

async fn check_object_auth_key<O: ObjectRepository, S, E: AuthKeyExtractor>(
    service: &Arc<ObjectService<O, S>>,
    oid: &ObjectId,
    extractor: E,
) -> Result<(), StatusCode> {
    let auth_key = extractor.extract_auth_key()?;
    match service.object_auth(oid, &auth_key).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(StatusCode::UNAUTHORIZED),
        Err(ObjectError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(ObjectError::Other(e)) => {
            event!(Level::ERROR, "Authentication error: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn check_session_auth_key<S: SessionRepository, E: AuthKeyExtractor>(
    service: &Arc<SessionService<S>>,
    sid: &SessionId,
    extractor: E,
) -> Result<(), StatusCode> {
    let auth_key = extractor.extract_auth_key()?;
    match service.session_auth(sid, &auth_key).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(StatusCode::UNAUTHORIZED),
        Err(SessionError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(SessionError::Other(e)) => {
            event!(Level::ERROR, "Authentication error: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
