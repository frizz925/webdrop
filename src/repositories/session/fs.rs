use std::{
    ffi::OsStr,
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use base64::{prelude::BASE64_STANDARD, Engine};

use crate::{
    models::session::{Session, SessionId},
    repositories::{fs::BaseFsRepository, Result, SESSION_AUTH_KEY_FILE, SESSION_FILE},
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
    async fn list(&self) -> Result<Vec<SessionId>> {
        let mut vec = Vec::default();
        for result in fs::read_dir(&self.dir)? {
            let dir = result?;
            if let Some(name) = dir.path().file_name().and_then(OsStr::to_str) {
                if let Ok(sid) = SessionId::from_str(name) {
                    vec.push(sid);
                }
            }
        }
        Ok(vec)
    }

    async fn create(&self, sess: &Session) -> Result<()> {
        let sid = &sess.id;
        let dir = self.session_dir_path(sid);
        fs::create_dir(&dir)?;

        let path = self.session_file_path(sid);
        let file = fs::File::create_new(path)?;
        self.save(file, sess)?;

        if let Some(crypto) = sess.crypto.as_ref() {
            let path = self.session_auth_key_path(sid);
            let mut file = fs::File::create_new(path)?;
            file.write_all(crypto.auth_key.as_bytes())?;
        }

        Ok(())
    }

    async fn exists(&self, sid: &SessionId) -> Result<bool> {
        let path = self.session_file_path(&sid);
        match fs::metadata(path) {
            Ok(meta) => Ok(meta.is_file()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get(&self, sid: &SessionId) -> Result<Session> {
        let session: Session = self.load(self.session_file_path(sid))?;
        Ok(session.into())
    }

    async fn delete(&self, sid: &SessionId) -> Result<()> {
        fs::remove_dir_all(self.session_dir_path(sid))?;
        Ok(())
    }

    async fn auth_key(&self, sid: &SessionId) -> Result<Option<Vec<u8>>> {
        let key_path = self.session_auth_key_path(sid);
        let key = if fs::exists(&key_path)? {
            self.read_string(self.session_auth_key_path(sid))?
        } else {
            return Ok(None);
        };
        let raw = BASE64_STANDARD.decode(key)?;
        Ok(Some(raw))
    }
}

impl BaseFsRepository for SessionFsRepository {}

#[cfg(test)]
mod tests {
    use temp_dir::TempDir;

    use super::*;

    #[tokio::test]
    async fn create_and_get_session() -> Result<()> {
        let tmpdir = TempDir::new()?;
        let repo = SessionFsRepository::new(tmpdir.path());
        let sess = Session::default();
        let sid = &sess.id;
        repo.create(&sess).await?;
        assert!(repo.exists(sid).await?);
        let sess2 = repo.get(sid).await?;
        assert_eq!(sess2, sess);
        Ok(())
    }
}
