use super::{object::Object, snowflake::SnowflakeId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type SessionId = SnowflakeId;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub objects: Vec<Object>,
    pub creation_time: DateTime<Utc>,
}

impl Session {
    pub fn new(sid: SessionId) -> Self {
        Self {
            id: sid,
            objects: Vec::default(),
            creation_time: Utc::now(),
        }
    }
}
