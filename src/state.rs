use crate::{
    messages::{CloseRoom, CloseSession, CreateRoom, EnterTheRoom, GetRoomName, NewSession},
    room::Room,
    session::Session,
};
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use std::{collections::HashMap};
use uuid::Uuid;

#[derive(Default)]
pub struct State {
    rooms: HashMap<Uuid, (String, Addr<Room>)>,
    sessions: HashMap<Uuid, Addr<Session>>,
}

impl Actor for State {
    type Context = Context<Self>;
}

impl Handler<CreateRoom> for State {
    type Result = Option<Uuid>;

    fn handle(&mut self, msg: CreateRoom, ctx: &mut Self::Context) -> Self::Result {
        let id: Uuid = Uuid::new_v4();
        let room = Room {
            uuid: id,
            state: ctx.address(),
            sessions: Default::default(),
        }
        .start();
        self.rooms.insert(id, (msg.room_name, room));
        Some(id)
    }
}

impl Handler<NewSession> for State {
    type Result = ();

    fn handle(&mut self, msg: NewSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.id, msg.session);
    }
}

impl Handler<CloseSession> for State {
    type Result = ();

    fn handle(&mut self, msg: CloseSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.0);
    }
}

impl Handler<CloseRoom> for State {
    type Result = ();

    fn handle(&mut self, msg: CloseRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.remove(&msg.room_id);
    }
}

impl Handler<EnterTheRoom> for State {
    type Result = Option<(String, Addr<Room>)>;

    fn handle(&mut self, msg: EnterTheRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.room_id).map(|room| room.clone())
    }
}

impl Handler<GetRoomName> for State {
    type Result = Option<String>;

    fn handle(&mut self, msg: GetRoomName, _: &mut Self::Context) -> Self::Result {
        self.rooms
            .get(&msg.room_id)
            .map(|(room_name, _)| room_name.clone())
    }
}
