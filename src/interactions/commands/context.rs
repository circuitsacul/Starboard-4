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
    pub bot: StarboardBot,
    pub interaction: Box<ApplicationCommand>,
}

impl CommandCtx {
    pub fn build_resp(&self) -> InteractionResponseDataBuilder {
        InteractionResponseDataBuilder::new()
    }

    pub async fn raw_respond(
        &self,
        data: Option<InteractionResponseData>,
        kind: InteractionResponseType,
    ) -> anyhow::Result<Response<EmptyBody>> {
        let i = self.bot.interaction_client().await?;

        i.create_response(
            self.interaction.id,
            &self.interaction.token,
            &InteractionResponse { data, kind },
        )
        .exec()
        .await
        .map_err(|e| e.into())
    }

    pub async fn defer(&self, ephemeral: bool) -> anyhow::Result<Response<EmptyBody>> {
        let mut data = self.build_resp();
        if ephemeral {
            data = data.flags(MessageFlags::EPHEMERAL);
        }

        self.raw_respond(
            Some(data.build()),
            InteractionResponseType::DeferredChannelMessageWithSource,
        )
        .await
    }

    pub async fn respond(
        &self,
        data: InteractionResponseData,
    ) -> anyhow::Result<Response<EmptyBody>> {
        self.raw_respond(
            Some(data),
            InteractionResponseType::ChannelMessageWithSource,
        )
        .await
    }

    pub async fn respond_str(
        &self,
        response: &str,
        ephemeral: bool,
    ) -> anyhow::Result<Response<EmptyBody>> {
        let mut data = self.build_resp().content(response.into());
        if ephemeral {
            data = data.flags(MessageFlags::EPHEMERAL);
        }

        self.raw_respond(
            Some(data.build()),
            InteractionResponseType::ChannelMessageWithSource,
        )
        .await
    }
}
