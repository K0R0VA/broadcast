use crate::{messages::{CloseRoom, CloseSession, CreateRoom, EnterTheRoom, NewSession}, room::Room, session::Session};
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    rooms: HashMap<String, Addr<Room>>,
    sessions: HashMap<Uuid, Addr<Session>>,
}

impl Actor for State {
    type Context = Context<Self>;
}

impl Handler<CreateRoom> for State {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, ctx: &mut Self::Context) -> Self::Result {
        let room = Room {name: msg.room_name.clone(), state: ctx.address(), sessions: Default::default()}.start();
        self.rooms.insert(msg.room_name, room);
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
        self.rooms.remove(&msg.room_name);
    }
}

impl Handler<EnterTheRoom> for State {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: EnterTheRoom, ctx: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.room_name).map(|room| room.clone())
    }
}