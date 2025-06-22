use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, head, post},
    Json, Router,
};
use tracing::{event, Level};

use crate::{
    controllers::AuthKeyExtractor,
    models::{
        object::{ObjectDao, ObjectId, Upload},
        session::{CreateSession, SessionDto, SessionId},
    },
    repositories::session::SessionRepository,
    services::session::{SessionError, SessionService},
    ConcreteSessionService, ObjectServiceFactory,
};

use super::StatusCodeError;

pub struct ApiController {
    session: Arc<ConcreteSessionService>,
    object: ObjectServiceFactory,
}

impl ApiController {
    pub fn new(session: Arc<ConcreteSessionService>, object: ObjectServiceFactory) -> Self {
        Self { session, object }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        Router::new()
            .route("/session", post(create_session))
            .route("/session/encrypted", post(create_session_encrypted))
            .route(
                "/session/{sid}",
                head(head_session).get(get_session).delete(delete_session),
            )
            .route(
                "/session/{sid}/objects",
                get(list_objects).post(create_object),
            )
            .route(
                "/session/{sid}/objects/{oid}",
                get(get_object).delete(delete_object),
            )
            .with_state(state)
    }
}

async fn create_session(
    State(controller): State<Arc<ApiController>>,
) -> Result<Json<SessionDto>, StatusCode> {
    normalize_json_result("create session", controller.session.create(None).await)
}

async fn create_session_encrypted(
    State(controller): State<Arc<ApiController>>,
    Json(request): Json<CreateSession>,
) -> Result<Json<SessionDto>, StatusCode> {
    normalize_json_result(
        "create session encrypted",
        controller.session.create(Some(request)).await,
    )
}

async fn head_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
) -> Result<StatusCode, StatusCode> {
    normalize_result(
        "head session",
        controller.session.exists(&sid).await.map(|exists| {
            if exists {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }),
    )
}

async fn get_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
) -> Result<Json<SessionDto>, StatusCode> {
    normalize_json_result("get session", controller.session.get(&sid).await)
}

async fn delete_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    check_auth_key(&controller.session, &sid, &headers).await?;
    normalize_result(
        "delete session",
        controller
            .session
            .delete(&sid)
            .await
            .map(|_| StatusCode::NO_CONTENT),
    )
}

async fn list_objects(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
    headers: HeaderMap,
) -> Result<Json<Vec<ObjectDao>>, StatusCode> {
    check_auth_key(&controller.session, &sid, &headers).await?;
    let service = (controller.object)(&sid);
    normalize_json_result("list objects", service.list().await)
}

async fn get_object(
    State(controller): State<Arc<ApiController>>,
    Path((sid, oid)): Path<(SessionId, ObjectId)>,
    headers: HeaderMap,
) -> Result<Json<ObjectDao>, StatusCode> {
    check_auth_key(&controller.session, &sid, &headers).await?;
    let service = (controller.object)(&sid);
    normalize_json_result("get object", service.get(&oid).await)
}

async fn create_object(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
    headers: HeaderMap,
    Json(upload): Json<Upload>,
) -> Result<Json<ObjectDao>, StatusCode> {
    check_auth_key(&controller.session, &sid, &headers).await?;
    let service = (controller.object)(&sid);
    normalize_json_result("create object", service.put(upload).await)
}

async fn delete_object(
    State(controller): State<Arc<ApiController>>,
    Path((sid, oid)): Path<(SessionId, ObjectId)>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    check_auth_key(&controller.session, &sid, &headers).await?;
    let service = (controller.object)(&sid);
    normalize_result(
        "delete object",
        service.delete(&oid).await.map(|_| StatusCode::NO_CONTENT),
    )
}

async fn check_auth_key<R: SessionRepository>(
    service: &Arc<SessionService<R>>,
    sid: &SessionId,
    headers: &HeaderMap,
) -> Result<(), StatusCode> {
    let auth_key = headers.extract_auth_key()?;
    if service
        .auth(sid, &auth_key)
        .await
        .map_err(|err| match err {
            SessionError::NotFound => StatusCode::NOT_FOUND,
            SessionError::Other(e) => {
                event!(Level::ERROR, "Authentication error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?
    {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn normalize_result<T, E: StatusCodeError>(
    action: &'static str,
    res: Result<T, E>,
) -> Result<T, StatusCode> {
    res.map_err(error_to_status(action))
}

fn normalize_json_result<T, E: StatusCodeError>(
    action: &'static str,
    res: Result<T, E>,
) -> Result<Json<T>, StatusCode> {
    res.map(Json).map_err(error_to_status(action))
}

fn error_to_status<E: StatusCodeError>(action: &'static str) -> impl FnOnce(E) -> StatusCode {
    move |e| {
        event!(Level::ERROR, "Failed to {action}: {e}");
        e.into_status_code()
    }
}
