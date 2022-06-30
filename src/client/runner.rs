use futures::stream::StreamExt;
use tokio::signal;
use twilight_gateway::cluster::Events;

use crate::client::bot::StarboardBot;
use crate::events::handle_event;

async fn shutdown_handler(bot: StarboardBot) {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
    }
    println!("Shutting down bot...");
    bot.cluster.down();
    println!("Bot shut down.");
}

pub async fn run(mut events: Events, bot: StarboardBot) {
    // start the cluster
    let clone = bot.clone();
    tokio::spawn(async move { clone.cluster.up().await });
    tokio::spawn(shutdown_handler(bot.clone()));

    // handle events
    while let Some((shard_id, event)) = events.next().await {
        handle_event(shard_id, event, bot.clone()).await;
    }
}
