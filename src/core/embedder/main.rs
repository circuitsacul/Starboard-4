use twilight_model::id::Id;

use crate::client::bot::StarboardBot;

use super::Embedder;

impl Embedder<'_> {
    pub fn get_top_text(&self) -> String {
        "Hello".to_string()
    }

    pub async fn send(
        &self,
        bot: &StarboardBot,
    ) -> Result<twilight_http::Response<twilight_model::channel::Message>, twilight_http::Error>
    {
        bot.http
            .create_message(Id::new(
                self.config.starboard.channel_id.try_into().unwrap(),
            ))
            .content(&self.get_top_text())
            .unwrap()
            .exec()
            .await
    }
}
