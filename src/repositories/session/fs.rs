use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::{
    fs,
    io::{copy, AsyncRead, AsyncReadExt, AsyncWriteExt},
};
use uuid::Uuid;

use crate::{
    models::{
        file::{File, FileUpload},
        session::{Session, SessionId},
    },
    repositories::{Result, SessionRepository},
};

use super::Error;

const SESSION_FILE: &str = "session.json";

pub struct SessionFsRepository {
    dir: PathBuf,
}

impl SessionFsRepository {
    pub fn new<D: AsRef<Path>>(dir: D) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }

    async fn update(&self, sess: &Session) -> Result<()> {
        let dir = self.dir.join(sess.id.to_string());
        if !fs::try_exists(&dir).await? {
            fs::create_dir(&dir).await?;
        }

        let path = dir.join(SESSION_FILE);
        let file = fs::File::create(path).await?;
        self.save(file, &sess).await?;

        Ok(())
    }

    async fn save(&self, mut file: fs::File, sess: &Session) -> Result<()> {
        let json = serde_json::to_string(sess)?;
        file.write_all(json.as_bytes()).await?;
        Ok(())
    }
}

impl SessionRepository for SessionFsRepository {
    async fn create(&self) -> Result<Session> {
        let sid = SessionId::generate()?;
        let dir = self.dir.join(sid.to_string());
        fs::create_dir(&dir).await?;

        let sess = Session::new(sid);
        let path = dir.join(SESSION_FILE);
        let file = fs::File::create_new(path).await?;
        self.save(file, &sess).await?;

        Ok(sess)
    }

    async fn get(&self, sid: SessionId) -> Result<Option<Session>> {
        let path = self.dir.join(sid.to_string()).join(SESSION_FILE);
        match fs::File::open(path).await {
            Ok(mut file) => {
                let mut s = String::new();
                file.read_to_string(&mut s).await?;
                let sess = serde_json::from_str(&s)?;
                Ok(Some(sess))
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn upload<R: AsyncRead + Send + Unpin + 'static>(
        &self,
        sid: SessionId,
        file: FileUpload,
        mut reader: R,
    ) -> Result<()> {
        let dir = self.dir.join(sid.to_string());
        if let Some(mut sess) = self.get(sid).await? {
            let meta: File = file.into();
            let fid = meta.id;

            let path = dir.join(fid.to_string());
            let mut file = fs::File::create_new(path).await?;
            copy(&mut reader, &mut file).await?;
            drop(file);

            let path = dir.join(format!("{fid}.json"));
            let mut file = fs::File::create_new(path).await?;
            let json = serde_json::to_string(&meta)?;
            file.write_all(json.as_bytes()).await?;
            drop(file);

            sess.files.push(meta);
            self.update(&sess).await?;
            Ok(())
        } else {
            Err(Box::new(Error::SessionNotFound))
        }
    }

    async fn download(
        &self,
        sid: SessionId,
        fid: Uuid,
    ) -> Result<impl AsyncRead + Send + Unpin + 'static> {
        let path = self.dir.join(sid.to_string()).join(fid.to_string());
        let file = fs::File::open(path).await?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use super::*;

    const STORAGE_DIR: Option<&str> = Some("storage");

    #[tokio::test]
    async fn create_and_get_session() -> Result<()> {
        let repo = if let Some(dir) = STORAGE_DIR {
            SessionFsRepository::new(dir)
        } else {
            let tmpdir = TempDir::new()?;
            SessionFsRepository::new(tmpdir.path())
        };
        let sid = repo.create().await?.id;
        if let Some(sess) = repo.get(sid).await? {
            assert_eq!(sess.id, sid);
        } else {
            panic!("Session must exist");
        }
        Ok(())
    }
}
