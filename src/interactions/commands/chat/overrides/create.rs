use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::{validation, Starboard, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    map_dup_none, unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "create", desc = "Create an override.")]
pub struct CreateOverride {
    /// The name of the override.
    name: String,
    /// The starboard this override belongs too.
    #[command(autocomplete = true)]
    starboard: String,
}

impl CreateOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let name = match validation::name::validate_name(&self.name) {
            Err(why) => {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            Ok(name) => name,
        };

        let starboard = Starboard::get_by_name(&ctx.bot.pool, &self.starboard, guild_id).await?;
        let starboard = match starboard {
            None => {
                ctx.respond_str(&format!("'{}' is not a starboard.", self.starboard), true)
                    .await?;
                return Ok(());
            }
            Some(val) => val,
        };

        let count = StarboardOverride::count_by_starboard(&ctx.bot.pool, starboard.id).await?;
        if count >= constants::MAX_OVERRIDES_PER_STARBOARD {
            ctx.respond_str(
                &format!(
                    "You can only have up to {} overrides per starboard.",
                    constants::MAX_OVERRIDES_PER_STARBOARD
                ),
                true,
            )
            .await?;
            return Ok(());
        }

        let ov = StarboardOverride::create(&ctx.bot.pool, guild_id, &name, starboard.id);
        let ov = map_dup_none!(ov)?;

        if ov.is_none() {
            ctx.respond_str(
                &format!("An override with the name '{name}' already exists."),
                true,
            )
            .await?;
        } else {
            ctx.respond_str(
                &format!(
                    "Created override '{}' in starboard '{}'.",
                    name, self.starboard
                ),
                false,
            )
            .await?;
        }

        Ok(())
    }
}
