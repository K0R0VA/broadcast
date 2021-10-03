use actix::{Addr, MailboxError, Message};
use uuid::Uuid;

use crate::{broadcaster::Broadcaster, room::Room, session::Session};

pub struct CreateRoom {
    pub room_name: String,
}

impl Message for CreateRoom {
    type Result = ();
}

pub struct NewSession {
    pub id: Uuid,
    pub session: Addr<Session>
}

impl Message for NewSession {
    type Result = ();
}

pub struct CloseSession (pub Uuid);

impl Message for CloseSession {
    type Result = ();
}

pub struct CloseRoom {
    pub room_name: String,
}

impl Message for CloseRoom {
    type Result = ();
}

pub struct EnterTheRoom {
    pub room_name: String,
}

impl Message for EnterTheRoom {
    type Result = Option<Addr<Room>>;
}