mod fs;

use std::future::Future;

use crate::models::session::{CreateSession, Session, SessionId};

use super::Result;

pub use fs::SessionFsRepository;

pub trait SessionRepository {
    fn create(&self, req: CreateSession) -> impl Future<Output = Result<Session>>;

    fn get(&self, sid: &SessionId) -> impl Future<Output = Result<Session>>;

    fn exists(&self, sid: &SessionId) -> impl Future<Output = Result<bool>>;

    fn delete(&self, sid: &SessionId) -> impl Future<Output = Result<()>>;

    fn auth(&self, sid: &SessionId, auth_key: &str) -> impl Future<Output = Result<bool>>;
}
