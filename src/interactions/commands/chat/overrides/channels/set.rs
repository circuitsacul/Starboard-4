use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "set", desc = "Set the channels that an override affects.")]
pub struct SetOverrideChannels {
    /// The override to set the channels for.
    #[command(autocomplete = true, rename = "override")]
    name: String,
    /// A list of channels that the override should affect. Use "none" to
    /// remove all.
    channels: String,
}

impl SetOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let channel_ids: Vec<_> = textable_channel_ids(&ctx.bot, guild_id, &self.channels);
        if let Err(why) = StarboardOverride::validate_channels(&channel_ids) {
            ctx.respond_str(&why, true).await?;
            return Ok(());
        }
        let ov = StarboardOverride::set_channels(&ctx.bot.pool, guild_id, &self.name, &channel_ids)
            .await?;

        if ov.is_none() {
            ctx.respond_str(
                &format!("No override with the name '{}' exists.", self.name),
                true,
            )
            .await?;
        } else {
            ctx.respond_str(
                &format!("Set the channels for override '{}'.", self.name),
                false,
            )
            .await?;
        }
        Ok(())
    }
}
