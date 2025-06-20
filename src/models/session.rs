use std::collections::VecDeque;

use crate::models::{crypto::KDFParams, object::ObjectId};

use super::{object::ObjectDao, snowflake::SnowflakeId};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type SessionId = SnowflakeId;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
    pub id: SessionId,
    pub creation_time: DateTime<Utc>,
    pub crypto: Option<SessionCryptoDto>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionDao {
    pub id: SessionId,
    pub objects: VecDeque<ObjectDao>,
    pub creation_time: DateTime<Utc>,
    pub crypto: Option<SessionCryptoDao>,
}

impl SessionDao {
    pub fn new(sid: SessionId, req: Option<CreateSession>) -> Self {
        Self {
            id: sid,
            objects: VecDeque::default(),
            creation_time: Utc::now(),
            crypto: req.map(Into::into),
        }
    }

    pub fn add_object(&mut self, obj: ObjectDao) {
        self.objects.push_front(obj);
    }

    pub fn remove_object(&mut self, oid: &ObjectId) {
        if let Some(idx) = self.objects.iter().position(|o| &o.id == oid) {
            self.objects.remove(idx);
        }
    }
}

impl Into<SessionDto> for SessionDao {
    fn into(self) -> SessionDto {
        SessionDto {
            id: self.id,
            creation_time: self.creation_time,
            crypto: self.crypto.map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSession {
    pub auth_key: String,
    pub kdf_params: KDFParams,
}

#[derive(Serialize, Deserialize)]
pub struct SessionCryptoDao {
    pub auth_key: String,
    pub kdf_params: KDFParams,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionCryptoDto {
    pub kdf_params: KDFParams,
}

impl Into<SessionCryptoDao> for CreateSession {
    fn into(self) -> SessionCryptoDao {
        SessionCryptoDao {
            kdf_params: self.kdf_params,
            auth_key: self.auth_key,
        }
    }
}

impl Into<SessionCryptoDto> for SessionCryptoDao {
    fn into(self) -> SessionCryptoDto {
        SessionCryptoDto {
            kdf_params: self.kdf_params,
        }
    }
}
