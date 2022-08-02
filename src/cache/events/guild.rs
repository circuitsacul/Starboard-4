use std::collections::HashSet;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, GuildEmojisUpdate};

use crate::cache::{cache::Cache, models::guild::CachedGuild, update::UpdateCache};

#[async_trait]
impl UpdateCache for GuildCreate {
    async fn update_cache(&self, cache: &Cache) {
        let mut nsfw_channels = HashSet::new();
        let mut sfw_channels = HashSet::new();

        for channel in self.channels.iter() {
            match channel.nsfw {
                None => {}
                Some(true) => {
                    nsfw_channels.insert(channel.id);
                }
                Some(false) => {
                    sfw_channels.insert(channel.id);
                }
            }
        }

        let guild = CachedGuild {
            emojis: self.emojis.iter().map(|e| e.id).collect(),
            nsfw_channels,
            sfw_channels,
            active_thread_parents: self
                .threads
                .iter()
                .map(|t| (t.id, t.parent_id.unwrap()))
                .collect(),
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
        cache.guilds.alter(&self.guild_id, |_, mut guild| {
            for emoji in &self.emojis {
                guild.emojis.insert(emoji.id);
            }
            guild
        });
    }
}
