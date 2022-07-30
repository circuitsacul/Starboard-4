use dashmap::{DashMap, DashSet};
use stretto::ValueRef;
use twilight_gateway::Event;
use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker, GuildMarker, MessageMarker, UserMarker},
    Id,
};

use crate::{
    client::bot::StarboardBot,
    constants,
    errors::StarboardResult,
    utils::{
        async_dash::{AsyncDashMap, AsyncDashSet},
        get_status::get_status,
    },
};

use super::{
    models::{guild::CachedGuild, message::CachedMessage, user::CachedUser},
    update::UpdateCache,
};

pub struct Cache {
    // discord side
    pub guilds: AsyncDashMap<Id<GuildMarker>, CachedGuild>,
    pub users: AsyncDashMap<Id<UserMarker>, CachedUser>,
    pub messages: stretto::AsyncCache<Id<MessageMarker>, Option<CachedMessage>>,

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
            users: DashMap::new().into(),
            messages: stretto::AsyncCache::new(
                (constants::MAX_MESSAGES * 10).try_into().unwrap(),
                constants::MAX_MESSAGES.into(),
                tokio::spawn,
            )
            .unwrap(),
            autostar_channel_ids: autostar_channel_ids.into(),
            guild_autostar_channel_names: stretto::AsyncCache::new(
                (constants::MAX_NAMES * 10).try_into().unwrap(),
                constants::MAX_NAMES.into(),
                tokio::spawn,
            )
            .unwrap(),
            guild_starboard_names: stretto::AsyncCache::new(
                (constants::MAX_NAMES * 10).try_into().unwrap(),
                constants::MAX_NAMES.into(),
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
            Event::ChannelCreate(event) => event.update_cache(self).await,
            Event::ChannelDelete(event) => event.update_cache(self).await,
            Event::ChannelUpdate(event) => event.update_cache(self).await,
            Event::ThreadCreate(event) => event.update_cache(self).await,
            Event::ThreadDelete(event) => event.update_cache(self).await,
            Event::ThreadUpdate(event) => event.update_cache(self).await,
            Event::ThreadListSync(event) => event.update_cache(self).await,
            Event::GuildEmojisUpdate(event) => event.update_cache(self).await,
            Event::MemberChunk(event) => event.update_cache(self).await,
            Event::MemberAdd(event) => event.update_cache(self).await,
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

    // "fetch or get" methods
    pub async fn fog_message(
        &self,
        bot: &StarboardBot,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> StarboardResult<ValueRef<Option<CachedMessage>>> {
        if let Some(cached) = self.messages.get(&message_id) {
            return Ok(cached);
        }

        let msg = bot.http.message(channel_id, message_id).exec().await;
        let msg = match msg {
            Err(why) => {
                if get_status(&why) == Some(404) {
                    None
                } else {
                    return Err(why.into());
                }
            }
            Ok(msg) => Some(msg.model().await.unwrap().into()),
        };

        self.messages.insert(message_id, msg, 1).await;

        self.messages.wait().await.unwrap();
        Ok(self.messages.get(&message_id).unwrap())
    }
}
