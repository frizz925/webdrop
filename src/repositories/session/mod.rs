mod fs;

use std::future::Future;

use crate::models::session::{Session, SessionId};

use super::Result;

pub use fs::SessionFsRepository;

pub trait SessionRepository {
    fn create(&self) -> impl Future<Output = Result<Session>>;

    fn upsert(&self, sess: Session) -> impl Future<Output = Result<()>>;

    fn get(&self, sid: &SessionId) -> impl Future<Output = Result<Session>>;

    fn exists(&self, sid: &SessionId) -> impl Future<Output = Result<bool>>;
}
