use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    client::bot::StarboardBot,
    core::embedder::Embedder,
    database::{Message as DbMessage, Vote},
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
    refresh: &'this RefreshMessage<'bot>,
    config: &'this StarboardConfig,
}

impl<'this, 'bot> RefreshStarboard<'this, 'bot> {
    pub fn new(refresh: &'this RefreshMessage<'bot>, config: &'this StarboardConfig) -> Self {
        Self { refresh, config }
    }

    pub async fn refresh(&self) -> StarboardResult<()> {
        let points = Vote::count(
            &self.refresh.bot.pool,
            unwrap_id!(self.refresh.message_id),
            self.config.starboard.id,
        )
        .await?;
        let embedder = Embedder::new(points, &self.config);
        embedder.send(&self.refresh.bot).await?;
        Ok(())
    }
}
