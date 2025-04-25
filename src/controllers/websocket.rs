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
    models::{event::Event, session::SessionId},
    utils::sync::Channel,
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
    let channel = service.subscribe();
    ws.on_upgrade(move |socket| async {
        if let Err(e) = handle_socket(socket, channel).await {
            event!(Level::ERROR, "WebSocket error: {e}")
        }
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    channel: Arc<Channel<Event>>,
) -> Result<(), Box<dyn Error>> {
    let mut closed = false;
    while !closed {
        for event in channel.pop().await {
            let json = serde_json::to_string(&event)?;
            let message = Message::Text(json.into());
            socket.send(message).await?;
            if let Event::SessionDeleted = event {
                closed = true;
                break;
            }
        }
    }
    socket.close().await?;
    Ok(())
}
