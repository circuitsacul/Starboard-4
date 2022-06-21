use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::interactions::commands::command::AppCommand;
use crate::interactions::commands::context::CommandCtx;

#[derive(CreateCommand, CommandModel)]
#[command(name = "ping", desc = "Pong!")]
pub struct Ping {}

#[async_trait]
impl AppCommand for Ping {
    async fn callback(&self, ctx: CommandCtx) {
        let i = ctx.bot.interaction_client().await.unwrap();
        let resp = InteractionResponseDataBuilder::new()
            .content("Pong!".into())
            .build();
        let _ = i
            .create_response(
                ctx.command.id,
                &ctx.command.token,
                &InteractionResponse {
                    data: Some(resp),
                    kind: InteractionResponseType::ChannelMessageWithSource,
                },
            )
            .exec()
            .await;
    }
}
