use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::io::AsyncRead;

use crate::{
    models::{
        object::{ObjectDao, ObjectId, Upload},
        session::SessionDao,
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

    fn get_object(&self, oid: &ObjectId) -> Result<ObjectDao> {
        self.load(self.object_metadata_path(oid))
    }

    fn put_object(&self, obj: ObjectDao) -> Result<ObjectDao> {
        let oid = &obj.id;

        let path = self.object_metadata_path(oid);
        let file = fs::File::create_new(path)?;
        self.save(file, &obj)?;

        let mut sess = self.load_session()?;
        sess.add_object(obj.clone());
        self.save_session(&sess)?;

        Ok(obj)
    }

    fn load_session(&self) -> Result<SessionDao> {
        let path = self.session_file_path();
        self.load(path)
    }

    fn save_session(&self, sess: &SessionDao) -> Result<()> {
        let path = self.session_file_path();
        let file = fs::File::create(path)?;
        self.save(file, sess)
    }
}

impl ObjectRepository for ObjectFsRepository {
    async fn list(&self) -> Result<Vec<ObjectDao>> {
        let session = self.load_session()?;
        Ok(session.objects.into())
    }

    async fn get(&self, oid: &ObjectId) -> Result<ObjectDao> {
        self.get_object(oid)
    }

    async fn put(&self, upload: Upload) -> Result<ObjectDao> {
        let obj: ObjectDao = upload.try_into()?;
        self.put_object(obj)
    }

    async fn upload<R>(&self, upload: Upload, mut reader: R) -> Result<ObjectDao>
    where
        R: AsyncRead + Send + Unpin,
    {
        let obj: ObjectDao = upload.try_into()?;
        {
            let path = self.object_file_path(&obj.id);
            let mut file = tokio::fs::File::create_new(path).await?;
            tokio::io::copy(&mut reader, &mut file).await?;
        }
        self.put_object(obj)
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

    async fn auth(&self, auth_key: &[u8]) -> Result<bool> {
        self.check_auth_key(self.session_auth_key_path(), auth_key)
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
