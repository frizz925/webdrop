mod fs;

use std::future::Future;

use tokio::io::AsyncRead;

use crate::models::object::{Object, ObjectId, Upload};

use super::Result;

pub use fs::ObjectFsRepository;

pub trait ObjectRepository: Unpin + Send + Sync {
    fn list(&self) -> impl Future<Output = Result<Vec<Object>>>;

    fn put(&self, upload: Upload) -> impl Future<Output = Result<Object>>;

    fn upload<R>(&self, upload: Upload, reader: R) -> impl Future<Output = Result<Object>>
    where
        R: AsyncRead + Unpin + Send + Sync;

    fn auth(&self, auth_key: &[u8]) -> impl Future<Output = Result<bool>>;

    fn get(&self, oid: &ObjectId) -> impl Future<Output = Result<Object>>;

    fn download(
        &self,
        oid: &ObjectId,
    ) -> impl Future<Output = Result<Box<dyn AsyncRead + Unpin + Send + Sync>>>;

    fn delete(&self, oid: &ObjectId) -> impl Future<Output = Result<()>>;
}
