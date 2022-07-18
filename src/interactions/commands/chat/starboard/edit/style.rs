use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::Starboard,
    get_guild_id,
    interactions::commands::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "style", desc = "Edit the general style of your starboard.")]
pub struct EditGeneralStyle {
    /// The starboard to edit.
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

        let starboard =
            Starboard::get_by_name(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await?;
        let mut starboard = match starboard {
            None => {
                ctx.respond_str("No starboard with that name was found.", true)
                    .await?;
                return Ok(());
            }
            Some(starboard) => starboard,
        };

        if let Some(val) = self.display_emoji {
            let emoji;
            if val == "none" {
                emoji = None;
            } else {
                emoji = match SimpleEmoji::from_user_input(val, &ctx.bot, guild_id).await {
                    None => {
                        ctx.respond_str("Invalid emoji for `display-emoji`.", true)
                            .await?;
                        return Ok(());
                    }
                    Some(emoji) => Some(emoji),
                }
            };
            starboard.settings.display_emoji = emoji.map(|emoji| emoji.into_stored());
        }
        if let Some(val) = self.ping_author {
            starboard.settings.ping_author = val;
        }
        if let Some(val) = self.use_server_profile {
            starboard.settings.use_server_profile = val;
        }
        if let Some(val) = self.extra_embeds {
            starboard.settings.extra_embeds = val;
        }
        if let Some(val) = self.use_webhook {
            starboard.settings.use_webhook = val;
        }

        starboard.update_settings(&ctx.bot.pool).await?;
        ctx.respond_str(&format!("Updated settings for '{}'.", self.name), false)
            .await?;
        Ok(())
    }
}
