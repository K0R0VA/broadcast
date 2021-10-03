use actix::{Actor, ActorContext, Handler};
use broadcast_context::{BroadcastContext, LocalTrackMessage};

use crate::messages::CloseSession;
pub struct Broadcaster;

impl Actor for Broadcaster {
    type Context = BroadcastContext<Self>;
}

impl Handler<LocalTrackMessage> for Broadcaster {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _: LocalTrackMessage, _: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<CloseSession> for Broadcaster {
    type Result = ();

    fn handle(&mut self, _: CloseSession, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}