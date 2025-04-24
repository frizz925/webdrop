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
        object::{Object, ObjectId, ObjectResult, Upload},
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

    async fn put_object(&self, obj: Object) -> Result<ObjectResult> {
        let oid = &obj.id;

        let path = self.object_metadata_path(oid);
        let mut file = fs::File::create_new(path).await?;
        let json = serde_json::to_string(&obj)?;
        file.write_all(json.as_bytes()).await?;

        let result = ObjectResult::from_object(&obj, None);
        let mut sess = self.load_session().await?;
        sess.objects.push(obj);
        self.save_session(&sess).await?;

        Ok(result)
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
    async fn put(&self, upload: Upload) -> Result<ObjectResult> {
        let obj: Object = upload.try_into()?;
        self.put_object(obj).await
    }

    async fn upload<R>(&self, upload: Upload, mut reader: R) -> Result<ObjectResult>
    where
        R: AsyncRead + Send + Unpin + 'static,
    {
        let obj: Object = upload.try_into()?;
        {
            let path = self.object_file_path(&obj.id);
            let mut file = fs::File::create_new(path).await?;
            copy(&mut reader, &mut file).await?;
        }
        self.put_object(obj).await
    }

    async fn get(&self, oid: &ObjectId) -> Result<ObjectResult> {
        let obj = self.get_object(oid).await?;
        Ok(ObjectResult::from_object(&obj, None))
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
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use temp_dir::TempDir;

    use super::*;

    const STORAGE_DIR: Option<&str> = Some("storage/test");

    #[tokio::test]
    async fn put_and_get_object() -> Result<()> {
        let repo = if let Some(dir) = STORAGE_DIR {
            ObjectFsRepository::new(dir)
        } else {
            let tmpdir = TempDir::new()?;
            ObjectFsRepository::new(tmpdir.path())
        };
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
