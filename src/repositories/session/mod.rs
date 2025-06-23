mod fs;

use std::future::Future;

use crate::models::session::{CreateSession, Session, SessionId};

use super::Result;

pub use fs::SessionFsRepository;

pub trait SessionRepository {
    fn list(&self) -> impl Future<Output = Result<Vec<SessionId>>>;

    fn create(&self, req: Option<CreateSession>) -> impl Future<Output = Result<Session>>;

    fn exists(&self, sid: &SessionId) -> impl Future<Output = Result<bool>>;

    fn get(&self, sid: &SessionId) -> impl Future<Output = Result<Session>>;

    fn delete(&self, sid: &SessionId) -> impl Future<Output = Result<()>>;

    fn auth(&self, sid: &SessionId, auth_key: &[u8]) -> impl Future<Output = Result<bool>>;
}
