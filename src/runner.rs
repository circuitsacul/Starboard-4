use futures::stream::StreamExt;

use crate::bot::Starboard;

pub async fn run(mut bot: Starboard) {
    let cluster_spawn = bot.cluster.clone();
    tokio::spawn(async move { cluster_spawn.up().await });

    while let Some((shard_id, event)) = bot.events.next().await {
        bot.cache.update(&event);

        println!("Shard {}: {:?}", shard_id, event.kind());
    }
}
