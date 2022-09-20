use std::fmt::Debug;

use sqlx::PgPool;
use tokio::sync::RwLock;
use twilight_error::ErrorHandler;
use twilight_gateway::{
    cluster::{Cluster, Events, ShardScheme},
    Intents,
};
use twilight_http::client::{Client as HttpClient, InteractionClient};
use twilight_model::{
    id::{marker::ChannelMarker, Id},
    oauth::PartialApplication,
};
use twilight_standby::Standby;

use crate::{cache::cache::Cache, client::config::Config};

use super::{cooldowns::Cooldowns, locks::Locks};

pub struct StarboardBot {
    pub cluster: Cluster,
    pub http: HttpClient,
    pub cache: Cache,
    pub application: RwLock<Option<PartialApplication>>,
    pub pool: PgPool,
    pub errors: ErrorHandler,
    pub standby: Standby,
    pub config: Config,
    pub cooldowns: Cooldowns,
    pub locks: Locks,
}

impl Debug for StarboardBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Starboard")
    }
}

impl StarboardBot {
    pub async fn new(config: Config) -> anyhow::Result<(Events, StarboardBot)> {
        // Setup gateway connection
        let scheme = ShardScheme::try_from((0..config.shards, config.shards))?;
        let intents = Intents::GUILDS
            | Intents::GUILD_EMOJIS_AND_STICKERS
            | Intents::GUILD_MEMBERS
            | Intents::GUILD_MESSAGES
            | Intents::MESSAGE_CONTENT
            | Intents::GUILD_MESSAGE_REACTIONS;

        let (cluster, events) = Cluster::builder(config.token.clone(), intents)
            .shard_scheme(scheme)
            .build()
            .await?;

        // Setup HTTP connection
        let http = HttpClient::new(config.token.clone());

        // Setup database connection
        let pool = PgPool::connect(&config.db_url).await?;

        // run migrations
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("failed to run migrations");

        // Setup error handling
        let mut errors = ErrorHandler::new();
        if let Some(channel_id) = config.error_channel {
            errors.channel(channel_id.try_into().unwrap());
        }

        // load autostar channels
        let asc: Vec<_> = sqlx::query!("SELECT channel_id FROM autostar_channels")
            .fetch_all(&pool)
            .await?
            .into_iter()
            .map(|rec| Id::<ChannelMarker>::new(rec.channel_id as u64))
            .collect();

        let mut map = dashmap::DashSet::new();
        map.extend(asc);

        // Setup cache
        let cache = Cache::new(map);

        // Return the bot struct
        Ok((
            events,
            Self {
                cluster,
                http,
                cache,
                application: RwLock::new(None),
                pool,
                errors,
                standby: Standby::new(),
                config,
                cooldowns: Cooldowns::new(),
                locks: Locks::new(),
            },
        ))
    }

    pub async fn interaction_client(&self) -> InteractionClient {
        match &*self.application.read().await {
            Some(info) => self.http.interaction(info.id),
            None => panic!("interaction_client called before bot was ready."),
        }
    }
}
