use std::sync::Arc;

use tokio::signal;
use futures::stream::StreamExt;
use twilight_gateway::cluster::Events;

use crate::client::bot::Starboard;
use crate::events::{EventCtx, handle_event};

async fn shutdown_handler(bot: Arc<Starboard>) {
    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
    }
    println!("Shutting down bot...");
    bot.cluster.down();
    println!("Bot shut down.");
}

pub async fn run(mut events: Events, bot: Starboard) {
    let bot = Arc::new(bot);

    // start the cluster
    let clone = Arc::clone(&bot);
    tokio::spawn(async move { clone.cluster.up().await });
    tokio::spawn(shutdown_handler(Arc::clone(&bot)));

    // handle events
    while let Some((shard_id, event)) = events.next().await {
        bot.cache.write().await.update(&event);

        let ctx = EventCtx {
            shard_id,
            event,
            bot: Arc::clone(&bot),
        };
        tokio::spawn(handle_event(ctx));
    }
}
