use std::{
    fmt::Display,
    num::ParseIntError,
    str::FromStr,
    time::{Duration, SystemTimeError, UNIX_EPOCH},
};

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

const SESSION_EPOCH_SECONDS: u64 = 1717340400;

type SnowflakeBytes = [u8; 8];

// First 48-bit is timestamp, then the rest is random bytes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SnowflakeId(SnowflakeBytes);

impl SnowflakeId {
    pub fn generate() -> Result<Self, SystemTimeError> {
        let mut sid = Self::default();
        let session_epoch = UNIX_EPOCH
            .checked_add(Duration::from_secs(SESSION_EPOCH_SECONDS))
            .unwrap();
        let timestamp = (session_epoch.elapsed()?.as_millis() as u64).to_be_bytes();
        sid.0[..6].copy_from_slice(&timestamp[2..]);
        let mut rng = SmallRng::from_os_rng();
        rng.fill_bytes(&mut sid.0[6..]);
        Ok(sid)
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value.to_be_bytes())
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_be_bytes(self.0)
    }
}

impl Default for SnowflakeId {
    fn default() -> Self {
        Self(SnowflakeBytes::default())
    }
}

impl From<u64> for SnowflakeId {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl Into<u64> for SnowflakeId {
    fn into(self) -> u64 {
        self.as_u64()
    }
}

impl FromStr for SnowflakeId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_u64(s.parse()?))
    }
}

impl Display for SnowflakeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_u64().to_string().as_str())
    }
}

impl Serialize for SnowflakeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_u64().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SnowflakeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(Self::from_u64(value))
    }
}
