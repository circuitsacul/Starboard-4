use crate::events::context::EventCtx;

pub async fn handle_event(ctx: EventCtx) {
    println!("Shard {}: {:?}", ctx.shard_id, ctx.event.kind());
}
