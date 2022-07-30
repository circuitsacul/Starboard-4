use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelUpdate};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for ChannelCreate {
    async fn update_cache(&self, cache: &Cache) {
        let guild_id = match self.guild_id {
            None => return,
            Some(guild_id) => guild_id,
        };
        let is_nsfw = match self.nsfw {
            None => return,
            Some(nsfw) => nsfw,
        };

        cache.guilds.alter(&guild_id, |_, mut guild| {
            match is_nsfw {
                true => {
                    guild.nsfw_channels.insert(self.id);
                }
                false => {
                    guild.sfw_channels.insert(self.id);
                }
            }
            guild
        })
    }
}

#[async_trait]
impl UpdateCache for ChannelDelete {
    async fn update_cache(&self, cache: &Cache) {
        let guild_id = match self.guild_id {
            None => return,
            Some(guild_id) => guild_id,
        };

        cache.guilds.alter(&guild_id, |_, mut guild| {
            guild.nsfw_channels.remove(&self.id);
            guild.sfw_channels.remove(&self.id);
            guild.active_thread_parents = guild
                .active_thread_parents
                .into_iter()
                .filter(|(_, channel_id)| channel_id != &self.id)
                .collect();

            guild
        })
    }
}

#[async_trait]
impl UpdateCache for ChannelUpdate {
    async fn update_cache(&self, cache: &Cache) {
        let guild_id = match self.guild_id {
            None => return,
            Some(guild_id) => guild_id,
        };
        let is_nsfw = match self.nsfw {
            None => return,
            Some(nsfw) => nsfw,
        };

        cache.guilds.alter(&guild_id, |_, mut guild| {
            if is_nsfw {
                guild.sfw_channels.remove(&self.id);
                guild.nsfw_channels.insert(self.id);
            } else {
                guild.nsfw_channels.remove(&self.id);
                guild.sfw_channels.insert(self.id);
            }

            guild
        })
    }
}
