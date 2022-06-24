use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::get_guild_id;
use crate::interactions::commands::context::CommandCtx;
use crate::models::AutoStarChannel;
use crate::utils::embed;

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your autostar channels.")]
pub struct ViewAutoStarChannels {
    /// The name of the autostar channel to view. Leave blank to show all.
    name: Option<String>,
}

impl ViewAutoStarChannels {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(name) = &self.name {
            let asc = AutoStarChannel::get_by_name(&ctx.bot.pool, name, guild_id).await?;

            if let Some(asc) = asc {
                let resp = ctx
                    .build_resp()
                    .embeds([embed::build().description(format!("{:#?}", asc)).build()])
                    .build();

                ctx.respond(resp).await?;
            } else {
                ctx.respond_str("No autostar channels with that name were found.", true)
                    .await?;
            }
        } else {
            let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id).await?;

            if asc.len() == 0 {
                ctx.respond_str("This server has no autostar channels.", true)
                    .await?;
                return Ok(());
            }

            let resp = ctx
                .build_resp()
                .embeds([embed::build().description(format!("{:#?}", asc)).build()])
                .build();

            ctx.respond(resp).await?;
        }

        Ok(())
    }
}
