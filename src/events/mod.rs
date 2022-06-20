use std::sync::Arc;

use twilight_gateway::Event;

use crate::client::bot::Starboard;
use crate::interactions::handle::handle_interaction;
use crate::interactions::commands::register::post_commands;

pub async fn handle_event(shard_id: u64, event: Event, bot: Arc<Starboard>) {
    bot.cache.write().await.update(&event);

    println!("Shard {}: {:?}", shard_id, event.kind());
    tokio::spawn(internal_handle_event(shard_id, event, bot));
}

async fn internal_handle_event(shard_id: u64, event: Event, bot: Arc<Starboard>) {
    match event {
        Event::InteractionCreate(int) => {
            handle_interaction(shard_id, int.0, bot).await;
        }
        Event::Ready(info) => {
            bot.application.write().await.replace(info.application);
            post_commands(bot).await;
        }
        _ => {}
    }
}
