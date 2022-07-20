use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, Starboard},
    get_guild_id,
    interactions::commands::context::CommandCtx,
    map_dup_none, unwrap_id,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "rename", desc = "Rename a starboard.")]
pub struct RenameStarboard {
    /// The current name of the starboard.
    #[command(rename = "current-name", autocomplete = true)]
    current_name: String,
    /// The new name for the starboard.
    #[command(rename = "new-name")]
    new_name: String,
}

impl RenameStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let new_name = match validation::name::validate_name(&self.new_name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let ret = map_dup_none!(Starboard::rename(
            &ctx.bot.pool,
            &self.current_name,
            unwrap_id!(guild_id),
            &new_name
        ))?;

        match ret {
            None => {
                ctx.respond_str(
                    &format!("A starboard with the name '{}' already exists.", new_name),
                    true,
                )
                .await?
            }
            Some(None) => {
                ctx.respond_str("No starboard with that name was found.", true)
                    .await?
            }
            Some(Some(_)) => {
                ctx.bot.cache.guild_starboard_names.remove(&guild_id).await;

                ctx.respond_str(
                    &format!(
                        "Renamed the starboard from '{}' to '{}'.",
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
