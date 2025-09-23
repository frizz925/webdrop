mod fs;

use std::future::Future;

use crate::models::session::{Session, SessionId};

use super::Result;

pub use fs::SessionFsRepository;

pub trait SessionRepository {
    fn list(&self) -> impl Future<Output = Result<Vec<SessionId>>>;

    fn create(&self, sess: &Session) -> impl Future<Output = Result<()>>;

    fn exists(&self, sid: &SessionId) -> impl Future<Output = Result<bool>>;

    fn get(&self, sid: &SessionId) -> impl Future<Output = Result<Session>>;

    fn delete(&self, sid: &SessionId) -> impl Future<Output = Result<()>>;

    fn auth_key(&self, sid: &SessionId) -> impl Future<Output = Result<Option<Vec<u8>>>>;
}
