use actix::{Addr, Message};
use recipient::Recipient;
use uuid::Uuid;

use crate::{recipient, room::Room, session::Session};

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
    pub recipient_id: Uuid,
    pub recipient: Recipient,
    pub recipient_addr: Addr<Session>
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

pub struct GetRoomName {
    pub room_id: Uuid,
}

impl Message for GetRoomName {
    type Result = Option<String>;
}

pub struct NewUserInRoom(pub Uuid);

impl Message for NewUserInRoom {
    type Result = ();
}

pub struct RoomSessions(pub Vec<Uuid>);

impl Message for RoomSessions {
    type Result = ();
}

pub struct StartWatch {
    pub recipient: Addr<Session>,
    pub local_description: String,
    pub broadcaster_id: Uuid,
    pub recipient_id: Uuid,
}

impl Message for StartWatch {
    type Result = ();
}