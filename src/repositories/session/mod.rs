use std::{error::Error as StdError, fmt::Display};

mod fs;

pub use fs::SessionFsRepository;

#[derive(Debug)]
pub enum Error {
    SessionNotFound,
    FileNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionNotFound => f.write_str("Session not found"),
            Self::FileNotFound => f.write_str("File not found"),
        }
    }
}

impl StdError for Error {}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}
