use std::collections::HashSet;

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
        if let Some(ov) = ov {
            let to_remove: HashSet<_> = textable_channel_ids(&ctx.bot, guild_id, &self.channels);
            let channel_ids: Vec<_> = ov
                .channel_ids
                .iter()
                .copied()
                .filter(|id| !to_remove.contains(id))
                .collect();

            let ret =
                StarboardOverride::set_channels(&ctx.bot.pool, guild_id, &self.name, &channel_ids)
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
