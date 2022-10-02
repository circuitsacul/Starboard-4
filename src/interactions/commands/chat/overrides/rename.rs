use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, StarboardOverride},
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none, unwrap_id,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "rename", desc = "Rename an override.")]
pub struct RenameOverride {
    /// The current name of the override.
    #[command(autocomplete = true, rename = "current-name")]
    old_name: String,
    /// The new name of the override.
    #[command(rename = "new-name")]
    name: String,
}

impl RenameOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let name = match validation::name::validate_name(&self.name) {
            Ok(val) => val,
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
        };

        let ov = StarboardOverride::rename(&ctx.bot.pool, guild_id, &self.old_name, &self.name);
        let ov = map_dup_none!(ov)?;

        match ov {
            None => {
                ctx.respond_str(
                    &format!("An override with the name '{}' already exists.", name),
                    true,
                )
                .await?;
            }
            Some(None) => {
                ctx.respond_str(
                    &format!("No override with the name '{}' exists.", self.old_name),
                    true,
                )
                .await?;
            }
            Some(Some(_)) => {
                ctx.respond_str(
                    &format!("Renamed override '{}' to '{}'.", self.old_name, name),
                    false,
                )
                .await?;
            }
        }

        Ok(())
    }
}
