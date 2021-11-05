use std::collections::HashMap;

use actix::{Actor, ActorContext, ActorFutureExt, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use uuid::Uuid;

use crate::{messages::{CloseRoom, CloseSession, NewSession, NewUserInRoom, RoomSessions, StartWatch}, session::Session, state::State};

pub struct Room {
    pub uuid: Uuid,
    pub sessions: HashMap<Uuid, Addr<Session>>,
    pub state: Addr<State>,
}

impl Actor for Room {
    type Context = Context<Self>;
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        self.state
            .send(CloseRoom {
                room_id: self.uuid,
            })
            .into_actor(self)
            .then(|_, _, _| actix::fut::ready(()))
            .wait(ctx);
        actix::Running::Stop
    }
}

impl Handler<NewSession> for Room {
    type Result = ();

    fn handle(&mut self, msg: NewSession, _: &mut Self::Context) -> Self::Result {
        let sessions = self.sessions.iter().map(|(key, addr)| {
            addr.do_send(NewUserInRoom(msg.id));
            *key
        }).collect();
        msg.session.do_send(RoomSessions(sessions));
        self.sessions.insert(msg.id, msg.session);
    }
}


impl Handler<CloseSession> for Room {
    type Result = ();

    fn handle(&mut self, msg: CloseSession, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.0);
        self.sessions.is_empty().then(|| ctx.stop());
    }
}

impl Handler<StartWatch> for Room {
    type Result = ();

    fn handle(&mut self, msg: StartWatch, ctx: &mut Self::Context) -> Self::Result {
        if let Some(session) = self.sessions.get(&msg.broadcaster_id) {
            session.send(msg)
                .into_actor(self)
                .then(|_, _, _| actix::fut::ready(()))
                .wait(ctx);
        }
    }
}
