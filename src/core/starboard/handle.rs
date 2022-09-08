use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    client::bot::StarboardBot,
    core::embedder::Embedder,
    database::{Message as DbMessage, StarboardMessage, Vote},
    errors::StarboardResult,
    unwrap_id,
    utils::get_status::get_status,
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
    configs: Option<Arc<Vec<StarboardConfig>>>,
}

impl RefreshMessage<'_> {
    pub fn new<'bot>(
        bot: &'bot StarboardBot,
        message_id: Id<MessageMarker>,
    ) -> RefreshMessage<'bot> {
        RefreshMessage {
            bot,
            message_id,
            configs: None,
            sql_message: None,
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

        std::mem::drop(guard);
        Ok(())
    }

    // caching methods
    pub fn set_configs(&mut self, configs: Vec<StarboardConfig>) {
        self.configs.replace(Arc::new(configs));
    }

    async fn get_configs(&mut self) -> sqlx::Result<Arc<Vec<StarboardConfig>>> {
        if self.configs.is_none() {
            let msg = self.get_sql_message().await?;
            let guild_id = Id::new(msg.guild_id.try_into().unwrap());
            let channel_id = Id::new(msg.channel_id.try_into().unwrap());

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

        let embedder = Embedder::new(points, &self.config);
        let sb_msg = self.get_starboard_message().await?;

        let action = get_message_status(&self.refresh.bot, &self.config, &orig, points).await?;

        if let Some(sb_msg) = sb_msg {
            if points == sb_msg.last_known_point_count as i32 {
                return Ok(false);
            } else {
                StarboardMessage::set_last_point_count(
                    &self.refresh.bot.pool,
                    sb_msg.starboard_message_id,
                    points.try_into().unwrap(),
                )
                .await?;
            }

            let (ret, retry_on_err, delete_on_ok) = match action {
                MessageStatus::Remove => {
                    let ret = self
                        .refresh
                        .bot
                        .http
                        .delete_message(
                            Id::new(self.config.starboard.channel_id.try_into().unwrap()),
                            Id::new(sb_msg.starboard_message_id.try_into().unwrap()),
                        )
                        .exec()
                        .await;
                    (ret.map(|_| ()), false, true)
                }
                MessageStatus::Send | MessageStatus::NoAction => {
                    let ret = embedder
                        .edit(
                            &self.refresh.bot,
                            Id::new(sb_msg.starboard_message_id.try_into().unwrap()),
                            false,
                        )
                        .await;
                    (ret.map(|_| ()), true, false)
                }
                MessageStatus::Trash => {
                    let ret = embedder
                        .edit(
                            &self.refresh.bot,
                            Id::new(sb_msg.starboard_message_id.try_into().unwrap()),
                            true,
                        )
                        .await;
                    (ret.map(|_| ()), false, false)
                }
            };

            if let Err(why) = ret {
                if matches!(get_status(&why), Some(404)) {
                    StarboardMessage::delete(&self.refresh.bot.pool, sb_msg.starboard_message_id)
                        .await?;
                    return Ok(retry_on_err);
                } else {
                    return Err(why.into());
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
                .send(&self.refresh.bot)
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
