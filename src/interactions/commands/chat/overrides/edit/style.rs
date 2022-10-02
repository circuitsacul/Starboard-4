use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::StarboardOverride,
    get_guild_id,
    interactions::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "style", desc = "Edit the general style of your starboard.")]
pub struct EditGeneralStyle {
    /// The override to edit.
    #[command(autocomplete = true)]
    name: String,

    /// The emoji to show next to the point count. Use 'none' for nothing.
    #[command(rename = "display-emoji")]
    display_emoji: Option<String>,
    /// Whether to mention the author on starboard posts.
    #[command(rename = "ping-author")]
    ping_author: Option<bool>,
    /// Whether to use per-server avatar and nicknames for posts.
    #[command(rename = "use-server-profile")]
    use_server_profile: Option<bool>,
    /// Whether to include extra embeds that were on the original message.
    #[command(rename = "extra-embeds")]
    extra_embeds: Option<bool>,
    /// Whether to use a webhook for starboard messages.
    #[command(rename = "use-webhook")]
    use_webhook: Option<bool>,
}

impl EditGeneralStyle {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let ov = StarboardOverride::get(&ctx.bot.pool, unwrap_id!(guild_id), &self.name).await?;
        let ov = match ov {
            None => {
                ctx.respond_str("No override with that name was found.", true)
                    .await?;
                return Ok(());
            }
            Some(ov) => ov,
        };
        let mut settings = ov.get_overrides()?;

        if let Some(val) = self.display_emoji {
            let emoji = if val == "none" {
                None
            } else {
                match SimpleEmoji::from_user_input(val, &ctx.bot, guild_id) {
                    None => {
                        ctx.respond_str("Invalid emoji for `display-emoji`.", true)
                            .await?;
                        return Ok(());
                    }
                    Some(emoji) => Some(emoji),
                }
            };
            settings.display_emoji = Some(emoji.map(|emoji| emoji.into_stored()));
        }
        if let Some(val) = self.ping_author {
            settings.ping_author = Some(val);
        }
        if let Some(val) = self.use_server_profile {
            settings.use_server_profile = Some(val);
        }
        if let Some(val) = self.extra_embeds {
            settings.extra_embeds = Some(val);
        }
        if let Some(val) = self.use_webhook {
            settings.use_webhook = Some(val);
        }

        StarboardOverride::update_settings(&ctx.bot.pool, ov.id, settings).await?;
        ctx.respond_str(
            &format!("Updated settings for override '{}'.", self.name),
            false,
        )
        .await?;
        Ok(())
    }
}
