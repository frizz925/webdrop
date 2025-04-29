use std::{
    io::{Error as IoError, ErrorKind},
    path::{Path, PathBuf},
};

use tokio::{
    fs,
    io::{copy, AsyncRead, AsyncReadExt, AsyncWriteExt},
};

use crate::{
    models::{
        object::{Object, ObjectId, Upload},
        session::Session,
    },
    repositories::{Result, SESSION_FILE},
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

    fn object_metadata_path(&self, oid: &ObjectId) -> PathBuf {
        self.dir.join(format!("{oid}.json"))
    }

    fn object_file_path(&self, oid: &ObjectId) -> PathBuf {
        self.dir.join(oid.to_string())
    }

    async fn get_object(&self, oid: &ObjectId) -> Result<Object> {
        let path = self.object_metadata_path(oid);
        let mut file = fs::File::open(path).await?;
        let mut json = String::new();
        file.read_to_string(&mut json).await?;
        let obj = serde_json::from_str(&json)?;
        Ok(obj)
    }

    async fn put_object(&self, obj: Object) -> Result<Object> {
        let oid = &obj.id;

        let path = self.object_metadata_path(oid);
        let mut file = fs::File::create_new(path).await?;
        let json = serde_json::to_string(&obj)?;
        file.write_all(json.as_bytes()).await?;

        let mut sess = self.load_session().await?;
        sess.objects.push(obj.clone());
        sess.objects.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        self.save_session(&sess).await?;

        Ok(obj)
    }

    async fn load_session(&self) -> Result<Session> {
        let path = self.session_file_path();
        let mut file = fs::File::open(path).await?;
        let mut json = String::new();
        file.read_to_string(&mut json).await?;
        let sess = serde_json::from_str(&json)?;
        Ok(sess)
    }

    async fn save_session(&self, sess: &Session) -> Result<()> {
        let json = serde_json::to_string(sess)?;
        let path = self.session_file_path();
        let mut file = fs::File::create(path).await?;
        file.write_all(json.as_bytes()).await?;
        Ok(())
    }
}

impl ObjectRepository for ObjectFsRepository {
    async fn put(&self, upload: Upload) -> Result<Object> {
        let obj: Object = upload.try_into()?;
        self.put_object(obj).await
    }

    async fn upload<R>(&self, upload: Upload, mut reader: R) -> Result<Object>
    where
        R: AsyncRead + Send + Unpin,
    {
        let obj: Object = upload.try_into()?;
        {
            let path = self.object_file_path(&obj.id);
            let mut file = fs::File::create_new(path).await?;
            copy(&mut reader, &mut file).await?;
        }
        self.put_object(obj).await
    }

    async fn get(&self, oid: &ObjectId) -> Result<Object> {
        Ok(self.get_object(oid).await?)
    }

    async fn download(
        &self,
        oid: &ObjectId,
        name: &str,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send + Sync>> {
        let obj = self.get_object(oid).await?;
        if obj.get_file_name().filter(|s| s == name).is_none() {
            let err = IoError::from(ErrorKind::NotFound);
            return Err(Box::new(err));
        }
        let path = self.object_file_path(oid);
        let file = fs::File::open(path).await?;
        Ok(Box::new(file))
    }

    async fn delete(&self, oid: &ObjectId) -> Result<()> {
        let path = self.object_file_path(oid);
        if let Err(e) = fs::remove_file(path).await {
            if e.kind() != ErrorKind::NotFound {
                return Err(Box::new(e));
            }
        }
        fs::remove_file(self.object_metadata_path(oid)).await?;

        let mut sess = self.load_session().await?;
        sess.objects = sess.objects.into_iter().filter(|o| &o.id != oid).collect();
        self.save_session(&sess).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use temp_dir::TempDir;

    use super::*;

    #[tokio::test]
    async fn put_and_get_object() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let repo = ObjectFsRepository::new(tmpdir.path());
        let upload = Upload {
            mime: "text/plain".to_owned(),
            content: Value::Null,
        };
        let oid = repo.put(upload).await?.id;
        let obj = repo.get(&oid).await?;
        assert_eq!(obj.id, oid);
        Ok(())
    }
}
