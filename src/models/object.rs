use std::time::SystemTimeError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::snowflake::SnowflakeId;

pub type ObjectId = SnowflakeId;

pub struct UnknownKindError;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentKind {
    File,
}

pub trait Content: Serialize {}

#[derive(Serialize, Deserialize)]
pub struct FileContent {
    pub kind: ContentKind,
    pub name: String,
}

impl FileContent {
    pub fn new(name: String) -> Self {
        Self {
            kind: ContentKind::File,
            name,
        }
    }
}

impl Content for FileContent {}

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

impl ObjectDao {
    pub fn get_file_name(&self) -> Option<String> {
        let json = self.content.to_owned();
        if let Ok(content) = serde_json::from_value::<FileContent>(json) {
            match content.kind {
                ContentKind::File => return Some(content.name),
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub mime: String,
    pub content: Value,
    pub crypto: Option<ObjectCryptoDto>,
}

impl Upload {
    pub fn new<C: Content>(mime: String, content: C) -> Self {
        Self {
            mime,
            content: serde_json::to_value(content).unwrap(),
            crypto: None,
        }
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
