use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    concat_format,
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::AutoStarChannel,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{embed, id_as_i64::GetI64},
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your autostar channels.")]
pub struct ViewAutoStarChannels {
    /// The name of the autostar channel to view. Leave blank to show all.
    #[command(autocomplete = true)]
    name: Option<String>,
}

impl ViewAutoStarChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();

        if let Some(name) = &self.name {
            let asc = AutoStarChannel::get_by_name(&ctx.bot.pool, name, guild_id_i64).await?;

            if let Some(asc) = asc {
                let emojis =
                    Vec::<SimpleEmoji>::from_stored(asc.emojis).into_readable(&ctx.bot, guild_id);
                let max_chars = asc
                    .max_chars
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "none".to_string());

                let note = if asc.premium_locked {
                    concat!(
                        "This autostar channel is locked because it exceeds the non-premium ",
                        "limit.\n\n"
                    )
                } else {
                    ""
                };

                let asc_settings = concat_format!(
                    "{}" <- note;
                    "This autostar channel is in <#{}>.\n\n" <- asc.channel_id;
                    "emojis: {}\n" <- emojis;
                    "min-chars: {}\n" <- asc.min_chars;
                    "max-chars: {}\n" <- max_chars;
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
            let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

            if asc.is_empty() {
                ctx.respond_str("This server has no autostar channels.", true)
                    .await?;
                return Ok(());
            }

            let mut desc = String::new();
            for a in asc.into_iter() {
                write!(
                    desc,
                    "'{}' in <#{}>: {}",
                    a.name,
                    a.channel_id,
                    Vec::<SimpleEmoji>::from_stored(a.emojis).into_readable(&ctx.bot, guild_id),
                )
                .unwrap();
                if a.premium_locked {
                    write!(desc, " (premium-locked)").unwrap();
                }
                writeln!(desc).unwrap();
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
