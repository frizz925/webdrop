use std::{str::FromStr, time::SystemTimeError};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::snowflake::SnowflakeId;

pub type ObjectId = SnowflakeId;

struct UnknownKindError;

#[derive(Deserialize)]
enum Kind {
    File,
}

impl FromStr for Kind {
    type Err = UnknownKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file" => Ok(Self::File),
            _ => Err(UnknownKindError),
        }
    }
}

#[derive(Deserialize)]
struct FileContent {
    kind: Kind,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub mime: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
}

impl Object {
    pub fn get_file_name(&self) -> Option<String> {
        let json = self.content.to_owned();
        if let Ok(content) = serde_json::from_value::<FileContent>(json) {
            match content.kind {
                Kind::File => return Some(content.name),
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectResult {
    pub id: ObjectId,
    pub mime: String,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
}

impl ObjectResult {
    pub fn from_object(obj: &Object) -> Self {
        Self {
            id: obj.id,
            mime: obj.mime.clone(),
            timestamp: obj.timestamp,
            content: obj.content.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub mime: String,
    pub content: Value,
}

impl TryInto<Object> for Upload {
    type Error = SystemTimeError;

    fn try_into(self) -> Result<Object, Self::Error> {
        Ok(Object {
            id: ObjectId::generate()?,
            mime: self.mime,
            timestamp: Utc::now(),
            content: self.content,
        })
    }
}
