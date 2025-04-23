use std::{str::FromStr, sync::Arc};

use axum::{
    debug_handler,
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use qrcode::{render::svg, QrCode};
use serde::Deserialize;
use tower_http::services::ServeDir;

use crate::{models::session::SessionId, repositories::SessionRepository};

use super::{session::SessionController, ConcreteSessionRepository, PUBLIC_PATH};

const SESSION_COOKIE: &str = "my-session-id";

pub struct MainController {
    session: Arc<ConcreteSessionRepository>,
}

#[derive(Deserialize)]
struct QrcodeRequest {
    content: String,
}

impl MainController {
    pub fn new(session: ConcreteSessionRepository) -> Self {
        Self {
            session: Arc::new(session),
        }
    }

    pub fn into_router(self) -> Router {
        let session = self.session.clone();
        let state = Arc::new(self);
        Router::new()
            .route("/", get(entry_handler).with_state(state))
            .route("/qrcode", post(qrcode_handler))
            .nest("/s", SessionController::new(session).into_router())
            .fallback_service(ServeDir::new(PUBLIC_PATH))
    }
}

#[debug_handler]
async fn entry_handler(
    State(controller): State<Arc<MainController>>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), impl IntoResponse> {
    let opt_sid = jar.get(SESSION_COOKIE).and_then(|c| {
        let s = c.value_trimmed();
        SessionId::from_str(s).ok()
    });
    let (jar, sid) = if let Some(sid) = opt_sid {
        (jar, sid)
    } else {
        let repository = &controller.session;
        match repository.create().await {
            Ok(sess) => {
                let cookie = Cookie::new(SESSION_COOKIE, sess.id.to_string());
                let jar = jar.add(cookie);
                (jar, sess.id)
            }
            Err(e) => {
                let resp = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.to_string())
                    .unwrap();
                return Err(resp);
            }
        }
    };
    let redirect = format!("/s/{sid}");
    Ok((jar, Redirect::to(&redirect)))
}

async fn qrcode_handler(Json(req): Json<QrcodeRequest>) -> impl IntoResponse {
    match QrCode::new(req.content) {
        Ok(code) => Response::builder()
            .header(header::CONTENT_TYPE, "image/svg")
            .body(code.render::<svg::Color>().build())
            .unwrap(),
        Err(e) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(e.to_string())
            .unwrap(),
    }
}
