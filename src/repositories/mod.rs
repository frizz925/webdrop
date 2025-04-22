use std::{error::Error, future::Future, result::Result as StdResult};

use tokio::io::AsyncRead;
use uuid::Uuid;

use crate::models::{
    file::FileUpload,
    session::{Session, SessionId},
};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub trait SessionRepository {
    fn create(&self) -> impl Future<Output = Result<Session>>;

    fn get(&self, sid: SessionId) -> impl Future<Output = Result<Option<Session>>>;

    fn upload<R: AsyncRead + Send + Unpin + 'static>(
        &self,
        sid: SessionId,
        file: FileUpload,
        reader: R,
    ) -> impl Future<Output = Result<()>>;

    fn download(
        &self,
        sid: SessionId,
        fid: Uuid,
    ) -> impl Future<Output = Result<impl AsyncRead + Send + Unpin + 'static>>;
}

pub mod session;
