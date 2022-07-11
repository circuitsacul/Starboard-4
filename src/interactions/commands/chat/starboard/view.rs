use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder};

use crate::{
    core::starboard::config::StarboardConfig,
    database::Starboard,
    get_guild_id,
    interactions::commands::{context::CommandCtx, format_settings::format_settings},
    unwrap_id,
    utils::embed,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your starboards.")]
pub struct ViewStarboard {
    /// The name of the starboard to view. Leave blank to show all.
    #[command(autocomplete = true)]
    name: Option<String>,
}

impl ViewStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(name) = self.name {
            let starboard =
                Starboard::get_by_name(&ctx.bot.pool, &name, unwrap_id!(guild_id)).await?;

            if let Some(starboard) = starboard {
                let config = StarboardConfig::new(starboard, vec![])?;
                let pretty = format_settings(&ctx.bot, guild_id, &config).await;

                let embed = embed::build()
                    .title(format!("Starboard '{}'", &config.starboard.name))
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
            let starboards = Starboard::list_by_guild(&ctx.bot.pool, unwrap_id!(guild_id)).await?;
            if starboards.is_empty() {
                ctx.respond_str("This server has no starboards.", true)
                    .await?;
            } else {
                let mut final_result = String::new();

                for sb in starboards.into_iter() {
                    final_result.push_str(&format!("'{}' in <#{}>\n", sb.name, sb.channel_id))
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
