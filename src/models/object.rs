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
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub content: Value,
    pub generate_auth_key: Option<bool>,
}

impl Upload {
    pub fn new(content: Value, generate_auth_key: bool) -> Self {
        Self {
            content,
            generate_auth_key: Some(generate_auth_key),
        }
    }
}

impl Default for Upload {
    fn default() -> Self {
        Self {
            content: Value::Null,
            generate_auth_key: None,
        }
    }
}

impl Into<Object> for Upload {
    fn into(self) -> Object {
        let auth_key = if self.generate_auth_key.unwrap_or_default() {
            let mut buf = [0u8; 48];
            StdRng::from_os_rng().fill_bytes(&mut buf);
            let encoded = BASE64_STANDARD_NO_PAD.encode(buf);
            Some(encoded)
        } else {
            None
        };
        Object {
            id: ObjectId::generate(),
            timestamp: Utc::now(),
            content: self.content,
            mime: None,
            auth_key,
        }
    }
}
