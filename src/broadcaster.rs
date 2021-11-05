use std::sync::Arc;

use crate::{
    broadcaster_context::BroadcastContext,
    broadcaster_context::LocalTrackMessage,
    recipient::{Recipient, RecipientLocalTrackMessage},
};
use actix::{Actor, ActorContext, ActorFutureExt, Addr, ContextFutureSpawner, Handler, WrapFuture};
use uuid::Uuid;
use webrtc::media::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

use crate::{
    messages::{CloseSession, NewRecipient},
    session::Session,
};

pub struct Broadcaster {
    pub session_id: Uuid,
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
            let recipient = msg.recipient.start();
            self.recipients.push(recipient.clone());
            recipient
                .send(RecipientLocalTrackMessage {
                    address: msg.recipient_addr,
                    broadcaster_id: self.session_id,
                    local_track: Arc::clone(local_track),
                })
                .into_actor(self)
                .then(|_, _, _| actix::fut::ready(()))
                .wait(ctx);
        }
    }
}
