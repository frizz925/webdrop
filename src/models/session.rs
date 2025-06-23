use std::collections::VecDeque;

use crate::models::{crypto::KDFParams, object::ObjectId};

use super::{object::Object, snowflake::SnowflakeId};

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
pub struct Session {
    pub id: SessionId,
    pub objects: VecDeque<Object>,
    pub creation_time: DateTime<Utc>,
    pub crypto: Option<SessionCrypto>,
}

impl Session {
    pub fn new(sid: SessionId, req: Option<CreateSession>) -> Self {
        Self {
            id: sid,
            objects: VecDeque::default(),
            creation_time: Utc::now(),
            crypto: req.map(Into::into),
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

impl Into<SessionDto> for Session {
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

impl Into<SessionCrypto> for CreateSession {
    fn into(self) -> SessionCrypto {
        SessionCrypto {
            kdf_params: self.kdf_params,
            auth_key: self.auth_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionCryptoDto {
    pub kdf_params: KDFParams,
}

#[derive(Serialize, Deserialize)]
pub struct SessionCrypto {
    pub auth_key: String,
    pub kdf_params: KDFParams,
}

impl Into<SessionCryptoDto> for SessionCrypto {
    fn into(self) -> SessionCryptoDto {
        SessionCryptoDto {
            kdf_params: self.kdf_params,
        }
    }
}
