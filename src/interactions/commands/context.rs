use std::sync::Arc;

use twilight_http::{response::marker::EmptyBody, Response};
use twilight_model::{
    application::interaction::ApplicationCommand,
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::client::bot::StarboardBot;

#[derive(Debug)]
pub struct CommandCtx {
    pub shard_id: u64,
    pub bot: Arc<StarboardBot>,
    pub command: Box<ApplicationCommand>,
}

impl CommandCtx {
    pub async fn respond(
        &self,
        response: InteractionResponseData,
        defer: bool,
    ) -> anyhow::Result<Response<EmptyBody>> {
        let i = self.bot.interaction_client().await?;

        i.create_response(
            self.command.id,
            &self.command.token,
            &InteractionResponse {
                data: Some(response),
                kind: match defer {
                    false => InteractionResponseType::ChannelMessageWithSource,
                    true => InteractionResponseType::DeferredChannelMessageWithSource,
                },
            },
        )
        .exec()
        .await
        .map_err(|e| e.into())
    }

    pub async fn respond_str(
        &self,
        response: &str,
        ephemeral: bool,
    ) -> anyhow::Result<Response<EmptyBody>> {
        let mut data = InteractionResponseDataBuilder::new().content(response.into());
        if ephemeral {
            data = data.flags(MessageFlags::EPHEMERAL);
        }

        self.respond(data.build(), false).await
    }

    pub async fn defer(&self, ephemeral: bool) -> anyhow::Result<Response<EmptyBody>> {
        let mut data = InteractionResponseDataBuilder::new();
        if ephemeral {
            data = data.flags(MessageFlags::EPHEMERAL);
        }

        self.respond(data.build(), true).await
    }
}
