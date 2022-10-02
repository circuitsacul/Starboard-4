use std::collections::HashSet;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::mentions::textable_channel_ids, StarboardOverride},
    get_guild_id,
    interactions::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Add channels to an override.")]
pub struct AddOverrideChannels {
    /// The override to add channels to.
    #[command(autocomplete = true, rename = "override")]
    name: String,
    /// The channels to add.
    channels: String,
}

impl AddOverrideChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let ov = StarboardOverride::get(&ctx.bot.pool, guild_id, &self.name).await?;
        if let Some(ov) = ov {
            let mut channel_ids: HashSet<_> =
                textable_channel_ids(&ctx.bot, guild_id, &self.channels);
            channel_ids.extend(ov.channel_ids);
            let new_channels: Vec<_> = channel_ids.into_iter().collect();

            let ret =
                StarboardOverride::set_channels(&ctx.bot.pool, guild_id, &self.name, &new_channels)
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
