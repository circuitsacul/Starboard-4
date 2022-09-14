use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    cache::models::message::CachedMessage, client::bot::StarboardBot,
    core::starboard::config::StarboardConfig, database::Message as DbMessage,
};

use super::builder::BuiltStarboardEmbed;

pub struct Embedder<'config> {
    pub points: i32,
    pub config: &'config StarboardConfig,
    pub orig_message: Arc<Option<CachedMessage>>,
    pub orig_sql_message: Arc<DbMessage>,
}

impl<'config> Embedder<'config> {
    pub fn new(
        points: i32,
        config: &'config StarboardConfig,
        orig_message: Arc<Option<CachedMessage>>,
        orig_sql_message: Arc<DbMessage>,
    ) -> Self {
        Self {
            points,
            config,
            orig_message,
            orig_sql_message,
        }
    }
}

impl Embedder<'_> {
    fn build(&self) -> BuiltStarboardEmbed {
        BuiltStarboardEmbed::build(&self)
    }

    pub async fn send(
        &self,
        bot: &StarboardBot,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        let built = match self.build() {
            BuiltStarboardEmbed::Full(built) => built,
            BuiltStarboardEmbed::Partial(_) => panic!("Tried to send an unbuildable message."),
        };

        bot.http
            .create_message(Id::new(
                self.config.starboard.channel_id.try_into().unwrap(),
            ))
            .content(&built.top_content)
            .unwrap()
            .embeds(&built.embeds)
            .unwrap()
            .attachments(&built.upload_attachments)
            .unwrap()
            .exec()
            .await
    }

    pub async fn edit(
        &self,
        bot: &StarboardBot,
        message_id: Id<MessageMarker>,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        match self.build() {
            BuiltStarboardEmbed::Full(built) => {
                bot.http
                    .update_message(
                        Id::new(self.config.starboard.channel_id.try_into().unwrap()),
                        message_id,
                    )
                    .content(Some(&built.top_content))
                    .unwrap()
                    .embeds(Some(&built.embeds))
                    .unwrap()
                    .attachments(&built.upload_attachments)
                    .unwrap()
                    .exec()
                    .await
            }
            BuiltStarboardEmbed::Partial(built) => {
                bot.http
                    .update_message(
                        Id::new(self.config.starboard.channel_id.try_into().unwrap()),
                        message_id,
                    )
                    .content(Some(&built.top_content))
                    .unwrap()
                    .exec()
                    .await
            }
        }
    }
}
