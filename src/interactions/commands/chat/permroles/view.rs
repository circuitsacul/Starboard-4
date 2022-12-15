use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;
use twilight_util::builder::embed::EmbedFooterBuilder;

use crate::{
    concat_format,
    database::{models::permrole::SortVecPermRole, PermRole, PermRoleStarboard, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{embed, id_as_i64::GetI64},
};

macro_rules! fmt_trib {
    ($to_fmt: expr) => {
        $to_fmt
            .map(|v| v.to_string())
            .unwrap_or("default".to_string())
    };
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "view", desc = "View the PermRoles for this server.")]
pub struct ViewPermRoles {
    /// The PermRole to view settings for.
    role: Option<Role>,
}

impl ViewPermRoles {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(role) = self.role {
            let permrole = PermRole::get(&ctx.bot.pool, role.id.get_i64()).await?;

            if let Some(permrole) = permrole {
                let mut pr_config = format!("Settings for {}:\n", role.mention());
                pr_config.push_str(&concat_format!(
                    "vote: {}\n" <- fmt_trib!(permrole.give_votes);
                    "receive-votes: {}\n" <- fmt_trib!(permrole.receive_votes);
                    "xproles: {}\n" <- fmt_trib!(permrole.obtain_xproles);
                ));

                let permrole_sbs =
                    PermRoleStarboard::list_by_permrole(&ctx.bot.pool, permrole.role_id).await?;

                for pr_sb in permrole_sbs {
                    let sb = Starboard::get(&ctx.bot.pool, pr_sb.starboard_id).await?;
                    let sb = match sb {
                        None => {
                            eprintln!(
                                "Starboard for PermRole didn't exist. This shouldn't happen."
                            );
                            continue;
                        }
                        Some(sb) => sb,
                    };

                    pr_config.push_str(&format!(
                        "\nSettings for '{}' in <#{}>:\n",
                        sb.name, sb.channel_id
                    ));
                    pr_config.push_str(&concat_format!(
                        "vote: {}\n" <- fmt_trib!(pr_sb.give_votes);
                        "receive-votes: {}\n" <- fmt_trib!(pr_sb.receive_votes);
                    ));
                }

                let embed = embed::build()
                    .title("PermRoles")
                    .description(pr_config)
                    .build();

                ctx.respond(ctx.build_resp().embeds([embed]).build())
                    .await?;
            } else {
                ctx.respond_str(&format!("{} is not a PermRole.", role.mention()), true)
                    .await?;
            }
        } else {
            let mut perm_roles = PermRole::list_by_guild(&ctx.bot.pool, guild_id.get_i64()).await?;

            if perm_roles.is_empty() {
                ctx.respond_str("This server has no PermRoles.", true)
                    .await?;
                return Ok(());
            }

            let mut pr_config = String::new();

            perm_roles.sort_permroles(&ctx.bot);
            for pr in perm_roles.into_iter().rev() {
                pr_config.push_str(&format!("<@&{}>\n", pr.role_id));
            }

            let embed = embed::build()
                .title("PermRoles")
                .description(pr_config)
                .footer(EmbedFooterBuilder::new(concat!(
                    "PermRoles are applied from bottom to top (same as Discord roles).\n",
                    "Use '/permroles view' with a specific role to see its settings.",
                )))
                .build();

            ctx.respond(ctx.build_resp().embeds([embed]).build())
                .await?;
        }

        Ok(())
    }
}
