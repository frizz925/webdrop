use std::time::SystemTimeError;

use chrono::{DateTime, Utc};
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
    pub mime: Option<String>,
}

impl Into<ObjectDto> for Object {
    fn into(self) -> ObjectDto {
        ObjectDto {
            id: self.id,
            timestamp: self.timestamp,
            content: self.content,
            mime: self.mime,
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

impl TryInto<Object> for Upload {
    type Error = SystemTimeError;

    fn try_into(self) -> Result<Object, Self::Error> {
        Ok(Object {
            id: ObjectId::generate()?,
            timestamp: Utc::now(),
            content: self.content,
            mime: None,
        })
    }
}
