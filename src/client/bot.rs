use tokio::sync::RwLock;

use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, Events, ShardScheme},
    Intents,
};
use twilight_http::{Client as HttpClient, client::InteractionClient};
use twilight_model::oauth::PartialApplication;

use crate::client::config::Config;
use crate::utils::types::Res;

#[derive(Debug)]
pub struct Starboard {
    pub cluster: Cluster,
    pub http: HttpClient,
    pub cache: RwLock<InMemoryCache>,
    pub app_info: RwLock<Option<PartialApplication>>,
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

        Ok((
            events,
            Self {
                cluster,
                http,
                cache: RwLock::new(cache),
                app_info: RwLock::new(None),
            },
        ))
    }

    pub async fn interaction_client<'a>(&'a self) -> Option<InteractionClient<'a>> {
        match self.app_info.read().await.clone() {
            Some(info) => Some(self.http.interaction(info.id)),
            None => None,
        }
    }
}
