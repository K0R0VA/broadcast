use crate::{
    messages::{
        CloseRoom, CloseSession, CreateRoom, EnterTheRoom, GetRoomInfo, NewSession, RoomInfo,
    },
    room::Room,
    session::Session,
};
use actix::{Actor, Addr, AsyncContext, Context, Handler, ResponseFuture};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct State {
    rooms: HashMap<Uuid, Addr<Room>>,
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
            name: msg.room_name,
            uuid: id,
            state: ctx.address(),
            sessions: Default::default(),
        }
        .start();
        self.rooms.insert(id, room);
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
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: EnterTheRoom, _: &mut Self::Context) -> Self::Result {
        self.rooms.get(&msg.room_id).cloned()
    }
}

impl Handler<GetRoomInfo> for State {
    type Result = ResponseFuture<Option<RoomInfo>>;

    fn handle(&mut self, msg: GetRoomInfo, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.rooms.get(&msg.room_id).cloned() {
            return Box::pin(async move { 
                if let Ok(room) = addr.send(msg).await {
                    return room;
                } 
                None
            })
        }
        Box::pin(async { None })
    }
}
