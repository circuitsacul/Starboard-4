use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    cache::models::message::CachedMessage,
    client::bot::StarboardBot,
    core::{
        embedder::Embedder,
        emoji::{EmojiCommon, SimpleEmoji},
    },
    database::{Message as DbMessage, StarboardMessage, Vote},
    errors::StarboardResult,
    unwrap_id,
    utils::{get_status::get_status, into_id::IntoId},
};

use super::{
    config::StarboardConfig,
    msg_status::{get_message_status, MessageStatus},
};

pub struct RefreshMessage<'bot> {
    bot: &'bot StarboardBot,
    /// The id of the inputted message. May or may not be the original.
    message_id: Id<MessageMarker>,
    sql_message: Option<Arc<DbMessage>>,
    orig_message: Option<Option<Arc<CachedMessage>>>,
    configs: Option<Arc<Vec<StarboardConfig>>>,
}

impl RefreshMessage<'_> {
    pub fn new(bot: &StarboardBot, message_id: Id<MessageMarker>) -> RefreshMessage {
        RefreshMessage {
            bot,
            message_id,
            configs: None,
            sql_message: None,
            orig_message: None,
        }
    }

    pub async fn refresh(&mut self) -> StarboardResult<()> {
        let orig = self.get_sql_message().await?;
        let guard = self.bot.locks.post_update_lock.lock(orig.message_id);
        if guard.is_none() {
            return Ok(());
        }

        let configs = self.get_configs().await?;
        for c in configs.iter() {
            RefreshStarboard::new(self, c).refresh().await?;
        }

        Ok(())
    }

    // caching methods
    pub fn set_configs(&mut self, configs: Vec<StarboardConfig>) {
        self.configs.replace(Arc::new(configs));
    }

    async fn get_configs(&mut self) -> sqlx::Result<Arc<Vec<StarboardConfig>>> {
        if self.configs.is_none() {
            let msg = self.get_sql_message().await?;
            let guild_id = msg.guild_id.into_id();
            let channel_id = msg.channel_id.into_id();

            let configs = StarboardConfig::list_for_channel(self.bot, guild_id, channel_id)
                .await
                .unwrap();
            self.set_configs(configs);
        }

        Ok(self.configs.as_ref().unwrap().clone())
    }

    pub fn set_sql_message(&mut self, message: DbMessage) {
        self.sql_message.replace(Arc::new(message));
    }

    async fn get_sql_message(&mut self) -> sqlx::Result<Arc<DbMessage>> {
        if self.sql_message.is_none() {
            let sql_message =
                DbMessage::get_original(&self.bot.pool, unwrap_id!(self.message_id)).await?;
            self.set_sql_message(sql_message.unwrap());
        }

        Ok(self.sql_message.as_ref().unwrap().clone())
    }

    pub fn set_orig_message(&mut self, message: Option<Arc<CachedMessage>>) {
        self.orig_message.replace(message);
    }

    async fn get_orig_message(&mut self) -> StarboardResult<Option<Arc<CachedMessage>>> {
        if self.orig_message.is_none() {
            let sql_message = self.get_sql_message().await?;
            let orig_message = self
                .bot
                .cache
                .fog_message(
                    self.bot,
                    sql_message.channel_id.into_id(),
                    sql_message.message_id.into_id(),
                )
                .await?;

            self.set_orig_message(orig_message);
        }

        Ok(self.orig_message.as_ref().unwrap().clone())
    }
}

struct RefreshStarboard<'this, 'bot> {
    refresh: &'this mut RefreshMessage<'bot>,
    config: &'this StarboardConfig,
}

impl<'this, 'bot> RefreshStarboard<'this, 'bot> {
    pub fn new(refresh: &'this mut RefreshMessage<'bot>, config: &'this StarboardConfig) -> Self {
        Self { refresh, config }
    }

    pub async fn refresh(&mut self) -> StarboardResult<()> {
        // I use a loop because recursion inside async functions requires another crate :(
        let mut tries = 0;
        loop {
            if tries == 2 {
                return Ok(());
            }
            tries += 1;
            let retry = self.refresh_one().await?;
            match retry {
                true => continue,
                false => return Ok(()),
            }
        }
    }

    async fn refresh_one(&mut self) -> StarboardResult<bool> {
        let orig = self.refresh.get_sql_message().await?;
        let points = Vote::count(
            &self.refresh.bot.pool,
            orig.message_id,
            self.config.starboard.id,
        )
        .await?;

        let orig_message = self.refresh.get_orig_message().await?;
        let sql_message = self.refresh.get_sql_message().await?;
        let orig_message_author = self
            .refresh
            .bot
            .cache
            .fog_user(self.refresh.bot, sql_message.author_id.into_id())
            .await?;
        let (ref_msg, ref_msg_author) = if let Some(msg) = &orig_message {
            if let Some(id) = msg.referenced_message {
                let ref_msg = self
                    .refresh
                    .bot
                    .cache
                    .fog_message(self.refresh.bot, sql_message.channel_id.into_id(), id)
                    .await?;

                let ref_msg_author = match &ref_msg {
                    None => None,
                    Some(ref_msg) => Some(
                        self.refresh
                            .bot
                            .cache
                            .fog_user(self.refresh.bot, ref_msg.author_id)
                            .await?,
                    ),
                };

                (ref_msg, ref_msg_author.flatten())
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let embedder = Embedder::new(
            points,
            self.config,
            orig_message,
            orig_message_author,
            ref_msg,
            ref_msg_author,
            sql_message,
        );
        let sb_msg = self.get_starboard_message().await?;

        let action = get_message_status(self.refresh.bot, self.config, &orig, points).await?;

        if let Some(sb_msg) = sb_msg {
            if points == sb_msg.last_known_point_count as i32 {
                return Ok(false);
            }
            StarboardMessage::set_last_point_count(
                &self.refresh.bot.pool,
                sb_msg.starboard_message_id,
                points as i16,
            )
            .await?;

            let (ret, retry_on_err, delete_on_ok) = match action {
                MessageStatus::Remove => {
                    let ret = self
                        .refresh
                        .bot
                        .http
                        .delete_message(
                            self.config.starboard.channel_id.into_id(),
                            sb_msg.starboard_message_id.into_id(),
                        )
                        .await;
                    (ret.map(|_| ()), false, true)
                }
                MessageStatus::Send | MessageStatus::NoAction | MessageStatus::Trash => {
                    let ret = embedder
                        .edit(self.refresh.bot, sb_msg.starboard_message_id.into_id())
                        .await;
                    (ret.map(|_| ()), true, false)
                }
            };

            if let Err(why) = ret {
                if matches!(get_status(&why), Some(404)) {
                    StarboardMessage::delete(&self.refresh.bot.pool, sb_msg.starboard_message_id)
                        .await?;
                    Ok(retry_on_err)
                } else {
                    Err(why.into())
                }
            } else {
                if delete_on_ok {
                    StarboardMessage::delete(&self.refresh.bot.pool, sb_msg.starboard_message_id)
                        .await?;
                }
                Ok(false)
            }
        } else {
            match action {
                MessageStatus::Remove | MessageStatus::Trash | MessageStatus::NoAction => {
                    return Ok(false)
                }
                MessageStatus::Send => {}
            }

            let msg = embedder
                .send(self.refresh.bot)
                .await?
                .model()
                .await
                .unwrap();
            StarboardMessage::create(
                &self.refresh.bot.pool,
                orig.message_id,
                unwrap_id!(msg.id),
                self.config.starboard.id,
                points,
            )
            .await?;

            let mut to_react: Vec<SimpleEmoji> = Vec::new();
            if self.config.resolved.autoreact_upvote {
                to_react.extend(Vec::<SimpleEmoji>::from_stored(
                    self.config.resolved.upvote_emojis.clone(),
                ));
            }
            if self.config.resolved.autoreact_downvote {
                to_react.extend(Vec::<SimpleEmoji>::from_stored(
                    self.config.resolved.downvote_emojis.clone(),
                ));
            }

            for emoji in to_react {
                let _ = self
                    .refresh
                    .bot
                    .http
                    .create_reaction(msg.channel_id, msg.id, &emoji.reactable())
                    .await;
            }

            Ok(false)
        }
    }

    async fn get_starboard_message(&mut self) -> sqlx::Result<Option<StarboardMessage>> {
        let orig = self.refresh.get_sql_message().await?;
        StarboardMessage::get_by_starboard(
            &self.refresh.bot.pool,
            orig.message_id,
            self.config.starboard.id,
        )
        .await
    }
}
