use std::sync::Arc;

use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, Events, ShardScheme},
    Intents,
};
use twilight_http::Client as HttpClient;

use crate::config::Config;
use crate::types::Res;

pub struct Starboard {
    pub cluster: Arc<Cluster>,
    pub http: Arc<HttpClient>,
    pub cache: InMemoryCache,
    pub events: Events,
    config: Config,
}

impl Starboard {
    pub async fn new(config: Config) -> Res<Starboard> {
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

        Ok(Self {
            cluster: Arc::new(cluster),
            http: Arc::new(http),
            cache,
            events,
            config,
        })
    }
}
