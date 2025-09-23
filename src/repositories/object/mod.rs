mod fs;

use std::future::Future;

use tokio::io::AsyncRead;

use crate::models::object::{Object, ObjectId};

use super::Result;

pub use fs::ObjectFsRepository;

pub trait ObjectRepository: Unpin + Send + Sync {
    fn list(&self) -> impl Future<Output = Result<Vec<Object>>>;

    fn put(&self, obj: &Object) -> impl Future<Output = Result<()>>;

    fn upload<R>(&self, obj: &Object, reader: R) -> impl Future<Output = Result<()>>
    where
        R: AsyncRead + Unpin + Send + Sync;

    fn get(&self, oid: &ObjectId) -> impl Future<Output = Result<Object>>;

    fn download(
        &self,
        oid: &ObjectId,
    ) -> impl Future<Output = Result<Box<dyn AsyncRead + Unpin + Send + Sync>>>;

    fn delete(&self, oid: &ObjectId) -> impl Future<Output = Result<()>>;

    fn auth_key(&self, oid: &ObjectId) -> impl Future<Output = Result<Option<Vec<u8>>>>;
}
