use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use twilight_gateway::Event;
use twilight_model::{
    channel::{Channel, ChannelType, Webhook},
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GuildMarker, MessageMarker, UserMarker, WebhookMarker,
        },
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

macro_rules! update_cache_events {
    ($cache: expr, $event: expr, $($matchable_event: path,)*) => {
        match $event {
            $(
                $matchable_event(event) => event.update_cache($cache).await,
            )*
            _ => (),
        }
    };
}

pub struct Cache {
    // discord side
    pub guilds: AsyncDashMap<Id<GuildMarker>, CachedGuild>,
    pub users: AsyncDashMap<Id<UserMarker>, Option<Arc<CachedUser>>>,
    pub webhooks: AsyncDashMap<Id<WebhookMarker>, Arc<Webhook>>,
    pub messages: stretto::AsyncCache<Id<MessageMarker>, Option<Arc<CachedMessage>>>,

    // database side
    pub autostar_channel_ids: AsyncDashSet<Id<ChannelMarker>>,
    pub guild_vote_emojis: AsyncDashMap<i64, Vec<String>>,

    // misc
    pub responses: stretto::AsyncCache<Id<MessageMarker>, Id<MessageMarker>>,
}

impl Cache {
    pub fn new(autostar_channel_ids: DashSet<Id<ChannelMarker>>) -> Self {
        Self {
            guilds: DashMap::new().into(),
            users: DashMap::new().into(),
            webhooks: DashMap::new().into(),
            messages: stretto::AsyncCache::new(
                (constants::MAX_MESSAGES * 10) as usize,
                constants::MAX_MESSAGES.into(),
                tokio::spawn,
            )
            .unwrap(),
            autostar_channel_ids: autostar_channel_ids.into(),
            guild_vote_emojis: DashMap::new().into(),
            responses: stretto::AsyncCache::new(
                (constants::MAX_STORED_RESPONSES * 10) as usize,
                constants::MAX_STORED_RESPONSES.into(),
                tokio::spawn,
            )
            .unwrap(),
        }
    }

    pub async fn update(&self, event: &Event) {
        update_cache_events!(
            self,
            event,
            Event::MessageCreate,
            Event::MessageDelete,
            Event::MessageDeleteBulk,
            Event::MessageUpdate,
            Event::GuildCreate,
            Event::GuildDelete,
            Event::RoleCreate,
            Event::RoleDelete,
            Event::RoleUpdate,
            Event::ChannelCreate,
            Event::ChannelDelete,
            Event::ChannelUpdate,
            Event::ThreadCreate,
            Event::ThreadDelete,
            Event::ThreadUpdate,
            Event::ThreadListSync,
            Event::GuildEmojisUpdate,
            Event::MemberChunk,
            Event::MemberAdd,
            Event::MemberRemove,
            Event::MemberUpdate,
        );
    }

    // helper methods
    pub fn guild_emoji_exists(&self, guild_id: Id<GuildMarker>, emoji_id: Id<EmojiMarker>) -> bool {
        self.guilds.with(&guild_id, |_, guild| {
            guild
                .as_ref()
                .map_or(false, |guild| guild.emojis.contains_key(&emoji_id))
        })
    }

    pub fn is_emoji_animated(
        &self,
        guild_id: Id<GuildMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Option<bool> {
        self.guilds.with(&guild_id, |_, guild| {
            guild
                .as_ref()
                .and_then(|guild| guild.emojis.get(&emoji_id).copied())
        })
    }

    pub fn is_channel_forum(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> bool {
        self.guilds.with(&guild_id, |_, guild| {
            guild
                .as_ref()
                .and_then(|guild| {
                    guild
                        .channels
                        .get(&channel_id)
                        .map(|channel| channel.kind == ChannelType::GuildForum)
                })
                .unwrap_or(false)
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
                    current_channel_id = Some(*thread_parent_id);
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
                let channel = bot.http.channel(channel_id).await?.model().await?;
                current_channel_id = channel.parent_id;
            }
        }

        Ok(channel_ids)
    }

    fn get_user(&self, user_id: Id<UserMarker>) -> Option<Arc<CachedUser>> {
        self.users
            .with(&user_id, |_, v| v.as_ref().and_then(|u| (*u).clone()))
    }

    pub async fn fog_user(
        &self,
        bot: &StarboardBot,
        user_id: Id<UserMarker>,
    ) -> StarboardResult<Option<Arc<CachedUser>>> {
        if !self.users.contains_key(&user_id) {
            let user_get = bot.http.user(user_id).await;
            let user = match user_get {
                Ok(user) => Some(Arc::new(user.model().await?.into())),
                Err(why) => {
                    if get_status(&why) == Some(404) {
                        None
                    } else {
                        return Err(why.into());
                    }
                }
            };

            self.users.insert(user_id, user);
        }

        Ok(self.get_user(user_id))
    }

    pub async fn fog_webhook(
        &self,
        bot: &StarboardBot,
        webhook_id: Id<WebhookMarker>,
    ) -> StarboardResult<Option<Arc<Webhook>>> {
        let cached = self.webhooks.with(&webhook_id, |_, wh| {
            wh.as_ref().map(|wh| wh.value().clone())
        });

        if cached.is_some() {
            return Ok(cached);
        }

        let wh = bot.http.webhook(webhook_id).await;

        let wh = match wh {
            Err(why) => {
                if get_status(&why) == Some(404) {
                    None
                } else {
                    return Err(why.into());
                }
            }
            Ok(wh) => {
                let wh = Arc::new(wh.model().await?);
                self.webhooks.insert(webhook_id, wh.clone());
                Some(wh)
            }
        };

        Ok(wh)
    }

    pub async fn fog_message(
        &self,
        bot: &StarboardBot,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> StarboardResult<Option<Arc<CachedMessage>>> {
        if let Some(cached) = self.messages.get(&message_id) {
            return Ok(cached.value().clone());
        }

        let msg = bot.http.message(channel_id, message_id).await;
        let msg = match msg {
            Err(why) => {
                if get_status(&why) == Some(404) {
                    None
                } else {
                    return Err(why.into());
                }
            }
            Ok(msg) => Some(Arc::new(msg.model().await?.into())),
        };

        self.messages.insert(message_id, msg, 1).await;

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
            let channel = bot.http.channel(channel_id).await;
            let channel = match channel {
                Ok(channel) => channel,
                Err(why) => {
                    return match get_status(&why) {
                        Some(404) => Ok(None),
                        _ => Err(why.into()),
                    }
                }
            };
            Ok(Some(channel.model().await?))
        }

        let Some(channel) = get_channel(bot, channel_id).await? else {
            return Ok(None);
        };
        if channel.kind.is_thread() {
            get_channel(bot, channel.parent_id.unwrap()).await
        } else {
            Ok(Some(channel))
        }
    }

    pub async fn fog_parent_channel_id(
        &self,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> StarboardResult<Option<Id<ChannelMarker>>> {
        let parent = self.guilds.with(&guild_id, |_, guild| {
            let Some(guild) = guild else { return None; };

            if guild.channels.contains_key(&channel_id) {
                return Some(channel_id);
            }

            if let Some(parent) = guild.active_thread_parents.get(&channel_id) {
                return Some(*parent);
            }

            None
        });

        if parent.is_some() {
            return Ok(parent);
        }

        let Some(parent) = self.fetch_channel_or_thread_parent(bot, channel_id).await? else {
            return Ok(None);
        };
        if parent.guild_id != Some(guild_id) {
            return Ok(None);
        }

        self.guilds.alter(&guild_id, |_, mut guild| {
            guild.channels.insert(
                parent.id,
                CachedChannel::from_channel(guild.channels.get(&parent.id), &parent),
            );
            guild
        });

        Ok(Some(parent.id))
    }

    pub async fn guild_has_channel(
        &self,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> StarboardResult<bool> {
        let parent_id = self
            .fog_parent_channel_id(bot, guild_id, channel_id)
            .await?;
        let Some(parent_id) = parent_id else {
            return Ok(false);
        };

        Ok(self.guilds.with(&guild_id, |_, guild| {
            guild
                .as_ref()
                .map_or(false, |guild| guild.channels.contains_key(&parent_id))
        }))
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
            let Some(guild) = guild else { return CachedResult::None; };

            // check if the channel_id is a known thread, and use the parent_id
            // if it is.
            let channel_id = guild
                .active_thread_parents
                .get(&channel_id)
                .map_or(channel_id, |&parent_id| parent_id);

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
        let Some(parent) = self.fetch_channel_or_thread_parent(bot, channel_id).await? else {
            return Ok(None);
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
