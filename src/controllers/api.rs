use std::{error::Error, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, head, post},
    Json, Router,
};
use log::error;
use serde::Serialize;

use crate::{
    models::{
        object::{ObjectId, ObjectResult, Upload},
        session::{Session, SessionId},
    },
    ConcreteSessionService, ObjectServiceFactory,
};

#[derive(Serialize)]
struct HeadSessionResponse {
    result: bool,
}

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
                head(head_session).get(get_session).post(create_object),
            )
            .route("/session/{sid}/{oid}", get(get_object))
            .with_state(state)
    }
}

async fn create_session(
    State(controller): State<Arc<ApiController>>,
) -> Result<Json<Session>, StatusCode> {
    normalize_result(controller.session.create().await)
}

async fn head_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
) -> Result<Json<HeadSessionResponse>, StatusCode> {
    normalize_result(
        controller
            .session
            .exists(&sid)
            .await
            .map(|result| HeadSessionResponse { result }),
    )
}

async fn get_session(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
) -> Result<Json<Session>, StatusCode> {
    normalize_result(controller.session.get(&sid).await)
}

async fn create_object(
    State(controller): State<Arc<ApiController>>,
    Path(sid): Path<SessionId>,
    Json(upload): Json<Upload>,
) -> Result<Json<ObjectResult>, StatusCode> {
    let service = (controller.object)(&sid);
    normalize_result(service.put(upload).await)
}

async fn get_object(
    State(controller): State<Arc<ApiController>>,
    Path((sid, oid)): Path<(SessionId, ObjectId)>,
) -> Result<Json<ObjectResult>, StatusCode> {
    let service = (controller.object)(&sid);
    normalize_result(service.get(&oid).await)
}

fn normalize_result<T, E: Error>(res: Result<T, E>) -> Result<Json<T>, StatusCode> {
    res.map_or_else(error_to_status, value_to_json)
}

fn value_to_json<T, E>(v: T) -> Result<Json<T>, E> {
    Ok(Json(v))
}

fn error_to_status<T, E: Error>(e: E) -> Result<T, StatusCode> {
    error!("Failed to create session: {e}");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
