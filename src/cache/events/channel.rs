use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelUpdate};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for ChannelCreate {
    async fn update_cache(&self, cache: &Cache) {
        if let Some(nsfw) = self.nsfw {
            cache.channel_nsfws.insert(self.id, nsfw);
        }
    }
}

#[async_trait]
impl UpdateCache for ChannelUpdate {
    async fn update_cache(&self, cache: &Cache) {
        if let Some(nsfw) = self.nsfw {
            cache.channel_nsfws.insert(self.id, nsfw);
        }
    }
}

#[async_trait]
impl UpdateCache for ChannelDelete {
    async fn update_cache(&self, cache: &Cache) {
        cache.channel_nsfws.remove(&self.id);
    }
}
