use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::emoji::{EmojiCommon, SimpleEmoji},
    get_guild_id,
    interactions::commands::context::CommandCtx,
    models::AutoStarChannel,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Set the emojis for an autostar channel.")]
pub struct EditAutoStar {
    /// The name of the autostar channel to edit.
    name: String,
    /// The emojis to use. Use "none" to set to none.
    emojis: Option<String>,
    /// The minimum number of characters a message needs.
    #[command(rename = "min-chars", min_value = 0, max_value = 5_000)]
    min_chars: Option<i64>,
    /// The maximum number of characters a message can have. Set to 0 to disable.
    #[command(rename = "max-chars", min_value = 0, max_value = 5_000)]
    max_chars: Option<i64>,
    /// Whether or not a message must include an image.
    #[command(rename = "require-image")]
    require_image: Option<bool>,
    /// Whether to delete messages that don't meet requirements.
    #[command(rename = "delete-invalid")]
    delete_invalid: Option<bool>,
}

impl EditAutoStar {
    pub async fn callback(self, ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let mut update = AutoStarChannel::edit_settings();

        if let Some(val) = self.emojis {
            let emojis = Vec::<SimpleEmoji>::from_user_input(val, &ctx.bot, guild_id).await;
            update.set_emojis(emojis.into_stored());
        }
        if let Some(val) = self.min_chars {
            update.set_min_chars(val.try_into().unwrap())?;
        }
        if let Some(val) = self.max_chars {
            update.set_max_chars(if val == 0 {
                None
            } else {
                Some(val.try_into().unwrap())
            })?;
        }
        if let Some(val) = self.require_image {
            update.set_require_image(val);
        }
        if let Some(val) = self.delete_invalid {
            update.set_delete_invalid(val);
        }

        let asc = update.exec(&ctx.bot.pool, &self.name, guild_id).await?;

        if asc.is_none() {
            ctx.respond_str("No autostar channels with that name were found.", true)
                .await?;
            return Ok(());
        }

        // set the emojis
        ctx.respond_str(
            &format!("Updated the settings for autostar channel '{}'.", self.name),
            false,
        )
        .await?;

        Ok(())
    }
}
