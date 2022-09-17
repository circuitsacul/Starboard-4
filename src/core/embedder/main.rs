use twilight_model::id::{marker::MessageMarker, Id};

use crate::client::bot::StarboardBot;

use super::Embedder;

impl Embedder<'_> {
    pub fn get_top_text(&self, trashed: bool) -> String {
        let point_str = self.points.to_string();
        if trashed {
            format!("trashed message {}", point_str)
        } else {
            point_str
        }
    }

    pub async fn send(
        &self,
        bot: &StarboardBot,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        bot.http
            .create_message(Id::new(self.config.starboard.channel_id as u64))
            .content(&self.get_top_text(false))
            .unwrap()
            .exec()
            .await
    }

    pub async fn edit(
        &self,
        bot: &StarboardBot,
        message_id: Id<MessageMarker>,
        trashed: bool,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        bot.http
            .update_message(Id::new(self.config.starboard.channel_id as u64), message_id)
            .content(Some(&self.get_top_text(trashed)))
            .unwrap()
            .exec()
            .await
    }
}
