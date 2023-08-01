use twilight_interactions::command::{CommandModel, CreateCommand};

use database::pipelines;
use errors::StarboardResult;

use crate::{interactions::context::CommandCtx, utils::id_as_i64::GetI64};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "move-autostar",
    desc = "Move a lock from one autostar channel to another."
)]
pub struct MoveAutostar {
    /// The autostar channel to move the lock from.
    #[command(rename = "from", autocomplete = true)]
    autostar_from: String,
    /// The autostar channel to move the lock to.
    #[command(rename = "to", autocomplete = true)]
    autostar_to: String,
}

impl MoveAutostar {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str("Please run this command inside a server.", true)
                .await?;
            return Ok(());
        };
        let guild_id_i64 = guild_id.get_i64();

        if let Err(why) = pipelines::locks::autostar::move_lock(
            &ctx.bot.db,
            guild_id_i64,
            &self.autostar_from,
            &self.autostar_to,
        )
        .await?
        {
            ctx.respond_str(&why, true).await?;
            return Ok(());
        };

        ctx.respond_str("Done.", true).await?;

        Ok(())
    }
}

// todo: delete this
// async fn get_for_update(
//     ctx: &mut CommandCtx,
//     con: &mut sqlx::Transaction<'_, sqlx::Postgres>,
//     guild_id: i64,
//     name: &str,
// ) -> StarboardResult<Option<AutoStarChannel>> {
//     let asc = sqlx::query_as!(
//         AutoStarChannel,
//         "SELECT * FROM autostar_channels WHERE guild_id=$1 AND name=$2 FOR UPDATE",
//         guild_id,
//         name,
//     )
//     .fetch_optional(con)
//     .await?;
//     let Some(asc) = asc else {
//         ctx.respond_str(&format!("Autotstar channel '{name}' does not exist."), true).await?;
//         return Ok(None);
//     };

//     Ok(Some(asc))
// }
