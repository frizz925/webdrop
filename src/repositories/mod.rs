pub mod fs;
pub mod object;
pub mod session;

use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

const SESSION_FILE: &str = "session.json";
const SESSION_AUTH_KEY_FILE: &str = "authkey.txt";
