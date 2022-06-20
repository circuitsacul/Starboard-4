use std::sync::Arc;

use twilight_gateway::Event;

use crate::client::bot::Starboard;

pub struct EventCtx {
    pub shard_id: u64,
    pub event: Event,
    pub bot: Arc<Starboard>,
}

pub async fn handle_event(ctx: EventCtx) {
    println!("Shard {}: {:?}", ctx.shard_id, ctx.event.kind());
}
