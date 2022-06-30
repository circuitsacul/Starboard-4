use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, GuildEmojisUpdate};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for GuildCreate {
    async fn update_cache(&self, cache: &Cache) {
        // update emojis
        cache
            .guild_emojis
            .insert(self.id, self.emojis.iter().map(|e| e.id).collect());
    }
}

#[async_trait]
impl UpdateCache for GuildDelete {
    async fn update_cache(&self, cache: &Cache) {
        // delete emojis
        cache.guild_emojis.remove(&self.id);
    }
}

#[async_trait]
impl UpdateCache for GuildEmojisUpdate {
    async fn update_cache(&self, cache: &Cache) {
        cache
            .guild_emojis
            .insert(self.guild_id, self.emojis.iter().map(|e| e.id).collect());
    }
}
