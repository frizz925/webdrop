use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct UnknownKindError;

#[derive(Serialize, Deserialize)]
pub enum Kind {
    Text,
    Binary,
    Image,
    Audio,
    Video,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Self::Text => "text",
            Self::Binary => "binary",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
        }
        .to_owned()
    }
}

impl FromStr for Kind {
    type Err = UnknownKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "binary" => Ok(Self::Binary),
            "image" => Ok(Self::Image),
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            _ => Err(UnknownKindError),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub mime: String,
    pub kind: Kind,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct FileUpload {
    pub name: String,
    pub mime: String,
    pub kind: Kind,
}

impl Into<File> for FileUpload {
    fn into(self) -> File {
        File {
            id: Uuid::new_v4(),
            name: self.name,
            mime: self.mime,
            kind: self.kind,
            timestamp: Utc::now(),
        }
    }
}
