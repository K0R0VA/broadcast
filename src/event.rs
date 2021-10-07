use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ClientEvent {
    EnterTheRoom(String),
    StartBroadcast(String),
    StartWatch(String),
}

#[derive(Serialize, Deserialize)]
pub enum ServerEvent {
    BroadcastDescription(String),
    RecipientDescription(String)
}