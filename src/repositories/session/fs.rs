use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    models::session::{Session, SessionId},
    repositories::{Result, SESSION_FILE},
};

use super::SessionRepository;

pub struct SessionFsRepository {
    dir: PathBuf,
}

impl SessionFsRepository {
    pub fn new<D: AsRef<Path>>(dir: D) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }

    fn session_dir_path(&self, sid: &SessionId) -> PathBuf {
        self.dir.join(sid.to_string())
    }

    fn session_file_path(&self, sid: &SessionId) -> PathBuf {
        self.session_dir_path(sid).join(SESSION_FILE)
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
        let dir = self.session_dir_path(&sid);
        fs::create_dir(&dir).await?;

        let sess = Session::new(sid);
        let path = self.session_file_path(&sid);
        let file = fs::File::create_new(path).await?;
        self.save(file, &sess).await?;

        Ok(sess)
    }

    async fn upsert(&self, sess: Session) -> Result<()> {
        let dir = self.session_dir_path(&sess.id);
        if !fs::try_exists(&dir).await? {
            fs::create_dir(&dir).await?;
        }

        let path = self.session_file_path(&sess.id);
        let file = fs::File::create(path).await?;
        self.save(file, &sess).await?;

        Ok(())
    }

    async fn get(&self, sid: &SessionId) -> Result<Session> {
        let path = self.session_file_path(&sid);
        let mut file = fs::File::open(path).await?;
        let mut s = String::new();
        file.read_to_string(&mut s).await?;
        Ok(serde_json::from_str(&s)?)
    }

    async fn exists(&self, sid: &SessionId) -> Result<bool> {
        let path = self.session_file_path(&sid);
        match fs::metadata(path).await {
            Ok(meta) => Ok(meta.is_file()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete(&self, sid: &SessionId) -> Result<()> {
        fs::remove_dir_all(self.session_dir_path(sid)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use super::*;

    #[tokio::test]
    async fn create_and_get_session() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let repo = SessionFsRepository::new(tmpdir.path());
        let sid = repo.create().await?.id;
        assert!(repo.exists(&sid).await?);
        let sess = repo.get(&sid).await?;
        assert_eq!(sess.id, sid);
        Ok(())
    }
}
