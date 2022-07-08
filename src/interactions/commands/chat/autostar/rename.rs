use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, AutoStarChannel},
    get_guild_id,
    interactions::commands::context::CommandCtx,
    map_dup_none, unwrap_id,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "rename", desc = "Rename an autostar channel.")]
pub struct RenameAutoStarChannel {
    /// The current name of the autostar channel.
    #[command(rename = "current-name", autocomplete = true)]
    current_name: String,
    /// The new name for the autostar channel.
    #[command(rename = "new-name")]
    new_name: String,
}

impl RenameAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let new_name = match validation::name::validate_name(&self.new_name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = map_dup_none!(AutoStarChannel::rename(
            &ctx.bot.pool,
            &self.current_name,
            unwrap_id!(guild_id),
            &new_name
        ))?;

        match ret {
            None => {
                ctx.respond_str(
                    &format!(
                        "An autostar channel with the name '{}' already exists.",
                        new_name
                    ),
                    true,
                )
                .await?
            }
            Some(None) => {
                ctx.respond_str("No autostar channel with that name was found.", true)
                    .await?
            }
            Some(Some(_)) => {
                ctx.bot
                    .cache
                    .guild_autostar_channel_names
                    .invalidate(&guild_id)
                    .await;

                ctx.respond_str(
                    &format!(
                        "Renamed the autostar channel from '{}' to '{}'.",
                        self.current_name, new_name
                    ),
                    false,
                )
                .await?
            }
        };

        Ok(())
    }
}
