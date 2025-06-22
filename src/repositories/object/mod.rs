mod fs;

use std::future::Future;

use tokio::io::AsyncRead;

use crate::models::object::{ObjectDao, ObjectId, Upload};

use super::Result;

pub use fs::ObjectFsRepository;

pub trait ObjectRepository: Unpin + Send + Sync {
    fn list(&self) -> impl Future<Output = Result<Vec<ObjectDao>>>;

    fn get(&self, oid: &ObjectId) -> impl Future<Output = Result<ObjectDao>>;

    fn put(&self, upload: Upload) -> impl Future<Output = Result<ObjectDao>>;

    fn upload<R>(&self, upload: Upload, reader: R) -> impl Future<Output = Result<ObjectDao>>
    where
        R: AsyncRead + Unpin + Send + Sync;

    fn download(
        &self,
        oid: &ObjectId,
    ) -> impl Future<Output = Result<Box<dyn AsyncRead + Unpin + Send + Sync>>>;

    fn delete(&self, oid: &ObjectId) -> impl Future<Output = Result<()>>;

    fn auth(&self, auth_key: &[u8]) -> impl Future<Output = Result<bool>>;
}
