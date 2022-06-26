use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::core::emoji::{EmojiCommon, SimpleEmoji};
use crate::interactions::commands::context::CommandCtx;
use crate::models::AutoStarChannel;
use crate::utils::embed;
use crate::{concat_format, get_guild_id};

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
                let asc_settings = concat_format!(
                    "channel: <#{}>\n" <- asc.channel_id;
                    "emojis: {}\n" <- Vec::<SimpleEmoji>::from_stored(asc.emojis).into_readable(&ctx.bot, guild_id).await;
                    "min-chars: {}\n" <- asc.min_chars;
                    "max-chars: {}\n" <- asc.max_chars.map(|v| v.to_string()).unwrap_or("none".to_string());
                    "require-image: {}\n" <- asc.require_image;
                    "delete-invalid: {}" <- asc.delete_invalid;
                );

                let emb = embed::build()
                    .title(format!("Autostar Channel '{name}'"))
                    .description(asc_settings)
                    .build();

                let resp = ctx.build_resp().embeds([emb]).build();

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

            let mut desc = String::new();
            for a in asc.into_iter() {
                desc.push_str(&format!(
                    "'{}' in <#{}>: {}\n",
                    a.name,
                    a.channel_id,
                    Vec::<SimpleEmoji>::from_stored(a.emojis)
                        .into_readable(&ctx.bot, guild_id)
                        .await
                ));
            }
            let emb = embed::build()
                .title("Autostar Channels")
                .description(desc)
                .build();
            let resp = ctx.build_resp().embeds([emb]).build();

            ctx.respond(resp).await?;
        }

        Ok(())
    }
}
