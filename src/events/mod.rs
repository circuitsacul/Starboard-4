use std::sync::Arc;

use twilight_gateway::Event;
use twilight_model::gateway::payload::outgoing::RequestGuildMembers;

use crate::{
    client::bot::StarboardBot,
    core,
    interactions::{commands::register::post_commands, handle::handle_interaction},
};

pub async fn handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    tokio::spawn(internal_handle_event(shard_id, event, bot));
}

async fn internal_handle_event(shard_id: u64, event: Event, bot: Arc<StarboardBot>) {
    bot.cache.update(&event).await;
    bot.standby.process(&event);

    let ret = tokio::spawn(match_events(shard_id, event, bot.clone())).await;

    match ret {
        Ok(ret) => match ret {
            Ok(_) => {}
            Err(why) => bot.errors.handle(&bot.http, why).await,
        },
        Err(why) => bot.errors.handle(&bot.http, why).await,
    }
}

async fn match_events(shard_id: u64, event: Event, bot: Arc<StarboardBot>) -> anyhow::Result<()> {
    match event {
        Event::InteractionCreate(int) => handle_interaction(shard_id, int.0, bot).await?,
        Event::ShardConnected(event) => println!("Shard {} connected.", event.shard_id),
        Event::Ready(info) => {
            if bot.application.read().await.is_none() {
                bot.application.write().await.replace(info.application);
                post_commands(bot).await;
            }
        }
        Event::MessageCreate(event) => {
            core::autostar::handle(&bot, &event).await?;
            crate::owner::handle::handle_message(shard_id, &bot, &event).await?;
        }
        Event::GuildCreate(event) => {
            // Request members chunk
            bot.cluster
                .command(
                    shard_id,
                    &RequestGuildMembers::builder(event.id).query("", None),
                )
                .await?;
        }
        Event::ReactionAdd(event) => {
            core::starboard::reaction_events::handle_reaction_add(&bot, event).await?;
        }
        Event::ReactionRemove(event) => {
            core::starboard::reaction_events::handle_reaction_remove(&bot, event).await?;
        }
        Event::ReactionRemoveAll(event) => {
            core::starboard::reaction_events::handle_reaction_remove_all(&bot, event).await?;
        }
        Event::ReactionRemoveEmoji(event) => {
            core::starboard::reaction_events::handle_reaction_remove_emoji(&bot, event).await?;
        }
        _ => {}
    }

    Ok(())
}
