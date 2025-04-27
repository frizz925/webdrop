use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub enum EventName {
    ObjectCreated,
    ObjectDeleted,
    SessionDeleted,
}

impl EventName {
    pub fn into_event(self) -> Event {
        Event::new(self, Value::Null)
    }
}

impl Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ObjectCreated => "object.created",
            Self::ObjectDeleted => "object.deleted",
            Self::SessionDeleted => "session.deleted",
        };
        f.write_str(s)
    }
}

impl Serialize for EventName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Serialize, Clone)]
pub struct Event {
    pub name: EventName,
    pub data: Value,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    pub fn new<T: Serialize>(name: EventName, data: T) -> Self {
        Self {
            name,
            data: serde_json::to_value(data).unwrap(),
            timestamp: Utc::now(),
        }
    }
}
