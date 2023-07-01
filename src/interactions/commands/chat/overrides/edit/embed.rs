use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::color, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_edit_embed);
locale_func!(overrides_edit_option_name);

locale_func!(sb_option_color);
locale_func!(sb_option_attachments_list);
locale_func!(sb_option_replied_to);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "embed",
    desc = "Edit the style of the embeds sent to your starboard.",
    desc_localizations = "overrides_edit_embed"
)]
pub struct EditEmbedStyle {
    /// The override to edit.
    #[command(autocomplete = true, desc_localizations = "overrides_edit_option_name")]
    name: String,

    /// The color of the embeds. Use 'none' for default.
    #[command(desc_localizations = "sb_option_color")]
    color: Option<String>,

    /// Whether to include a list of attachments.
    #[command(
        rename = "attachments-list",
        desc_localizations = "sb_option_attachments_list"
    )]
    attachments_list: Option<bool>,

    /// Whether to include the message that was replied to, if any.
    #[command(rename = "replied-to", desc_localizations = "sb_option_replied_to")]
    replied_to: Option<bool>,
}

impl EditEmbedStyle {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let lang = ctx.user_lang();

        let ov = StarboardOverride::get(&ctx.bot.pool, guild_id.get_i64(), &self.name).await?;
        let ov = match ov {
            None => {
                ctx.respond_str(&lang.override_missing(self.name), true)
                    .await?;
                return Ok(());
            }
            Some(ov) => ov,
        };
        let mut settings = ov.get_overrides()?;

        if let Some(val) = self.color {
            if val == "default" || val == lang.default() {
                settings.color = Some(None);
            } else {
                match color::parse_color(&val) {
                    Ok(val) => settings.color = Some(Some(val)),
                    Err(why) => {
                        ctx.respond_str(why, true).await?;
                        return Ok(());
                    }
                }
            }
        }
        if let Some(val) = self.attachments_list {
            settings.attachments_list = Some(val);
        }
        if let Some(val) = self.replied_to {
            settings.replied_to = Some(val);
        }

        StarboardOverride::update_settings(&ctx.bot.pool, ov.id, settings).await?;
        ctx.respond_str(&lang.overrides_edit_done(self.name), false)
            .await?;
        Ok(())
    }
}
