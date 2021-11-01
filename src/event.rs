use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ClientEvent {
    EnterTheRoom(Uuid),
    StartBroadcast(String),
    StartWatch(String),
}

#[derive(Serialize, Deserialize)]
pub enum ServerEvent {
    BroadcastDescription(String),
    RecipientDescription(String)
}