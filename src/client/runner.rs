use std::sync::Arc;

use tokio::{
    signal::unix::{SignalKind, signal},
    task::JoinSet,
};
use twilight_gateway::{EventTypeFlags, Shard, StreamExt as _, create_iterator};

use crate::{
    client::bot::StarboardBot,
    core::{
        posroles::loop_update_posroles,
        premium::{
            expire::loop_expire_premium, patreon::patreon_loop, roles::loop_update_supporter_roles,
        },
    },
    events::handle_event,
};

use super::cooldowns::Cooldowns;

async fn wait_for_shutdown() {
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    tokio::select! {
        _ = terminate.recv() => {
            println!("Received terminate signal.");
        }
        _ = interrupt.recv() => {
            println!("Received interrupt signal.");
        }
    }
}

pub async fn run(bot: StarboardBot) {
    let bot = Arc::new(bot);
    Cooldowns::start(bot.clone());

    if bot.config.development {
        println!("Running bot in development mode.");
    }

    // start background tasks
    tokio::spawn(loop_update_posroles(bot.clone()));
    tokio::spawn(loop_expire_premium(bot.clone()));
    tokio::spawn(patreon_loop(bot.clone()));
    tokio::spawn(loop_update_supporter_roles(bot.clone()));

    // handle events
    let shards: Vec<_> = create_iterator(
        0..bot.config.shards,
        bot.config.shards,
        bot.gw_config.clone(),
        |_, b| b.build(),
    )
    .collect();

    let mut runners = JoinSet::new();
    for shard in shards {
        runners.spawn(run_shard(shard, bot.clone()));
    }

    wait_for_shutdown().await;
    runners.shutdown().await;
}

async fn run_shard(mut shard: Shard, bot: Arc<StarboardBot>) {
    let shard_id = shard.id();
    while let Some(event) = shard.next_event(EventTypeFlags::all()).await {
        match event {
            Ok(event) => handle_event(shard_id, event, bot.clone()),
            Err(why) => bot.handle_error(&why.into()).await,
        }
    }
}
