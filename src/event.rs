use serde::{self, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientEvent {
    EnterTheRoom(Uuid),
    #[serde(deserialize_with = "deserialize_without_content")]
    LeaveTheRoom,
    StartBroadcast(String),
    StartWatch {
        local_description: String,
        broadcaster_id: Uuid,
    },
}

fn deserialize_without_content<'de, D, T>(_: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(T::default())
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    RoomSessions(Vec<Uuid>),
    NewSession(Uuid),
    BroadcastDescription(String),
    RecipientDescription {
        broadcaster_id: Uuid,
        description: String,
    },
}
