use std::fmt::Debug;

use sqlx::PgPool;
use tokio::sync::RwLock;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, Events, ShardScheme},
    Intents,
};
use twilight_http::client::{Client as HttpClient, InteractionClient};
use twilight_model::oauth::PartialApplication;

use crate::client::config::Config;

pub struct StarboardBot {
    pub cluster: Cluster,
    pub http: HttpClient,
    pub cache: RwLock<InMemoryCache>,
    pub application: RwLock<Option<PartialApplication>>,
    pub pool: PgPool,
}

impl Debug for StarboardBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Starboard")
    }
}

impl StarboardBot {
    pub async fn new(config: Config) -> anyhow::Result<(Events, StarboardBot)> {
        let scheme = ShardScheme::try_from((
            0..config.shards_per_cluster,
            config.shards_per_cluster * config.total_clusters,
        ))?;
        let intents = Intents::GUILDS
            | Intents::GUILD_MEMBERS
            | Intents::GUILD_MESSAGES
            | Intents::GUILD_MESSAGE_REACTIONS;

        let (cluster, events) = Cluster::builder(config.token.clone(), intents)
            .shard_scheme(scheme)
            .build()
            .await?;

        let http = HttpClient::new(config.token.clone());
        let cache = InMemoryCache::builder()
            .resource_types(
                ResourceType::USER
                    | ResourceType::USER_CURRENT
                    | ResourceType::MEMBER
                    | ResourceType::MESSAGE
                    | ResourceType::GUILD
                    | ResourceType::CHANNEL
                    | ResourceType::ROLE
                    | ResourceType::EMOJI,
            )
            .message_cache_size(10_000)
            .build();

        let pool = PgPool::connect(&config.db_url).await?;

        Ok((
            events,
            Self {
                cluster,
                http,
                cache: RwLock::new(cache),
                application: RwLock::new(None),
                pool,
            },
        ))
    }

    pub async fn interaction_client<'a>(&'a self) -> anyhow::Result<InteractionClient<'a>> {
        match self.application.read().await.clone() {
            Some(info) => Ok(self.http.interaction(info.id)),
            None => Err(anyhow::anyhow!(
                "interaction_client called before bot was ready."
            )),
        }
    }
}
