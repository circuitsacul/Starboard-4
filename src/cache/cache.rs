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
    pub messages: stretto::AsyncCache<Id<MessageMarker>, Arc<CachedMessage>>,

    // database side
    pub autostar_channel_ids: DashSet<Id<ChannelMarker>>,

    // autocomplete
    pub guild_autostar_channel_names: stretto::AsyncCache<Id<GuildMarker>, Arc<Vec<String>>>,
}

impl Cache {
    pub fn new(autostar_channel_ids: DashSet<Id<ChannelMarker>>) -> Self {
        Self {
            guild_emojis: DashMap::new(),
            messages: stretto::AsyncCacheBuilder::new(
                (constants::MAX_MESSAGES * 10).try_into().unwrap(),
                constants::MAX_MESSAGES.into(),
            )
            .finalize(tokio::spawn)
            .unwrap(),
            autostar_channel_ids,
            guild_autostar_channel_names: stretto::AsyncCacheBuilder::new(
                (constants::MAX_AUTOSTAR_NAMES * 10).try_into().unwrap(),
                constants::MAX_MESSAGES.into(),
            )
            .finalize(tokio::spawn)
            .unwrap(),
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
