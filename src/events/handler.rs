use crate::events::event::EventCtx;

pub async fn handle_event(ctx: EventCtx) {
    println!("Shard {}: {:?}", ctx.shard, ctx.event.kind());
}
