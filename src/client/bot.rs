use std::{fmt::Debug, sync::Arc};

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

use crate::cache::cache::Cache;
use crate::client::config::Config;

#[derive(Clone)]
pub struct StarboardBot {
    pub cluster: Arc<Cluster>,
    pub http: Arc<HttpClient>,
    pub cache: Cache,
    pub application: Arc<RwLock<Option<PartialApplication>>>,
    pub pool: Arc<PgPool>,
    pub errors: Arc<ErrorHandler>,
    pub standby: Arc<Standby>,
    pub config: Arc<Config>,
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
                cluster: Arc::new(cluster),
                http: Arc::new(http),
                cache: cache,
                application: Arc::new(RwLock::new(None)),
                pool: Arc::new(pool),
                errors: Arc::new(errors),
                standby: Arc::new(Standby::new()),
                config: Arc::new(config),
            },
        ))
    }

    pub async fn interaction_client<'a>(&'a self) -> anyhow::Result<InteractionClient<'a>> {
        match &*self.application.read().await {
            Some(info) => Ok(self.http.interaction(info.id)),
            None => Err(anyhow::anyhow!(
                "interaction_client called before bot was ready."
            )),
        }
    }
}
