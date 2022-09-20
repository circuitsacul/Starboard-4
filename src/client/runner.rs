use std::sync::Arc;

use futures::stream::StreamExt;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::mpsc,
};
use twilight_gateway::cluster::Events;

use crate::{client::bot::StarboardBot, events::handle_event};

async fn shutdown_handler(bot: Arc<StarboardBot>) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for kind in [SignalKind::terminate(), SignalKind::interrupt()] {
        let sender = tx.clone();
        let mut listener = signal(kind).unwrap();
        tokio::spawn(async move {
            listener.recv().await;
            sender.send(()).expect("failed to send signal");
        });
    }

    rx.recv().await;
    println!("Shutting down bot...");
    bot.cluster.down();
    println!("Bot shut down.");
}

pub async fn run(mut events: Events, bot: StarboardBot) {
    let bot = Arc::new(bot);

    if bot.config.development {
        println!("Running bot in development mode.");
    }

    // start the cluster
    let clone = bot.clone();
    tokio::spawn(async move { clone.cluster.up().await });
    tokio::spawn(shutdown_handler(bot.clone()));

    // handle events
    while let Some((shard_id, event)) = events.next().await {
        handle_event(shard_id, event, bot.clone()).await;
    }
}
