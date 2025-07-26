use std::collections::HashMap;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, GuildEmojisUpdate};

use crate::cache::{
    cache_struct::Cache,
    models::{channel::CachedChannel, guild::CachedGuild},
    update::UpdateCache,
};

#[async_trait]
impl UpdateCache for GuildCreate {
    async fn update_cache(&self, cache: &Cache) {
        let guild = match self {
            GuildCreate::Unavailable(guild) => {
                cache.guilds.remove(&guild.id);
                return;
            }
            GuildCreate::Available(guild) => guild,
        };

        let channels = guild
            .channels
            .iter()
            .map(|c| (c.id, CachedChannel::from_channel(None, c)))
            .collect();

        let new_guild = CachedGuild {
            name: guild.name.clone(),
            emojis: guild.emojis.iter().map(|e| (e.id, e.animated)).collect(),
            channels,
            active_thread_parents: guild
                .threads
                .iter()
                .map(|t| (t.id, t.parent_id.unwrap()))
                .collect(),
            roles: guild
                .roles
                .iter()
                .map(|r| (r.id, r.into()))
                .collect::<HashMap<_, _>>(),
        };
        cache.guilds.insert(guild.id, new_guild);
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
                guild.emojis.insert(emoji.id, emoji.animated);
            }
            guild
        });
    }
}
