use std::time::Duration;

use actix::{
    clock::Instant, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner,
    Handler, Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use broadcast_context::{recipient::RecipientResponse, BroadcastContext, Recipient};
use uuid::Uuid;

use crate::{
    broadcaster::Broadcaster,
    event::{ClientEvent, ServerEvent},
    messages::{CloseSession, EnterTheRoom, NewRecipient, NewSession},
    room::Room,
    state::State,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Session {
    id: Uuid,
    broadcaster: Option<Addr<Broadcaster>>,
    room: Option<Addr<Room>>,
    state: Addr<State>,
    hb: Instant,
}

impl Session {
    fn init_heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if Instant::now().duration_since(actor.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut WebsocketContext<Self>) {
        self.init_heartbeat(ctx);
        let session = ctx.address();
        self.state
            .send(NewSession {
                id: self.id,
                session,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.state
            .send(CloseSession(self.id))
            .into_actor(self)
            .then(|_, _, _| actix::fut::ready(()))
            .wait(ctx);
        if let Some(ref room) = self.room {
            room.send(CloseSession(self.id))
                .into_actor(self)
                .then(|_, _, _| actix::fut::ready(()))
                .wait(ctx);
        }
        if let Some(ref broadcaster) = self.broadcaster {
            broadcaster
                .send(CloseSession(self.id))
                .into_actor(self)
                .then(|_, _, _| actix::fut::ready(()))
                .wait(ctx);
        }
        Running::Stop
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Text(event)) => {
                let event: ClientEvent = serde_json::from_str(&event).expect("not valid json");
                match event {
                    ClientEvent::EnterTheRoom(room_name) => {
                        self.state
                            .send(EnterTheRoom { room_name })
                            .into_actor(self)
                            .then(|result, actor, ctx| {
                                if let Ok(Some(room)) = result {
                                    room.send(NewSession {
                                        id: actor.id,
                                        session: ctx.address(),
                                    })
                                    .into_actor(actor)
                                    .then(|_, _, _| actix::fut::ready(()))
                                    .wait(ctx);
                                    actor.room = Some(room);
                                }
                                actix::fut::ready(())
                            })
                            .wait(ctx);
                    }
                    ClientEvent::StartBroadcast(broadcast_key) => {
                        let broadcast = Broadcaster {
                            local_track: None,
                            recipients: Vec::default(),
                            session: ctx.address(),
                        };
                        BroadcastContext::create_with_addr(broadcast_key, broadcast)
                            .into_actor(self)
                            .then(|result, actor, ctx| {
                                if let Ok((broadcaster, response)) = result {
                                    actor.broadcaster = Some(broadcaster);
                                    let response = ServerEvent::BroadcastDescription(response);
                                    ctx.text(serde_json::to_string(&response).unwrap());
                                }
                                actix::fut::ready(())
                            })
                            .wait(ctx);
                    }
                    ClientEvent::StartWatch(receiver_key) => {
                        Recipient::new(receiver_key)
                            .into_actor(self)
                            .then(|result, actor, ctx| {
                                if let (Ok(recipient), Some(broadcast)) =
                                    (result, &actor.broadcaster)
                                {
                                    broadcast.send(NewRecipient { recipient })
                                        .into_actor(actor)
                                        .then(|_, _, _| actix::fut::ready(()))
                                        .wait(ctx);
                                }
                                actix::fut::ready(())
                            })
                            .wait(ctx);
                    }
                }
            }
            Ok(Message::Ping(ref msg)) => {
                self.hb = Instant::now();
                ctx.pong(msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => unimplemented!(),
        }
    }
}

impl Handler<RecipientResponse> for Session {
    type Result = ();

    fn handle(&mut self, msg: RecipientResponse, ctx: &mut Self::Context) -> Self::Result {
        let response = ServerEvent::RecipientDescription(msg.0);
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}
