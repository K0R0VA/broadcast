use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ClientEvent {
    EnterTheRoom(String),
    StartBroadcast(String),
    StartWatch(String),
}

pub enum ServerEvent {
    
}