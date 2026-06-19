use actix::prelude::*;

use crate::link::{PodMessage, PodResponse};
pub struct Pod;

// Provide Actor implementation for our actor
impl Actor for Pod {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}

    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

/// Define handler for `Messages` enum
impl Handler<PodMessage> for Pod {
    type Result = PodResponse;

    fn handle(&mut self, msg: PodMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PodMessage::Query(s) => PodResponse::Ok(s),
            _ => PodResponse::Err,
        }
    }
}
