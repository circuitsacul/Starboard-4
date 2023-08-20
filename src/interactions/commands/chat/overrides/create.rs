use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    constants,
    database::{validation, Starboard, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_create);
locale_func!(overrides_create_option_name);
locale_func!(overrides_create_option_starboard);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "create",
    desc = "Create an override.",
    desc_localizations = "overrides_create"
)]
pub struct CreateOverride {
    /// The name of the override.
    #[command(desc_localizations = "overrides_create_option_name")]
    name: String,

    /// The starboard this override belongs too.
    #[command(
        autocomplete = true,
        desc_localizations = "overrides_create_option_starboard"
    )]
    starboard: String,
}

impl CreateOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

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
                ctx.respond_str(&lang.starboard_missing(self.starboard), true)
                    .await?;
                return Ok(());
            }
            Some(val) => val,
        };

        let count = StarboardOverride::count_by_starboard(&ctx.bot.pool, starboard.id).await?;
        if count >= constants::MAX_OVERRIDES_PER_STARBOARD {
            ctx.respond_str(
                &lang.overrides_create_limit(constants::MAX_OVERRIDES_PER_STARBOARD),
                true,
            )
            .await?;
            return Ok(());
        }

        let ov = StarboardOverride::create(&ctx.bot.pool, guild_id, &name, starboard.id).await?;

        if ov.is_none() {
            ctx.respond_str(&lang.override_already_exists(name), true)
                .await?;
        } else {
            ctx.respond_str(&lang.overrides_create_done(name, self.starboard), false)
                .await?;
        }

        Ok(())
    }
}
