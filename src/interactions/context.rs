use std::sync::Arc;

use twilight_http::Response;
use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData,
        Interaction,
    },
    channel::{message::MessageFlags, Message},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::client::bot::StarboardBot;

pub type CommandCtx = Ctx<Box<CommandData>>;
pub type ComponentCtx = Ctx<MessageComponentInteractionData>;

#[derive(Debug)]
pub struct Ctx<T> {
    pub shard_id: u64,
    pub bot: Arc<StarboardBot>,
    pub interaction: Interaction,
    pub data: T,
    responded: bool,
}

type TwResult = Result<Response<Message>, twilight_http::Error>;

impl<T> Ctx<T> {
    pub fn new(shard_id: u64, bot: Arc<StarboardBot>, interaction: Interaction, data: T) -> Self {
        Self {
            shard_id,
            bot,
            interaction,
            data,
            responded: false,
        }
    }

    pub fn build_resp(&self) -> InteractionResponseDataBuilder {
        InteractionResponseDataBuilder::new()
    }

    pub async fn raw_respond(
        &mut self,
        data: Option<InteractionResponseData>,
        kind: InteractionResponseType,
    ) -> TwResult {
        let i = self.bot.interaction_client().await;

        if self.responded {
            let mut followup = i.create_followup(&self.interaction.token);
            let data = match data {
                None => panic!("cannot followup without data"),
                Some(data) => data,
            };

            if let Some(mentions) = &data.allowed_mentions {
                followup = followup.allowed_mentions(Some(mentions));
            }
            if let Some(attachments) = &data.attachments {
                followup = followup.attachments(attachments).unwrap();
            }
            if let Some(components) = &data.components {
                followup = followup.components(components).unwrap();
            }
            if let Some(content) = &data.content {
                followup = followup.content(content).unwrap();
            }
            if let Some(embeds) = &data.embeds {
                followup = followup.embeds(embeds).unwrap();
            }
            if let Some(flags) = data.flags {
                followup = followup.flags(flags);
            }
            if let Some(tts) = data.tts {
                followup = followup.tts(tts);
            }

            followup.exec().await
        } else {
            i.create_response(
                self.interaction.id,
                &self.interaction.token,
                &InteractionResponse { data, kind },
            )
            .exec()
            .await?;

            self.responded = true;

            i.response(&self.interaction.token).exec().await
        }
    }

    pub async fn defer(&mut self, ephemeral: bool) -> TwResult {
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

    pub async fn respond(&mut self, data: InteractionResponseData) -> TwResult {
        self.raw_respond(
            Some(data),
            InteractionResponseType::ChannelMessageWithSource,
        )
        .await
    }

    pub async fn respond_str(&mut self, response: &str, ephemeral: bool) -> TwResult {
        let mut data = self.build_resp().content(response);
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
