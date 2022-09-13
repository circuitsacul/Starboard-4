use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use twilight_gateway::Event;
use twilight_model::{
    channel::Channel,
    id::{
        marker::{ChannelMarker, EmojiMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};

use crate::{
    cache::models::channel::CachedChannel,
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
    pub messages: stretto::AsyncCache<Id<MessageMarker>, Arc<Option<CachedMessage>>>,

    // database side
    pub autostar_channel_ids: AsyncDashSet<Id<ChannelMarker>>,
    pub guild_vote_emojis: AsyncDashMap<i64, Vec<String>>,

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
            guild_vote_emojis: DashMap::new().into(),
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

    pub async fn qualified_channel_ids(
        &self,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> StarboardResult<Vec<Id<ChannelMarker>>> {
        let mut current_channel_id = Some(channel_id);
        let mut channel_ids = Vec::new();

        while let Some(channel_id) = current_channel_id {
            channel_ids.push(channel_id);

            let must_fetch = self.guilds.with(&guild_id, |_, guild| {
                let guild = guild.as_ref().unwrap();

                if let Some(thread_parent_id) = guild.active_thread_parents.get(&channel_id) {
                    current_channel_id = Some(thread_parent_id.to_owned());
                    return false;
                }

                if let Some(channel) = guild.channels.get(&channel_id) {
                    if let Some(parent_id) = channel.parent_id {
                        current_channel_id = Some(parent_id);
                    } else {
                        current_channel_id = None;
                    }
                    return false;
                }

                true
            });

            if must_fetch {
                let channel = bot
                    .http
                    .channel(channel_id)
                    .exec()
                    .await?
                    .model()
                    .await
                    .unwrap();
                current_channel_id = channel.parent_id;
            }
        }

        Ok(channel_ids)
    }

    pub async fn ensure_user(
        &self,
        bot: &StarboardBot,
        user_id: Id<UserMarker>,
    ) -> StarboardResult<()> {
        if self.users.contains_key(&user_id) {
            return Ok(());
        }

        let user = bot.http.user(user_id).exec().await?.model().await.unwrap();

        self.users.insert(user_id, (&user).into());

        Ok(())
    }

    pub async fn fog_message(
        &self,
        bot: &StarboardBot,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> StarboardResult<Arc<Option<CachedMessage>>> {
        if let Some(cached) = self.messages.get(&message_id) {
            return Ok(cached.value().clone());
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

        self.messages.insert(message_id, Arc::new(msg), 1).await;

        self.messages.wait().await.unwrap();
        Ok(self.messages.get(&message_id).unwrap().value().clone())
    }

    async fn fetch_channel_or_thread_parent(
        &self,
        bot: &StarboardBot,
        channel_id: Id<ChannelMarker>,
    ) -> StarboardResult<Option<Channel>> {
        async fn get_channel(
            bot: &StarboardBot,
            channel_id: Id<ChannelMarker>,
        ) -> StarboardResult<Option<Channel>> {
            let channel = bot.http.channel(channel_id).exec().await;
            let channel = match channel {
                Ok(channel) => channel,
                Err(why) => {
                    return match get_status(&why) {
                        Some(404) => Ok(None),
                        _ => Err(why.into()),
                    }
                }
            };
            Ok(Some(channel.model().await.unwrap()))
        }

        let channel = match get_channel(bot, channel_id).await? {
            None => return Ok(None),
            Some(channel) => channel,
        };
        if channel.kind.is_thread() {
            get_channel(bot, channel.parent_id.unwrap()).await
        } else {
            Ok(Some(channel))
        }
    }

    pub async fn fog_channel_nsfw(
        &self,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> StarboardResult<Option<bool>> {
        // First, check for the cached value.
        enum CachedResult {
            NotCached(Id<ChannelMarker>),
            Cached(bool),
            None,
        }

        let is_nsfw = self.guilds.with(&guild_id, |_, guild| {
            // get the guild from the cache
            let guild = match guild {
                None => return CachedResult::None,
                Some(guild) => guild,
            };

            // check if the channel_id is a known thread, and use the parent_id
            // if it is.
            let channel_id = match guild.active_thread_parents.get(&channel_id) {
                None => channel_id,
                Some(parent_id) => *parent_id,
            };

            // check the cached nsfw/sfw channel list
            if let Some(channel) = guild.channels.get(&channel_id) {
                if let Some(nsfw) = channel.is_nsfw {
                    return CachedResult::Cached(nsfw);
                }
            }

            CachedResult::NotCached(channel_id)
        });

        // handle the result
        let channel_id = match is_nsfw {
            CachedResult::None => return Ok(None),
            CachedResult::Cached(is_nsfw) => return Ok(Some(is_nsfw)),
            CachedResult::NotCached(channel_id) => channel_id,
        };

        // fetch the data from discord
        let parent = match self.fetch_channel_or_thread_parent(bot, channel_id).await? {
            None => return Ok(None),
            Some(parent) => parent,
        };
        // since this is 100% going to be a parent channel, and since discord always
        // includes the `nsfw` parameter for channels fetched over the api, this
        // should be safe.
        let is_nsfw = parent.nsfw.unwrap();

        // cache the value
        self.guilds.alter(&guild_id, |_, mut guild| {
            guild.channels.insert(
                parent.id,
                CachedChannel::from_channel(guild.channels.get(&parent.id), &parent),
            );
            guild
        });

        Ok(Some(is_nsfw))
    }
}
