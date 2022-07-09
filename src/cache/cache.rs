use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use twilight_gateway::Event;
use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::constants;

use super::{models::message::CachedMessage, update::UpdateCache};

pub struct Cache {
    // discord side
    pub guild_emojis: DashMap<Id<GuildMarker>, DashSet<Id<EmojiMarker>>>,
    pub messages: moka::future::Cache<Id<MessageMarker>, Arc<CachedMessage>>,

    // database side
    pub autostar_channel_ids: DashSet<Id<ChannelMarker>>,

    // autocomplete
    pub guild_autostar_channel_names: moka::future::Cache<Id<GuildMarker>, Arc<Vec<String>>>,
}

impl Cache {
    pub fn new(autostar_channel_ids: DashSet<Id<ChannelMarker>>) -> Self {
        Self {
            guild_emojis: DashMap::new(),
            messages: moka::future::Cache::builder()
                .max_capacity(constants::MAX_MESSAGES)
                .build(),
            autostar_channel_ids: autostar_channel_ids,
            guild_autostar_channel_names: moka::future::Cache::builder()
                .max_capacity(constants::MAX_AUTOSTAR_NAMES)
                .build(),
        }
    }

    pub async fn update(&self, event: &Event) {
        match event {
            Event::MessageCreate(event) => event.update_cache(self).await,
            Event::MessageDelete(event) => event.update_cache(self).await,
            Event::MessageDeleteBulk(event) => event.update_cache(self).await,
            Event::MessageUpdate(event) => event.update_cache(self).await,
            Event::GuildCreate(event) => event.update_cache(self).await,
            Event::GuildDelete(event) => event.update_cache(self).await,
            Event::GuildEmojisUpdate(event) => event.update_cache(self).await,
            _ => {}
        }
    }

    // helper methods
    pub fn guild_emoji_exists(&self, guild_id: Id<GuildMarker>, emoji_id: Id<EmojiMarker>) -> bool {
        match self.guild_emojis.get(&guild_id) {
            None => false,
            Some(guild_emojis) => match guild_emojis.get(&emoji_id) {
                None => false,
                Some(_) => true,
            },
        }
    }
}
