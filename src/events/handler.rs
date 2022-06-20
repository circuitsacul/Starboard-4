use std::sync::Arc;

use twilight_gateway::Event;

use crate::client::bot::Starboard;

pub async fn handle_event(shard: u64, event: Event, bot: &Arc<Starboard>) {
    bot.cache.write().await.update(&event);
    let bot = Arc::clone(bot);
    println!("Shard {}: {:?}", shard, event.kind());
}
