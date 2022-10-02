use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    get_guild_id,
    interactions::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove channels from an override.")]
pub struct RemoveOverrideChannels {
    /// The override to remove channels from.
    #[command(autocomplete = true, rename = "override")]
    name: String,
    /// The channels to remove.
    channels: String,
}

impl RemoveOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let ov = StarboardOverride::get(&ctx.bot.pool, guild_id, &self.name).await?;
        if let Some(mut ov) = ov {
            let channel_ids = textable_channel_ids(&ctx.bot, guild_id, &self.channels);
            ov.channel_ids.retain(|id| !channel_ids.contains(id));
            let ret = StarboardOverride::set_channels(
                &ctx.bot.pool,
                guild_id,
                &self.name,
                &ov.channel_ids,
            )
            .await?;

            if ret.is_some() {
                ctx.respond_str(
                    &format!("Updated the channels for override '{}'.", self.name),
                    false,
                )
                .await?;
                return Ok(());
            }
        }

        ctx.respond_str(
            &format!("No override with the name '{}' exists.", self.name),
            true,
        )
        .await?;
        Ok(())
    }
}
