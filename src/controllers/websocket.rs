use std::{error::Error, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::any,
    Router,
};
use futures::SinkExt;
use tracing::{event, Level};

use crate::{
    controllers::{AuthKeyExtractor, AuthParams},
    models::{
        event::{Event, EventName},
        session::SessionId,
    },
    repositories::session::SessionRepository,
    services::websocket::{WebSocketError, WebSocketService},
    utils::sync::Subscriber,
    WebSocketServiceFactory,
};

pub struct WebSocketController {
    factory: WebSocketServiceFactory,
}

impl WebSocketController {
    pub fn new(factory: WebSocketServiceFactory) -> Self {
        Self { factory }
    }

    pub fn into_router(self) -> Router {
        let state = Arc::new(self);
        Router::new()
            .route("/{sid}", any(websocket_handler))
            .with_state(state)
    }
}

async fn websocket_handler(
    State(controller): State<Arc<WebSocketController>>,
    Path(sid): Path<SessionId>,
    Query(params): Query<AuthParams>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, StatusCode> {
    let service = (controller.factory)(&sid);
    check_auth_key(&service, &sid, &params).await?;
    let subscriber = service.subscribe();
    let res = ws.on_upgrade(move |socket| async {
        if let Err(e) = handle_socket(socket, subscriber).await {
            event!(Level::ERROR, "WebSocket error: {e}")
        }
    });
    Ok(res)
}

async fn handle_socket(
    mut socket: WebSocket,
    subscriber: Arc<Subscriber<Event>>,
) -> Result<(), Box<dyn Error>> {
    let mut closed = false;
    while !closed {
        let mut events: Vec<Event> = subscriber.pop().await.into_iter().collect();
        events.sort_by_key(|e| e.timestamp);
        for event in events {
            let json = serde_json::to_string(&event)?;
            let message = Message::Text(json.into());
            socket.send(message).await?;
            if let EventName::SessionDeleted = event.name {
                closed = true;
                break;
            }
        }
    }
    socket.close().await?;
    Ok(())
}

async fn check_auth_key<R: SessionRepository>(
    service: &Arc<WebSocketService<R>>,
    sid: &SessionId,
    params: &AuthParams,
) -> Result<(), StatusCode> {
    let auth_key = params.extract_auth_key()?;
    if service
        .auth(sid, &auth_key)
        .await
        .map_err(|err| match err {
            WebSocketError::Other(e) => {
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
