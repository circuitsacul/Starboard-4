use std::sync::Arc;

use futures::stream::StreamExt;

use crate::client::bot::Starboard;
use crate::events::handler;

pub async fn run(bot: Starboard) {
    let bot = Arc::new(bot);

    let clone = Arc::clone(&bot);
    tokio::spawn(async move { clone.cluster.up().await });

    while let Some((shard_id, event)) = bot.events.write().await.next().await {
        handler::handle_event(shard_id, event, &bot).await;
    }
}
