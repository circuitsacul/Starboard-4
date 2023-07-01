use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::PermRole,
    errors::StarboardResult,
    interactions::{commands::choices::tribool::Tribool, context::CommandCtx},
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(permroles_edit);
locale_func!(permroles_edit_option_role);

locale_func!(permroles_option_vote);
locale_func!(permroles_option_receive_votes);
locale_func!(permroles_option_xproles);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "edit",
    desc = "Edit the global permissions for a PermRole.",
    desc_localizations = "permroles_edit"
)]
pub struct EditPermRole {
    /// The PermRole to edit.
    #[command(desc_localizations = "permroles_edit_option_role")]
    role: Role,

    /// Whether a user with this role can vote on messages.
    #[command(desc_localizations = "permroles_option_vote")]
    vote: Option<Tribool>,
    /// Whether a user with this role can receive votes.
    #[command(
        rename = "receive-votes",
        desc_localizations = "permroles_option_receive_votes"
    )]
    receive_votes: Option<Tribool>,
    /// Whether a user with this role can gain XPRoles.
    #[command(desc_localizations = "permroles_option_xproles")]
    xproles: Option<Tribool>,
}

impl EditPermRole {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let lang = ctx.user_lang();

        let pr = PermRole::get(&ctx.bot.pool, self.role.id.get_i64()).await?;
        let mut pr = match pr {
            None => {
                ctx.respond_str(lang.permrole_missing(self.role.mention()), true)
                    .await?;
                return Ok(());
            }
            Some(pr) => pr,
        };

        if let Some(val) = self.vote {
            pr.give_votes = val.as_bool();
        }
        if let Some(val) = self.receive_votes {
            pr.receive_votes = val.as_bool();
        }
        if let Some(val) = self.xproles {
            pr.obtain_xproles = val.as_bool();
        }

        pr.update(&ctx.bot.pool).await?;

        ctx.respond_str(lang.permroles_edit_done(self.role.mention()), false)
            .await?;
        Ok(())
    }
}
