use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::io::AsyncRead;

use crate::{
    models::{
        object::{Object, ObjectId, Upload},
        session::Session,
    },
    repositories::{
        fs::{AuthorizedRepository, BaseFsRepository},
        Result, SESSION_AUTH_KEY_FILE, SESSION_FILE,
    },
};

use super::ObjectRepository;

pub struct ObjectFsRepository {
    dir: PathBuf,
}

impl ObjectFsRepository {
    pub fn new<D: AsRef<Path>>(dir: D) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }

    fn session_file_path(&self) -> PathBuf {
        self.dir.join(SESSION_FILE)
    }

    fn session_auth_key_path(&self) -> PathBuf {
        self.dir.join(SESSION_AUTH_KEY_FILE)
    }

    fn object_metadata_path(&self, oid: &ObjectId) -> PathBuf {
        self.dir.join(format!("{oid}.json"))
    }

    fn object_file_path(&self, oid: &ObjectId) -> PathBuf {
        self.dir.join(oid.to_string())
    }

    fn get_object(&self, oid: &ObjectId) -> Result<Object> {
        self.load(self.object_metadata_path(oid))
    }

    fn put_object(&self, obj: Object) -> Result<Object> {
        let oid = &obj.id;

        let path = self.object_metadata_path(oid);
        let file = fs::File::create_new(path)?;
        self.save(file, &obj)?;

        let mut sess = self.load_session()?;
        sess.add_object(obj.clone());
        self.save_session(&sess)?;

        Ok(obj)
    }

    fn load_session(&self) -> Result<Session> {
        let path = self.session_file_path();
        self.load(path)
    }

    fn save_session(&self, sess: &Session) -> Result<()> {
        let path = self.session_file_path();
        let file = fs::File::create(path)?;
        self.save(file, sess)
    }
}

impl ObjectRepository for ObjectFsRepository {
    async fn list(&self) -> Result<Vec<Object>> {
        let session = self.load_session()?;
        Ok(session.objects.into_iter().map(Into::into).collect())
    }

    async fn put(&self, upload: Upload) -> Result<Object> {
        let obj: Object = upload.try_into()?;
        self.put_object(obj).map(Into::into)
    }

    async fn upload<R>(&self, upload: Upload, mut reader: R) -> Result<Object>
    where
        R: AsyncRead + Send + Unpin,
    {
        let obj: Object = upload.try_into()?;
        {
            let path = self.object_file_path(&obj.id);
            let mut file = tokio::fs::File::create_new(path).await?;
            tokio::io::copy(&mut reader, &mut file).await?;
        }
        self.put_object(obj).map(Into::into)
    }

    async fn auth(&self, auth_key: &[u8]) -> Result<bool> {
        self.check_auth_key(self.session_auth_key_path(), auth_key)
    }

    async fn get(&self, oid: &ObjectId) -> Result<Object> {
        Ok(self.get_object(oid)?.into())
    }

    async fn download(&self, oid: &ObjectId) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>> {
        let path = self.object_file_path(oid);
        let file = tokio::fs::File::open(path).await?;
        Ok(Box::new(file))
    }

    async fn delete(&self, oid: &ObjectId) -> Result<()> {
        let path = self.object_file_path(oid);
        if let Err(e) = fs::remove_file(path) {
            if e.kind() != ErrorKind::NotFound {
                return Err(Box::new(e));
            }
        }
        fs::remove_file(self.object_metadata_path(oid))?;

        let mut sess = self.load_session()?;
        sess.remove_object(oid);
        self.save_session(&sess)?;

        Ok(())
    }
}

impl BaseFsRepository for ObjectFsRepository {}
impl AuthorizedRepository for ObjectFsRepository {}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use crate::repositories::session::{SessionFsRepository, SessionRepository};

    use super::*;

    #[tokio::test]
    async fn put_and_get_object() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let dir = tmpdir.path();
        let sid = SessionFsRepository::new(dir).create(None).await?.id;
        let repo = ObjectFsRepository::new(dir.join(sid.to_string()));
        let upload = Upload::default();
        let oid = repo.put(upload).await?.id;
        let obj = repo.get(&oid).await?;
        assert_eq!(obj.id, oid);
        Ok(())
    }
}
