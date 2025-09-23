use std::{
    error::Error as StdError,
    fmt::Display,
    io::{Error as IoError, ErrorKind},
    result::Result as StdResult,
    sync::Arc,
};

use crate::{
    models::{
        event::EventName,
        session::{CreateSession, Session, SessionId},
    },
    registries::{OBJECT_SERVICES, WEBSOCKET_SERVICES},
    repositories::session::SessionRepository,
    WebSocketServiceFactory,
};

#[derive(Debug)]
pub enum SessionError {
    NotFound,
    Other(Box<dyn StdError>),
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotFound => "Session not found".to_owned(),
            Self::Other(e) => e.to_string(),
        };
        f.write_str(&s)
    }
}

impl StdError for SessionError {}

pub type Result<T> = StdResult<T, SessionError>;

pub struct SessionService<R> {
    repository: Arc<R>,
    websocket: WebSocketServiceFactory,
}

impl<R: SessionRepository> SessionService<R> {
    pub fn new(repository: Arc<R>, websocket: WebSocketServiceFactory) -> Self {
        Self {
            repository,
            websocket,
        }
    }

    pub async fn create(&self, req: Option<CreateSession>) -> Result<Session> {
        let sid = SessionId::generate();
        let sess = Session::new(sid, req);
        normalize_result(self.repository.create(&sess).await.map(|_| sess))
    }

    pub async fn get(&self, sid: &SessionId) -> Result<Session> {
        normalize_result(self.repository.get(sid).await)
    }

    pub async fn exists(&self, sid: &SessionId) -> Result<bool> {
        normalize_result(self.repository.exists(sid).await)
    }

    pub async fn delete(&self, sid: &SessionId) -> Result<()> {
        normalize_result(self.repository.delete(sid).await.map(|_| {
            let service = (self.websocket)(sid);
            service.publish(EventName::SessionDeleted.into_event());
            OBJECT_SERVICES.write().unwrap().remove(sid);
            WEBSOCKET_SERVICES.write().unwrap().remove(sid);
        }))
    }

    pub async fn session_auth(&self, sid: &SessionId, auth_key: &[u8]) -> Result<bool> {
        if let Some(expected) = self
            .repository
            .auth_key(sid)
            .await
            .map_err(normalize_error)?
        {
            Ok(auth_key == &expected)
        } else {
            Ok(true)
        }
    }
}

fn normalize_result<T>(res: StdResult<T, Box<dyn StdError>>) -> Result<T> {
    res.map_err(normalize_error)
}

fn normalize_error(err: Box<dyn StdError>) -> SessionError {
    if let Some(e) = err.downcast_ref::<IoError>() {
        if e.kind() == ErrorKind::NotFound {
            return SessionError::NotFound;
        }
    }
    SessionError::Other(err)
}
