use std::{
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use crate::{
    models::session::{CreateSession, SessionDao, SessionDto, SessionId},
    repositories::{
        fs::{AuthorizedRepository, BaseFsRepository},
        Result, SESSION_AUTH_KEY_FILE, SESSION_FILE,
    },
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

    fn session_auth_key_path(&self, sid: &SessionId) -> PathBuf {
        self.session_dir_path(sid).join(SESSION_AUTH_KEY_FILE)
    }
}

impl SessionRepository for SessionFsRepository {
    async fn exists(&self, sid: &SessionId) -> Result<bool> {
        let path = self.session_file_path(&sid);
        match fs::metadata(path) {
            Ok(meta) => Ok(meta.is_file()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get(&self, sid: &SessionId) -> Result<SessionDto> {
        let session: SessionDao = self.load(self.session_file_path(sid))?;
        Ok(session.into())
    }

    async fn create(&self, req: Option<CreateSession>) -> Result<SessionDto> {
        let sid = SessionId::generate()?;
        let dir = self.session_dir_path(&sid);
        fs::create_dir(&dir)?;

        let sess = SessionDao::new(sid, req);
        let path = self.session_file_path(&sid);
        let file = fs::File::create_new(path)?;
        self.save(file, &sess)?;

        if let Some(crypto) = sess.crypto.as_ref() {
            let path = self.session_auth_key_path(&sid);
            let mut file = fs::File::create_new(path)?;
            file.write_all(crypto.auth_key.as_bytes())?;
        }

        Ok(sess.into())
    }

    async fn delete(&self, sid: &SessionId) -> Result<()> {
        fs::remove_dir_all(self.session_dir_path(sid))?;
        Ok(())
    }

    async fn auth(&self, sid: &SessionId, auth_key: &[u8]) -> Result<bool> {
        self.check_auth_key(self.session_auth_key_path(sid), auth_key)
    }
}

impl BaseFsRepository for SessionFsRepository {}
impl AuthorizedRepository for SessionFsRepository {}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use super::*;

    #[tokio::test]
    async fn create_and_get_session() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let repo = SessionFsRepository::new(tmpdir.path());
        let sid = repo.create(None).await?.id;
        assert!(repo.exists(&sid).await?);
        let sess = repo.get(&sid).await?;
        assert_eq!(sess.id, sid);
        Ok(())
    }
}
