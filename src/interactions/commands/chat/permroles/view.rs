use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    constants, database::PermRole, get_guild_id, interactions::context::CommandCtx, unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "view", desc = "View the PermRoles for this server.")]
pub struct ViewPermRoles;

impl ViewPermRoles {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);

        let perm_roles = PermRole::list_by_guild(&ctx.bot.pool, unwrap_id!(guild_id)).await?;

        if perm_roles.is_empty() {
            ctx.respond_str("This server has no PermRoles.", false)
                .await?;
            return Ok(());
        }

        // macro_rules! fmt_trib {
        //     ($to_fmt: expr) => {
        //         $to_fmt.map(|v| v.to_string()).unwrap_or("default".to_string())
        //     };
        // }

        let mut pr_config = String::new();

        for pr in perm_roles {
            pr_config.push_str(&format!("<@&{}>\n", pr.role_id));
        }

        let embed = EmbedBuilder::new()
            .color(constants::BOT_COLOR)
            .title("PermRoles")
            .description(pr_config)
            .build();

        ctx.respond(ctx.build_resp().embeds([embed]).build())
            .await?;
        Ok(())
    }
}
