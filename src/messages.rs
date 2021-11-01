use actix::{Addr, Message};
use broadcast_context::Recipient;
use uuid::Uuid;

use crate::{room::Room, session::Session};

pub struct CreateRoom {
    pub room_name: String,
}

impl Message for CreateRoom {
    type Result = Option<Uuid>;
}

pub struct NewSession {
    pub id: Uuid,
    pub session: Addr<Session>,
}

impl Message for NewSession {
    type Result = ();
}

pub struct CloseSession(pub Uuid);

impl Message for CloseSession {
    type Result = ();
}

pub struct CloseRoom {
    pub room_id: Uuid,
}

impl Message for CloseRoom {
    type Result = ();
}

pub struct EnterTheRoom {
    pub room_id: Uuid,
}

impl Message for EnterTheRoom {
    type Result = Option<(String, Addr<Room>)>;
}

pub struct NewRecipient {
    pub recipient: Recipient,
}

impl Message for NewRecipient {
    type Result = ();
}

pub struct RecipientResponse {
    pub description: String,
}

impl Message for RecipientResponse {
    type Result = ();
}
