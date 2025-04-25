use std::fmt::Display;

use serde::Serialize;

use super::object::ObjectId;

#[derive(Serialize)]
struct ObjectEvent {
    name: String,
    #[serde(rename = "objectID")]
    object_id: ObjectId,
}

impl ObjectEvent {
    fn new<'a>(event: &'a Event, oid: &'a ObjectId) -> Self {
        Self {
            name: event.name().to_owned(),
            object_id: oid.to_owned(),
        }
    }
}

#[derive(Serialize)]
struct GenericEvent {
    name: String,
}

impl GenericEvent {
    fn new(event: &Event) -> Self {
        Self {
            name: event.name().to_owned(),
        }
    }
}

#[derive(Clone)]
pub enum Event {
    ObjectCreated(ObjectId),
    ObjectDeleted(ObjectId),
    SessionDeleted,
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ObjectCreated(_) => "object.created",
            Self::ObjectDeleted(_) => "object.deleted",
            Self::SessionDeleted => "session.deleted",
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::ObjectCreated(oid) => ObjectEvent::new(self, oid).serialize(serializer),
            Self::ObjectDeleted(oid) => ObjectEvent::new(self, oid).serialize(serializer),
            other => GenericEvent::new(other).serialize(serializer),
        }
    }
}
