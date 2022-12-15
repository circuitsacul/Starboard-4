use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder};

use crate::{
    core::starboard::config::StarboardConfig,
    database::{Starboard, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::format_settings::format_settings, context::CommandCtx},
    utils::{embed, id_as_i64::GetI64},
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your overrides.")]
pub struct ViewOverride {
    /// The name of the override to view. Leave blank to show all.
    #[command(autocomplete = true)]
    name: Option<String>,
}

impl ViewOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();

        if let Some(name) = self.name {
            let ov = StarboardOverride::get(&ctx.bot.pool, guild_id_i64, &name).await?;
            let ov = match ov {
                None => {
                    ctx.respond_str("No override with that name was found.", true)
                        .await?;
                    return Ok(());
                }
                Some(ov) => ov,
            };
            let sb = Starboard::get(&ctx.bot.pool, ov.starboard_id)
                .await?
                .unwrap();

            let channels: Vec<_> = ov.channel_ids.iter().map(|id| format!("<#{id}>")).collect();
            let channels = channels.join(", ");
            let config = StarboardConfig::new(sb, vec![ov])?;
            let pretty = format_settings(&ctx.bot, guild_id, &config);

            let embed = embed::build()
                .title(format!("Override '{}'", &name))
                .description(format!(
                    "This override applies to the following channels: {channels}"
                ))
                .field(
                    EmbedFieldBuilder::new("Requirements", pretty.requirements)
                        .inline()
                        .build(),
                )
                .field(
                    EmbedFieldBuilder::new("Behaviour", pretty.behavior)
                        .inline()
                        .build(),
                )
                .field(
                    EmbedFieldBuilder::new("Style", pretty.style)
                        .inline()
                        .build(),
                )
                .field(
                    EmbedFieldBuilder::new("Embed Style", pretty.embed)
                        .inline()
                        .build(),
                )
                .build();

            ctx.respond(ctx.build_resp().embeds([embed]).build())
                .await?;
        } else {
            let overrides = StarboardOverride::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;
            if overrides.is_empty() {
                ctx.respond_str("This server has no overrides.", true)
                    .await?;
            } else {
                let mut final_result = String::new();

                for ov in overrides {
                    writeln!(
                        final_result,
                        "override '{}' in {} channel(s)",
                        ov.name,
                        ov.channel_ids.len()
                    )
                    .unwrap();
                }

                let embed = embed::build()
                    .title("Overrides")
                    .description(final_result)
                    .footer(
                        EmbedFooterBuilder::new(concat!(
                            "Run '/overrides view' with a specific override's ",
                            "name to show all its settings"
                        ))
                        .build(),
                    )
                    .build();

                ctx.respond(ctx.build_resp().embeds([embed]).build())
                    .await?;
            }
        }

        Ok(())
    }
}
