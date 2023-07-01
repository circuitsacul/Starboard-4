use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::{PermRoleStarboard, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::choices::tribool::Tribool, context::CommandCtx},
    locale_func,
    utils::{id_as_i64::GetI64, pg_error::PgErrorTraits},
};

locale_func!(permroles_edit_sb);
locale_func!(permroles_edit_option_role);
locale_func!(permroles_edit_sb_option_starboard);

locale_func!(permroles_option_vote);
locale_func!(permroles_option_receive_votes);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "edit-starboard",
    desc = "Edit the settings for a PermRole in a starboard.",
    desc_localizations = "permroles_edit_sb"
)]
pub struct EditPermRoleStarboard {
    /// The PermRole to edit.
    #[command(desc_localizations = "permroles_edit_option_role")]
    role: Role,
    /// The starboard to edit the PermRole for.
    #[command(
        autocomplete = true,
        desc_localizations = "permroles_edit_sb_option_starboard"
    )]
    starboard: String,

    /// Whether a user can vote on messages.
    #[command(desc_localizations = "permroles_option_vote")]
    vote: Option<Tribool>,
    /// Whether a user's messages can be voted on.
    #[command(
        rename = "receive-votes",
        desc_localizations = "permroles_option_receive_votes"
    )]
    receive_vote: Option<Tribool>,
}

impl EditPermRoleStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let sb = Starboard::get_by_name(&ctx.bot.pool, &self.starboard, guild_id).await?;
        let sb = match sb {
            None => {
                ctx.respond_str(lang.starboard_missing(self.starboard), true)
                    .await?;
                return Ok(());
            }
            Some(sb) => sb,
        };

        let ret = PermRoleStarboard::create(&ctx.bot.pool, self.role.id.get_i64(), sb.id).await;
        let mut pr_sb = match ret {
            Ok(Some(val)) => val,
            Ok(None) => PermRoleStarboard::get(&ctx.bot.pool, self.role.id.get_i64(), sb.id)
                .await?
                .unwrap(),
            Err(why) => {
                if why.is_fk_violation() {
                    ctx.respond_str(lang.permrole_missing(self.role.mention()), true)
                        .await?;
                    return Ok(());
                }
                return Err(why.into());
            }
        };

        if let Some(val) = self.vote {
            pr_sb.give_votes = val.as_bool();
        }
        if let Some(val) = self.receive_vote {
            pr_sb.receive_votes = val.as_bool();
        }

        pr_sb.update(&ctx.bot.pool).await?;
        ctx.respond_str(
            lang.permroles_edit_sb_done(self.role.mention(), sb.name),
            false,
        )
        .await?;

        Ok(())
    }
}
