use async_trait::async_trait;
use dashmap::DashSet;
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, GuildEmojisUpdate};

use crate::cache::{cache::Cache, models::guild::CachedGuild, update::UpdateCache};

#[async_trait]
impl UpdateCache for GuildCreate {
    async fn update_cache(&self, cache: &Cache) {
        let guild = CachedGuild {
            emojis: self
                .emojis
                .iter()
                .map(|e| e.id)
                .collect::<DashSet<_>>()
                .into(),
        };
        cache.guilds.insert(self.id, guild);
    }
}

#[async_trait]
impl UpdateCache for GuildDelete {
    async fn update_cache(&self, cache: &Cache) {
        cache.guilds.remove(&self.id);
    }
}

#[async_trait]
impl UpdateCache for GuildEmojisUpdate {
    async fn update_cache(&self, cache: &Cache) {
        cache.guilds.alter(&self.guild_id, |_, guild| {
            for emoji in &self.emojis {
                guild.emojis.insert(emoji.id);
            }
            guild
        });
    }
}
