use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::StarboardOverride, get_guild_id, interactions::context::CommandCtx, unwrap_id,
    utils::views::confirm,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete", desc = "Delete an override.")]
pub struct DeleteOverride {
    /// The name of the override to delete.
    #[command(autocomplete = true)]
    name: String,
}

impl DeleteOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = unwrap_id!(get_guild_id!(ctx));

        let btn_ctx = confirm::simple(
            &mut ctx,
            &format!(
                "Are you sure you want to delete the override '{}'?",
                self.name
            ),
            true,
        )
        .await?;
        let mut btn_ctx = match btn_ctx {
            None => return Ok(()),
            Some(btn_ctx) => btn_ctx,
        };

        let ov = StarboardOverride::delete(&ctx.bot.pool, guild_id, &self.name).await?;
        if ov.is_none() {
            btn_ctx
                .edit_str(
                    &format!("No override with the name '{}' exists.", self.name),
                    true,
                )
                .await?;
        } else {
            btn_ctx
                .edit_str(&format!("Deleted override '{}'.", self.name), true)
                .await?;
        }

        Ok(())
    }
}
