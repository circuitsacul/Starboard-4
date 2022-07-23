use dashmap::{DashMap, DashSet};
use twilight_gateway::Event;
use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::{
    constants,
    utils::async_dash::{AsyncDashMap, AsyncDashSet},
};

use super::{
    models::{guild::CachedGuild, message::CachedMessage},
    update::UpdateCache,
};

pub struct Cache {
    // discord side
    pub guilds: AsyncDashMap<Id<GuildMarker>, CachedGuild>,
    pub channel_nsfws: AsyncDashMap<Id<ChannelMarker>, bool>,
    pub messages: stretto::AsyncCache<Id<MessageMarker>, CachedMessage>,

    // database side
    pub autostar_channel_ids: AsyncDashSet<Id<ChannelMarker>>,

    // autocomplete
    pub guild_autostar_channel_names: stretto::AsyncCache<Id<GuildMarker>, Vec<String>>,
    pub guild_starboard_names: stretto::AsyncCache<Id<GuildMarker>, Vec<String>>,
}

impl Cache {
    pub fn new(autostar_channel_ids: DashSet<Id<ChannelMarker>>) -> Self {
        Self {
            guilds: DashMap::new().into(),
            channel_nsfws: DashMap::new().into(),
            messages: stretto::AsyncCache::new(
                (constants::MAX_MESSAGES * 10).try_into().unwrap(),
                constants::MAX_MESSAGES.into(),
                tokio::spawn,
            )
            .unwrap(),
            autostar_channel_ids: autostar_channel_ids.into(),
            guild_autostar_channel_names: stretto::AsyncCache::new(
                (constants::MAX_AUTOSTAR_NAMES * 10).try_into().unwrap(),
                constants::MAX_AUTOSTAR_NAMES.into(),
                tokio::spawn,
            )
            .unwrap(),
            guild_starboard_names: stretto::AsyncCache::new(
                (constants::MAX_AUTOSTAR_NAMES * 10).try_into().unwrap(),
                constants::MAX_AUTOSTAR_NAMES.into(),
                tokio::spawn,
            )
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
            Event::ChannelCreate(event) => event.update_cache(self).await,
            Event::ChannelDelete(event) => event.update_cache(self).await,
            Event::ChannelUpdate(event) => event.update_cache(self).await,
            _ => {}
        }
    }

    // helper methods
    pub fn guild_emoji_exists(&self, guild_id: Id<GuildMarker>, emoji_id: Id<EmojiMarker>) -> bool {
        self.guilds.with(&guild_id, |_, guild| match guild {
            None => false,
            Some(guild) => guild.emojis.contains(&emoji_id),
        })
    }
}
