use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::{PermRoleStarboard, Starboard},
    get_guild_id,
    interactions::{commands::tribool::Tribool, context::CommandCtx},
    map_dup_none, unwrap_id,
    utils::pg_err_code::get_pg_err_code,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "edit-starboard",
    desc = "Edit the settings for a PermRole in a starboard."
)]
pub struct EditPermRoleStarboard {
    /// The PermRole to edit.
    role: Role,
    /// The starboard to edit the PermRole for.
    #[command(autocomplete = true)]
    starboard: String,

    /// Whether a user can vote on messages.
    vote: Option<Tribool>,
    /// Whether a user's messages can be voted on.
    #[command(rename = "receive-votes")]
    receive_vote: Option<Tribool>,
}

impl EditPermRoleStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let sb =
            Starboard::get_by_name(&ctx.bot.pool, &self.starboard, unwrap_id!(guild_id)).await?;
        let sb = match sb {
            None => {
                ctx.respond_str(
                    &format!("Starboard '{}' does not exist.", self.starboard),
                    true,
                )
                .await?;
                return Ok(());
            }
            Some(sb) => sb,
        };

        let pr_sb = map_dup_none!(PermRoleStarboard::create(
            &ctx.bot.pool,
            unwrap_id!(self.role.id),
            sb.id
        ));
        let pr_sb = match pr_sb {
            Ok(pr_sb) => pr_sb,
            Err(why) => {
                if get_pg_err_code(&why).as_deref() == Some("23503") {
                    ctx.respond_str(&format!("{} is not a PermRole.", self.role.mention()), true)
                        .await?;
                    return Ok(());
                } else {
                    return Err(why.into());
                }
            }
        };

        let mut pr_sb = match pr_sb {
            Some(pr_sb) => pr_sb,
            None => PermRoleStarboard::get(&ctx.bot.pool, unwrap_id!(self.role.id), sb.id)
                .await?
                .unwrap(),
        };

        if let Some(val) = self.vote {
            pr_sb.give_votes = val.as_bool();
        }
        if let Some(val) = self.receive_vote {
            pr_sb.receive_votes = val.as_bool();
        }

        pr_sb.update(&ctx.bot.pool).await?;
        ctx.respond_str(
            &format!(
                "Updated the settings for {} in '{}'.",
                self.role.mention(),
                sb.name
            ),
            false,
        )
        .await?;

        Ok(())
    }
}
