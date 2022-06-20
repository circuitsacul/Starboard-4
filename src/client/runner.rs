use std::sync::Arc;

use futures::stream::StreamExt;
use twilight_gateway::cluster::Events;

use crate::client::bot::Starboard;
use crate::events::event::EventCtx;
use crate::events::handler;

pub async fn run(mut events: Events, bot: Starboard) {
    let bot = Arc::new(bot);

    // start the cluster
    let clone = Arc::clone(&bot);
    tokio::spawn(async move { clone.cluster.up().await });

    // handle events
    while let Some((shard_id, event)) = events.next().await {
        bot.cache.write().await.update(&event);

        let ctx = EventCtx {
            shard: shard_id,
            event,
            bot: Arc::clone(&bot),
        };
        tokio::spawn(handler::handle_event(ctx));
    }
}
