use twilight_gateway::Event;
use twilight_model::gateway::payload::outgoing::RequestGuildMembers;

use crate::client::bot::StarboardBot;
use crate::core;
use crate::interactions::commands::register::post_commands;
use crate::interactions::handle::handle_interaction;

pub async fn handle_event(shard_id: u64, event: Event, bot: StarboardBot) {
    tokio::spawn(internal_handle_event(shard_id, event, bot));
}

async fn internal_handle_event(shard_id: u64, event: Event, bot: StarboardBot) {
    bot.cache.update(&event).await;

    let ret = tokio::spawn(match_events(shard_id, event, bot.clone())).await;

    match ret {
        Ok(ret) => match ret {
            Ok(_) => {}
            Err(why) => bot.errors.handle(&bot.http, why).await,
        },
        Err(why) => bot.errors.handle(&bot.http, why).await,
    }
}

async fn match_events(shard_id: u64, event: Event, bot: StarboardBot) -> anyhow::Result<()> {
    match event {
        Event::InteractionCreate(int) => handle_interaction(shard_id, int.0, bot).await?,
        Event::ShardConnected(event) => println!("Shard {} connected.", event.shard_id),
        Event::Ready(info) => {
            if bot.application.read().await.is_none() {
                bot.application.write().await.replace(info.application);
                post_commands(bot).await?;
            }
        }
        Event::MessageCreate(event) => core::autostar::handle(bot, event).await?,
        Event::GuildCreate(event) => {
            // Request members chunk
            bot.cluster
                .command(
                    shard_id,
                    &RequestGuildMembers::builder(event.id).query("", None),
                )
                .await?;
        }
        _ => {}
    }

    Ok(())
}
