mod fs;

use std::future::Future;

use crate::models::session::{CreateSession, SessionDto, SessionId};

use super::Result;

pub use fs::SessionFsRepository;

pub trait SessionRepository {
    fn exists(&self, sid: &SessionId) -> impl Future<Output = Result<bool>>;

    fn get(&self, sid: &SessionId) -> impl Future<Output = Result<SessionDto>>;

    fn create(&self, req: Option<CreateSession>) -> impl Future<Output = Result<SessionDto>>;

    fn delete(&self, sid: &SessionId) -> impl Future<Output = Result<()>>;

    fn auth(&self, sid: &SessionId, auth_key: &[u8]) -> impl Future<Output = Result<bool>>;
}
