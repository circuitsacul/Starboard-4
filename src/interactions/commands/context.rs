use std::sync::Arc;

use anyhow::Result;
use twilight_http::{Response, response::marker::EmptyBody};
use twilight_model::{
    application::interaction::ApplicationCommand, http::interaction::{InteractionResponseData, InteractionResponse, InteractionResponseType},
};

use crate::client::bot::Starboard;

#[derive(Debug)]
pub struct CommandCtx {
    pub shard_id: u64,
    pub bot: Arc<Starboard>,
    pub command: Box<ApplicationCommand>,
}

impl CommandCtx {
    pub async fn respond(&self, response: InteractionResponseData) -> Result<Response<EmptyBody>> {
        let i = self.bot.interaction_client().await?;

        i.create_response(
            self.command.id,
            &self.command.token,
            &InteractionResponse {
                data: Some(response),
                kind: InteractionResponseType::ChannelMessageWithSource,
            },
        ).exec().await.map_err(|e| e.into())
    }
}
