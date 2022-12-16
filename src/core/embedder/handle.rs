use std::sync::Arc;

use twilight_model::id::{marker::MessageMarker, Id};

use crate::{
    cache::models::{message::CachedMessage, user::CachedUser},
    client::bot::StarboardBot,
    core::starboard::config::StarboardConfig,
    database::Message as DbMessage,
    utils::into_id::IntoId,
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
    pub fn build(&self, force_partial: bool) -> BuiltStarboardEmbed {
        BuiltStarboardEmbed::build(self, force_partial)
    }

    pub async fn send(
        &self,
        bot: &StarboardBot,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        let built = match self.build(false) {
            BuiltStarboardEmbed::Full(built) => built,
            BuiltStarboardEmbed::Partial(_) => panic!("Tried to send an unbuildable message."),
        };
        let (attachments, errors) = built.upload_attachments.as_attachments(bot).await;

        for e in errors {
            bot.handle_error(&e).await;
        }

        let ret = bot
            .http
            .create_message(self.config.starboard.channel_id.into_id())
            .content(&built.top_content)
            .unwrap()
            .embeds(&built.embeds)
            .unwrap()
            .attachments(&attachments);

        if let Err(why) = &ret {
            dbg!(why);
        }

        ret.unwrap().await
    }

    pub async fn edit(
        &self,
        bot: &StarboardBot,
        message_id: Id<MessageMarker>,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        match self.build(!self.config.resolved.link_edits) {
            BuiltStarboardEmbed::Full(built) => {
                bot.http
                    .update_message(self.config.starboard.channel_id.into_id(), message_id)
                    .content(Some(&built.top_content))
                    .unwrap()
                    .embeds(Some(&built.embeds))
                    .unwrap()
                    .await
            }
            BuiltStarboardEmbed::Partial(built) => {
                bot.http
                    .update_message(self.config.starboard.channel_id.into_id(), message_id)
                    .content(Some(&built.top_content))
                    .unwrap()
                    .await
            }
        }
    }
}
