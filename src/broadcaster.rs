use std::sync::Arc;

use actix::{
    Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    WrapFuture,
};
use broadcast_context::{
    recipient::RecipientLocalTrackMessage, BroadcastContext, LocalTrackMessage, Recipient,
};
use tokio::task::spawn_blocking;
use webrtc::media::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

use crate::{
    messages::{CloseSession, NewRecipient, RecipientResponse},
    session::Session,
};

pub struct Broadcaster {
    pub local_track: Option<Arc<TrackLocalStaticRTP>>,
    pub recipients: Vec<Addr<Recipient>>,
    pub session: Addr<Session>,
}

impl Actor for Broadcaster {
    type Context = BroadcastContext<Self>;
}

impl Handler<LocalTrackMessage> for Broadcaster {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, message: LocalTrackMessage, _: &mut Self::Context) -> Self::Result {
        self.local_track = Some(message.local_track);
        Ok(())
    }
}

impl Handler<CloseSession> for Broadcaster {
    type Result = ();

    fn handle(&mut self, _: CloseSession, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<NewRecipient> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: NewRecipient, ctx: &mut Self::Context) -> Self::Result {
        if let Some(ref local_track) = self.local_track {
            let recipent = msg.recipient.start();
            self.recipients.push(recipient.clone());
            recipent
                .send(RecipientLocalTrackMessage {
                    address: self.session.clone(),
                    local_track: Arc::clone(local_track),
                })
                .into_actor(self)
                .then(|_, _, _| actix::fut::ready(()))
                .wait(ctx);
        }
    }
}
