use actix::{Actor, Handler};
use broadcast_context::{BroadcastContext, LocalTrackMessage};
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
