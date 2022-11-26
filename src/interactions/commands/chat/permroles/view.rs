use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::guild::Role;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{
    concat_format, constants, database::PermRole, get_guild_id, interactions::context::CommandCtx,
    unwrap_id,
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
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        if let Some(role) = self.role {
            let perm_role = PermRole::get(&ctx.bot.pool, unwrap_id!(role.id)).await?;

            if let Some(perm_role) = perm_role {
                let mut pr_config = format!("Settings for {}:\n", role.mention());
                pr_config.push_str(&concat_format!(
                    "vote: {}\n" <- fmt_trib!(perm_role.give_votes);
                    "receive-votes: {}\n" <- fmt_trib!(perm_role.receive_votes);
                    "xproles: {}\n" <- fmt_trib!(perm_role.obtain_xproles);
                ));

                let embed = EmbedBuilder::new()
                    .color(constants::BOT_COLOR)
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
            let perm_roles = PermRole::list_by_guild(&ctx.bot.pool, unwrap_id!(guild_id)).await?;

            if perm_roles.is_empty() {
                ctx.respond_str("This server has no PermRoles.", true)
                    .await?;
                return Ok(());
            }

            let mut pr_config = String::new();

            for pr in perm_roles {
                pr_config.push_str(&format!("<@&{}>\n", pr.role_id));
            }

            let embed = EmbedBuilder::new()
                .color(constants::BOT_COLOR)
                .title("PermRoles")
                .description(pr_config)
                .footer(EmbedFooterBuilder::new(
                    "Use '/permroles view' with a specific role to see its settings.",
                ))
                .build();

            ctx.respond(ctx.build_resp().embeds([embed]).build())
                .await?;
        }

        Ok(())
    }
}
