use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;

use crate::{
    database::{PermRoleStarboard, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::tribool::Tribool, context::CommandCtx},
    utils::id_as_i64::GetI64,
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
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        let sb = Starboard::get_by_name(&ctx.bot.pool, &self.starboard, guild_id.get_i64()).await?;
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

        let pr_sb = PermRoleStarboard::create(&ctx.bot.pool, self.role.id.get_i64(), sb.id).await?;

        let mut pr_sb = match pr_sb {
            Some(pr_sb) => pr_sb,
            None => PermRoleStarboard::get(&ctx.bot.pool, self.role.id.get_i64(), sb.id)
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
