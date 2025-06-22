use std::time::SystemTimeError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::snowflake::SnowflakeId;

pub type ObjectId = SnowflakeId;

pub struct UnknownKindError;

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectDao {
    pub id: ObjectId,
    pub mime: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectDto {
    pub id: ObjectId,
    pub mime: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub mime: String,
    pub content: Value,
}

impl Upload {
    pub fn new(mime: String, content: Value) -> Self {
        Self { mime, content }
    }
}

impl TryInto<ObjectDao> for Upload {
    type Error = SystemTimeError;

    fn try_into(self) -> Result<ObjectDao, Self::Error> {
        Ok(ObjectDao {
            id: ObjectId::generate()?,
            mime: self.mime,
            timestamp: Utc::now(),
            content: self.content,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectCryptoDao {
    subkey: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectCryptoDto {
    subkey: String,
}

impl Into<ObjectCryptoDao> for ObjectCryptoDto {
    fn into(self) -> ObjectCryptoDao {
        ObjectCryptoDao {
            subkey: self.subkey,
        }
    }
}
