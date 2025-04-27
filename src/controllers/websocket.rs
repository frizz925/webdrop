use std::{error::Error, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use futures::SinkExt;
use tracing::{event, Level};

use crate::{
    models::{
        event::{Event, EventName},
        session::SessionId,
    },
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
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let service = (controller.factory)(&sid);
    let subscriber = service.subscribe();
    ws.on_upgrade(move |socket| async {
        if let Err(e) = handle_socket(socket, subscriber).await {
            event!(Level::ERROR, "WebSocket error: {e}")
        }
    })
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
