use actix::{Actor, Addr, Context};

use crate::broadcaster::Broadcaster;

#[derive(Default)]
pub struct Room {
    broadcasters: Vec<Addr<Broadcaster>>,
}

impl Actor for Room {
    type Context = Context<Self>;
}


