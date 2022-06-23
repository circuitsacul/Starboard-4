use std::sync::Arc;

use twilight_gateway::Event;

use crate::client::bot::StarboardBot;
use crate::interactions::commands::register::post_commands;
use crate::interactions::handle::handle_interaction;

pub async fn handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    bot.cache.write().await.update(&event);

    println!("Shard {}: {:?}", shard_id, event.kind());
    tokio::spawn(internal_handle_event(shard_id, event, bot));
}

async fn internal_handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    let ret = match event {
        Event::InteractionCreate(int) => handle_interaction(shard_id, int.0, bot).await,
        Event::Ready(info) => {
            bot.application.write().await.replace(info.application);
            post_commands(bot).await
        }
        _ => Ok(()),
    };

    match ret {
        Ok(_) => {}
        Err(why) => eprintln!("Error in event handler: {}", why),
    }
}
