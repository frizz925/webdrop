use std::collections::VecDeque;

use crate::models::object::ObjectId;

use super::{object::Object, snowflake::SnowflakeId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type SessionId = SnowflakeId;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: SessionId,
    pub objects: Vec<Object>,
    pub creation_time: DateTime<Utc>,
    pub session_key: Option<String>,
    pub kdf_params: Option<Value>,
    pub auth_params: Option<Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSession {
    pub session_key: Option<String>,
    pub kdf_params: Option<Value>,
    pub auth_key: Option<String>,
    pub auth_params: Option<Value>,
}

impl Default for CreateSession {
    fn default() -> Self {
        Self {
            session_key: None,
            kdf_params: None,
            auth_key: None,
            auth_params: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionPrivate {
    pub id: SessionId,
    pub objects: VecDeque<Object>,
    pub creation_time: DateTime<Utc>,
    pub session_key: Option<String>,
    pub kdf_params: Option<Value>,
    pub auth_key: Option<String>,
    pub auth_params: Option<Value>,
}

impl SessionPrivate {
    pub fn new(sid: SessionId, req: CreateSession) -> Self {
        Self {
            id: sid,
            objects: VecDeque::default(),
            creation_time: Utc::now(),
            session_key: req.session_key,
            kdf_params: req.kdf_params,
            auth_key: req.auth_key,
            auth_params: req.auth_params,
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push_front(obj);
    }

    pub fn remove_object(&mut self, oid: &ObjectId) {
        if let Some(idx) = self.objects.iter().position(|o| &o.id == oid) {
            self.objects.remove(idx);
        }
    }
}

impl Into<Session> for SessionPrivate {
    fn into(self) -> Session {
        Session {
            id: self.id,
            objects: self.objects.into(),
            creation_time: self.creation_time,
            session_key: self.session_key,
            kdf_params: self.kdf_params,
            auth_params: self.auth_params,
        }
    }
}
