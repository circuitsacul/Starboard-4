use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::{
        emoji::{EmojiCommon, SimpleEmoji},
        starboard::webhooks::create_webhook,
    },
    database::{Starboard, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::choices::go_to_message::GoToMessage, context::CommandCtx},
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_edit_style);
locale_func!(overrides_edit_option_name);

locale_func!(sb_option_display_emoji);
locale_func!(sb_option_ping_author);
locale_func!(sb_option_use_server_profile);
locale_func!(sb_option_extra_embeds);
locale_func!(sb_option_go_to_message);
locale_func!(sb_option_use_webhook);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "style",
    desc = "Edit the general style of your starboard.",
    desc_localizations = "overrides_edit_style"
)]
pub struct EditGeneralStyle {
    /// The override to edit.
    #[command(autocomplete = true, desc_localizations = "overrides_edit_option_name")]
    name: String,

    /// The emoji to show next to the point count. Use 'none' for nothing.
    #[command(
        rename = "display-emoji",
        desc_localizations = "sb_option_display_emoji"
    )]
    display_emoji: Option<String>,

    /// Whether to mention the author on starboard posts.
    #[command(rename = "ping-author", desc_localizations = "sb_option_ping_author")]
    ping_author: Option<bool>,

    /// Whether to use per-server avatar and nicknames for posts.
    #[command(
        rename = "use-server-profile",
        desc_localizations = "sb_option_use_server_profile"
    )]
    use_server_profile: Option<bool>,

    /// Whether to include extra embeds that were on the original message.
    #[command(rename = "extra-embeds", desc_localizations = "sb_option_extra_embeds")]
    extra_embeds: Option<bool>,

    /// Where to put the "Go to Message" link.
    #[command(
        rename = "go-to-message",
        desc_localizations = "sb_option_go_to_message"
    )]
    go_to_message: Option<GoToMessage>,

    /// Whether to use a webhook for starboard messages.
    #[command(rename = "use-webhook", desc_localizations = "sb_option_use_webhook")]
    use_webhook: Option<bool>,
}

impl EditGeneralStyle {
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

        if let Some(val) = self.display_emoji {
            let emoji = if val == "none" || val == lang.none() {
                None
            } else {
                let mut emojis = SimpleEmoji::from_user_input(&val, &ctx.bot, guild_id);
                if emojis.len() != 1 {
                    ctx.respond_str(lang.validation_display_emoji(), true)
                        .await?;
                    return Ok(());
                }

                emojis.pop()
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
        if let Some(val) = self.go_to_message {
            settings.go_to_message = Some(val.value() as i16);
        }
        let message;
        if let Some(val) = self.use_webhook {
            settings.use_webhook = Some(val);

            let starboard = Starboard::get(&ctx.bot.pool, ov.starboard_id)
                .await?
                .unwrap();
            message = Some(create_webhook(&ctx.bot, &starboard).await);
        } else {
            message = None;
        }

        StarboardOverride::update_settings(&ctx.bot.pool, ov.id, settings).await?;

        let mut response = lang.overrides_edit_done(self.name);
        if let Some(message) = message {
            response.push_str("\n\n");
            response.push_str(message);
        }

        ctx.respond_str(&response, false).await?;
        Ok(())
    }
}
