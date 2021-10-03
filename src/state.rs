use crate::room::Room;
use actix::{Actor, Context, Addr};
use std::collections::HashMap;

pub struct State {
    rooms: HashMap<i32, Addr<Room>>,
}

impl Actor for State {
    type Context = Context<Self>;
}
