use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder};

use crate::{
    core::starboard::config::StarboardConfig,
    database::Starboard,
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::format_settings::format_settings, context::CommandCtx},
    utils::{embed, id_as_i64::GetI64},
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your starboards.")]
pub struct ViewStarboard {
    /// The name of the starboard to view. Leave blank to show all.
    #[command(autocomplete = true)]
    name: Option<String>,
}

impl ViewStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(name) = self.name {
            let starboard =
                Starboard::get_by_name(&ctx.bot.pool, &name, guild_id.get_i64()).await?;

            if let Some(starboard) = starboard {
                let config = StarboardConfig::new(starboard, &[], vec![])?;
                let pretty = format_settings(&ctx.bot, guild_id, &config).await?;

                let mut desc = String::new();
                if config.starboard.premium_locked {
                    desc.push_str(concat!(
                        "This starboard is locked because it exceeds the non-premium limit.\n\n"
                    ));
                }
                write!(
                    desc,
                    "This starboard is in <#{}>.",
                    config.starboard.channel_id
                )
                .unwrap();

                let embed = embed::build()
                    .title(format!("Starboard '{}'", &config.starboard.name))
                    .description(desc)
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
                ctx.respond_str("No starboard with that name was found.", true)
                    .await?;
            }
        } else {
            let starboards = Starboard::list_by_guild(&ctx.bot.pool, guild_id.get_i64()).await?;
            if starboards.is_empty() {
                ctx.respond_str("This server has no starboards.", true)
                    .await?;
            } else {
                let mut final_result = String::new();

                for sb in starboards {
                    write!(final_result, "'{}' in <#{}>", sb.name, sb.channel_id).unwrap();
                    if sb.premium_locked {
                        write!(final_result, " (premium-locked)").unwrap();
                    }
                    writeln!(final_result).unwrap();
                }

                let embed = embed::build()
                    .title("Starboards")
                    .description(final_result)
                    .footer(
                        EmbedFooterBuilder::new(concat!(
                            "Run '/starboards view' with a specific starboard ",
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
