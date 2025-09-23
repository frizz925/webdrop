use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use chrono::{DateTime, Utc};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::snowflake::SnowflakeId;

pub type ObjectId = SnowflakeId;

pub struct UnknownKindError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectDto {
    pub id: ObjectId,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
    pub mime: Option<String>,
    pub auth_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Object {
    pub id: ObjectId,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
    pub mime: Option<String>,
    pub auth_key: Option<String>,
}

impl Into<ObjectDto> for Object {
    fn into(self) -> ObjectDto {
        ObjectDto {
            id: self.id,
            timestamp: self.timestamp,
            content: self.content,
            mime: self.mime,
            auth_key: self.auth_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub content: Value,
}

impl Upload {
    pub fn new(content: Value) -> Self {
        Self { content }
    }
}

impl Default for Upload {
    fn default() -> Self {
        Self {
            content: Value::Null,
        }
    }
}

impl Into<Object> for Upload {
    fn into(self) -> Object {
        let mut auth_key_raw = [0u8; 48];
        StdRng::from_os_rng().fill_bytes(&mut auth_key_raw);
        let auth_key = BASE64_STANDARD_NO_PAD.encode(auth_key_raw);
        Object {
            id: ObjectId::generate(),
            timestamp: Utc::now(),
            content: self.content,
            mime: None,
            auth_key: Some(auth_key),
        }
    }
}
