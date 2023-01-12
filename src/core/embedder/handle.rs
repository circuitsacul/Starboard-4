use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    cache::models::{message::CachedMessage, user::CachedUser},
    client::bot::StarboardBot,
    core::starboard::{config::StarboardConfig, webhooks::get_valid_webhook},
    database::{Message as DbMessage, Starboard},
    errors::StarboardResult,
    utils::{get_status::get_status, id_as_i64::GetI64, into_id::IntoId},
};

use super::{attachment_handle::VecAttachments, builder::BuiltStarboardEmbed};

pub struct Embedder<'config, 'bot> {
    pub bot: &'bot StarboardBot,
    pub points: i32,
    pub config: &'config StarboardConfig,
    pub orig_message: Option<Arc<CachedMessage>>,
    pub orig_message_author: Option<Arc<CachedUser>>,
    pub referenced_message: Option<Arc<CachedMessage>>,
    pub referenced_message_author: Option<Arc<CachedUser>>,
    pub orig_sql_message: Arc<DbMessage>,
}

impl Embedder<'_, '_> {
    pub fn build(&self, force_partial: bool, watermark: bool) -> BuiltStarboardEmbed {
        BuiltStarboardEmbed::build(self, force_partial, watermark)
    }

    pub async fn send(
        &self,
        bot: &StarboardBot,
    ) -> StarboardResult<twilight_model::channel::Message> {
        let built = match self.build(false, self.config.resolved.use_webhook) {
            BuiltStarboardEmbed::Full(built) => built,
            BuiltStarboardEmbed::Partial(_) => panic!("Tried to send an unbuildable message."),
        };
        let (attachments, errors) = built.upload_attachments.as_attachments(bot).await;

        for e in errors {
            bot.handle_error(&e).await;
        }

        let guild_id = self.config.starboard.guild_id.into_id();
        let sb_channel_id = self.config.starboard.channel_id.into_id();

        let forum_post_name = if bot.cache.is_channel_forum(guild_id, sb_channel_id) {
            Some("New Post")
        } else {
            None
        };

        if self.config.resolved.use_webhook {
            loop {
                if let Some(wh) = get_valid_webhook(bot, &self.config.starboard, true).await? {
                    let parent = bot
                        .cache
                        .fog_parent_channel_id(bot, guild_id, sb_channel_id)
                        .await?
                        .unwrap();

                    let mut ret = bot
                        .http
                        .execute_webhook(wh.id, wh.token.as_ref().unwrap())
                        .content(&built.top_content)?
                        .embeds(&built.embeds)?
                        .attachments(&attachments)?;

                    if parent != sb_channel_id {
                        ret = ret.thread_id(sb_channel_id);
                    }

                    if let Some(name) = forum_post_name {
                        ret = ret.thread_name(name);
                    }

                    let ret = ret.wait().await;

                    let err = match ret {
                        Err(err) => err,
                        Ok(msg) => return Ok(msg.model().await?),
                    };

                    if get_status(&err) == Some(404) {
                        bot.cache.webhooks.remove(&wh.id);
                        continue;
                    }
                }

                Starboard::disable_webhooks(&bot.pool, self.config.starboard.id).await?;
                break;
            }
        }

        if let Some(name) = forum_post_name {
            let ret = bot
                .http
                .create_forum_thread(sb_channel_id, name)
                .message()
                .content(&built.top_content)?
                .embeds(&built.embeds)?
                .attachments(&attachments)?
                .await?
                .model()
                .await?;

            Ok(ret.message)
        } else {
            Ok(bot
                .http
                .create_message(self.config.starboard.channel_id.into_id())
                .content(&built.top_content)?
                .embeds(&built.embeds)?
                .attachments(&attachments)?
                .await?
                .model()
                .await?)
        }
    }

    pub async fn edit(
        &self,
        bot: &StarboardBot,
        message_id: Id<MessageMarker>,
        force_partial: bool,
    ) -> StarboardResult<bool> {
        let guild_id = self.config.starboard.guild_id.into_id();
        let sb_channel_id = self.config.starboard.channel_id.into_id();

        let is_forum = bot.cache.is_channel_forum(guild_id, sb_channel_id);
        let real_channel_id = if is_forum {
            message_id.get().into_id()
        } else {
            sb_channel_id
        };

        let Some(msg) = bot.cache.fog_message(bot, real_channel_id, message_id).await? else {
            return Ok(true);
        };

        let (wh, is_thread) = if msg.author_id.get() != bot.config.bot_id {
            if Some(msg.author_id.get_i64()) != self.config.starboard.webhook_id {
                return Ok(false);
            }

            let wh = get_valid_webhook(bot, &self.config.starboard, false).await?;

            let parent = bot
                .cache
                .fog_parent_channel_id(bot, guild_id, sb_channel_id)
                .await?
                .unwrap();

            (wh, sb_channel_id != parent)
        } else {
            (None, false)
        };

        match self.build(force_partial, wh.is_some()) {
            BuiltStarboardEmbed::Full(built) => {
                if let Some(wh) = wh {
                    let mut ud = bot
                        .http
                        .update_webhook_message(wh.id, wh.token.as_ref().unwrap(), message_id)
                        .content(Some(&built.top_content))?
                        .embeds(Some(&built.embeds))?;

                    if is_thread || is_forum {
                        ud = ud.thread_id(real_channel_id);
                    }

                    ud.await?;
                } else {
                    bot.http
                        .update_message(real_channel_id, message_id)
                        .content(Some(&built.top_content))?
                        .embeds(Some(&built.embeds))?
                        .await?;
                }
            }
            BuiltStarboardEmbed::Partial(built) => {
                if let Some(wh) = wh {
                    let mut ud = bot
                        .http
                        .update_webhook_message(wh.id, wh.token.as_ref().unwrap(), message_id)
                        .content(Some(&built.top_content))?;

                    if is_thread || is_forum {
                        ud = ud.thread_id(real_channel_id);
                    }

                    ud.await?;
                } else {
                    bot.http
                        .update_message(real_channel_id, message_id)
                        .content(Some(&built.top_content))?
                        .await?;
                }
            }
        };

        Ok(false)
    }

    pub async fn delete(
        &self,
        bot: &StarboardBot,
        message_id: Id<MessageMarker>,
    ) -> StarboardResult<()> {
        let sb_channel_id = self.config.starboard.channel_id.into_id();

        let is_forum = bot
            .cache
            .is_channel_forum(self.config.starboard.guild_id.into_id(), sb_channel_id);
        let real_channel_id = if is_forum {
            message_id.get().into_id()
        } else {
            sb_channel_id
        };

        let Some(msg) = bot.cache.fog_message(bot, real_channel_id, message_id).await? else {
            return Ok(());
        };

        if let Some(wh_id) = self.config.starboard.webhook_id {
            if wh_id == msg.author_id.get_i64() {
                if let Some(wh) = get_valid_webhook(bot, &self.config.starboard, false).await? {
                    let parent = bot
                        .cache
                        .fog_parent_channel_id(
                            bot,
                            self.config.starboard.guild_id.into_id(),
                            sb_channel_id,
                        )
                        .await?
                        .unwrap();

                    let mut ud = bot.http.delete_webhook_message(
                        wh.id,
                        wh.token.as_ref().unwrap(),
                        message_id,
                    );

                    if parent != sb_channel_id || is_forum {
                        ud = ud.thread_id(real_channel_id);
                    }

                    let ret = ud.await;
                    if ret.is_ok() {
                        return Ok(());
                    }
                }
            }
        }

        let _ = bot.http.delete_message(real_channel_id, message_id).await;

        Ok(())
    }
}
