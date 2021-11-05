use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientEvent {
    EnterTheRoom(Uuid),
    LeaveTheRoom,
    StartBroadcast(String),
    StartWatch { local_description: String, broadcaster_id: Uuid },
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
