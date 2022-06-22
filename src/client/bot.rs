use std::fmt::Debug;

use sqlx::PgPool;
use anyhow::{anyhow, Result};
use tokio::sync::RwLock;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, Events, ShardScheme},
    Intents,
};
use twilight_http::client::{Client as HttpClient, InteractionClient};
use twilight_model::oauth::PartialApplication;

use crate::client::config::Config;
use crate::utils::types::Res;

pub struct Starboard {
    pub cluster: Cluster,
    pub http: HttpClient,
    pub cache: RwLock<InMemoryCache>,
    pub application: RwLock<Option<PartialApplication>>,
    pub pool: PgPool,
}

impl Debug for Starboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Starboard")
    }
}

impl Starboard {
    pub async fn new(config: Config) -> Res<(Events, Starboard)> {
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
            .resource_types(ResourceType::MESSAGE)
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

    pub async fn interaction_client<'a>(&'a self) -> Result<InteractionClient<'a>> {
        match self.application.read().await.clone() {
            Some(info) => Ok(self.http.interaction(info.id)),
            None => Err(anyhow!("interaction_client called before bot was ready.")),
        }
    }
}
