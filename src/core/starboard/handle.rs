use std::sync::Arc;

use twilight_http::error::ErrorType;
use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    client::bot::StarboardBot,
    core::embedder::Embedder,
    database::{Message as DbMessage, StarboardMessage, Vote},
    errors::StarboardResult,
    unwrap_id,
};

use super::config::StarboardConfig;

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
        let orig = self.refresh.get_sql_message().await?;
        let points = Vote::count(
            &self.refresh.bot.pool,
            orig.message_id,
            self.config.starboard.id,
        )
        .await?;

        let embedder = Embedder::new(points, &self.config);
        let sb_msg = self.get_starboard_message().await?;

        if let Some(sb_msg) = sb_msg {
            if points == sb_msg.last_known_point_count as i32 {
                return Ok(());
            } else {
                StarboardMessage::set_last_point_count(
                    &self.refresh.bot.pool,
                    sb_msg.starboard_message_id.unwrap(),
                    points.try_into().unwrap(),
                )
                .await?;
            }

            let ret = embedder
                .edit(
                    &self.refresh.bot,
                    Id::new(sb_msg.starboard_message_id.unwrap().try_into().unwrap()),
                )
                .await;

            if let Err(why) = ret {
                let mut was_404 = false;
                if let ErrorType::Response {
                    body: _,
                    error: _,
                    status,
                } = why.kind()
                {
                    if status.get() == 404 {
                        was_404 = true;
                        StarboardMessage::delete(
                            &self.refresh.bot.pool,
                            sb_msg.starboard_message_id.unwrap(),
                        )
                        .await?;
                    }
                }

                if !was_404 {
                    return Err(why.into());
                }
            } else {
                return Ok(());
            }
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

        Ok(())
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
