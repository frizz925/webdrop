use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, head, post},
    Json, Router,
};
use log::error;

use crate::{
    models::{
        object::{Object, ObjectId, Upload},
        session::{Session, SessionId},
    },
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
            .route(
                "/session/{sid}",
                head(head_session)
                    .get(get_session)
                    .delete(delete_session)
                    .post(create_object),
            )
            .route(
                "/session/{sid}/{oid}",
                get(get_object).delete(delete_object),
            )
            .with_state(state)
    }
}

async fn create_session(
    State(controller): State<Arc<ApiController>>,
) -> Result<Json<Session>, StatusCode> {
    normalize_json_result("create session", controller.session.create().await)
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
) -> Result<Json<Session>, StatusCode> {
    normalize_json_result("get session", controller.session.get(&sid).await)
}

async fn delete_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
) -> Result<StatusCode, StatusCode> {
    normalize_result(
        "get session",
        controller
            .session
            .delete(&sid)
            .await
            .map(|_| StatusCode::NO_CONTENT),
    )
}

async fn create_object(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
    Json(upload): Json<Upload>,
) -> Result<Json<Object>, StatusCode> {
    let service = (controller.object)(&sid);
    normalize_json_result("create object", service.put(upload).await)
}

async fn get_object(
    State(controller): State<Arc<ApiController>>,
    Path((sid, oid)): Path<(SessionId, ObjectId)>,
) -> Result<Json<Object>, StatusCode> {
    let service = (controller.object)(&sid);
    normalize_json_result("get object", service.get(&oid).await)
}

async fn delete_object(
    State(controller): State<Arc<ApiController>>,
    Path((sid, oid)): Path<(SessionId, ObjectId)>,
) -> Result<StatusCode, StatusCode> {
    let service = (controller.object)(&sid);
    normalize_result(
        "delete object",
        service.delete(&oid).await.map(|_| StatusCode::NO_CONTENT),
    )
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
        error!("Failed to {action}: {e}");
        e.into_status_code()
    }
}
