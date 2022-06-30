use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use twilight_gateway::Event;
use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker, GuildMarker},
    Id,
};

use super::update::UpdateCache;

#[derive(Default, Clone)]
pub struct Cache {
    // discord side
    pub guild_emojis: Arc<DashMap<Id<GuildMarker>, DashSet<Id<EmojiMarker>>>>,

    // database side
    pub autostar_channel_ids: Arc<DashSet<Id<ChannelMarker>>>,
}

impl Cache {
    pub fn new(autostar_channel_ids: DashSet<Id<ChannelMarker>>) -> Self {
        Self {
            autostar_channel_ids: Arc::new(autostar_channel_ids),
            ..Self::default()
        }
    }

    pub async fn update(&self, event: &Event) {
        match event {
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
