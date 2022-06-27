use std::sync::Arc;

use twilight_gateway::Event;

use crate::client::bot::StarboardBot;
use crate::core;
use crate::interactions::commands::register::post_commands;
use crate::interactions::handle::handle_interaction;

pub async fn handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    bot.cache.update(&event);
    tokio::spawn(internal_handle_event(shard_id, event, bot));
}

async fn internal_handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    let clone = Arc::clone(&bot);
    let ret = match event {
        Event::InteractionCreate(int) => handle_interaction(shard_id, int.0, clone).await,
        Event::ShardConnected(event) => Ok(println!("Shard {} connected.", event.shard_id)),
        Event::Ready(info) => {
            if bot.application.read().await.is_none() {
                bot.application.write().await.replace(info.application);
                post_commands(clone).await
            } else {
                Ok(())
            }
        }
        Event::MessageCreate(event) => core::autostar::handle(clone, event).await,
        _ => Ok(()),
    };

    match ret {
        Ok(_) => {}
        Err(why) => bot.errors.handle(&bot.http, why).await,
    }
}
