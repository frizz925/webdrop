use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use base64::{prelude::BASE64_STANDARD, Engine};
use tokio::io::AsyncRead;

use crate::{
    models::{
        object::{Object, ObjectId},
        session::Session,
    },
    repositories::{fs::BaseFsRepository, Result, SESSION_AUTH_KEY_FILE, SESSION_FILE},
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

    fn put_object(&self, obj: &Object) -> Result<()> {
        let oid = &obj.id;

        let path = self.object_metadata_path(oid);
        let file = fs::File::create_new(path)?;
        self.save(file, &obj)?;

        let mut sess = self.load_session()?;
        sess.add_object(obj.clone());
        self.save_session(&sess)?;

        Ok(())
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

    async fn put(&self, obj: &Object) -> Result<()> {
        self.put_object(obj)
    }

    async fn upload<R>(&self, obj: &Object, mut reader: R) -> Result<()>
    where
        R: AsyncRead + Send + Unpin,
    {
        {
            let path = self.object_file_path(&obj.id);
            let mut file = tokio::fs::File::create_new(path).await?;
            tokio::io::copy(&mut reader, &mut file).await?;
        }
        self.put_object(obj).map(Into::into)
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

    async fn auth_key(&self, oid: &ObjectId) -> Result<Option<Vec<u8>>> {
        let obj = self.get_object(oid)?;
        let key = if let Some(auth_key) = obj.auth_key {
            auth_key
        } else {
            let key_path = self.session_auth_key_path();
            if fs::exists(&key_path)? {
                self.read_string(key_path)?
            } else {
                return Ok(None);
            }
        };
        let buf = BASE64_STANDARD.decode(key)?;
        Ok(Some(buf))
    }
}

impl BaseFsRepository for ObjectFsRepository {}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use crate::{
        models::object::Upload,
        repositories::session::{SessionFsRepository, SessionRepository},
    };

    use super::*;

    #[tokio::test]
    async fn put_and_get_object() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let dir = tmpdir.path();

        let sid = {
            let sess = Session::default();
            SessionFsRepository::new(dir).create(&sess).await?;
            sess.id
        };

        let repo = ObjectFsRepository::new(dir.join(sid.to_string()));
        let obj = Upload::default().try_into()?;
        repo.put(&obj).await?;

        let obj2 = repo.get(&obj.id).await?;
        assert_eq!(obj2, obj);
        Ok(())
    }
}
