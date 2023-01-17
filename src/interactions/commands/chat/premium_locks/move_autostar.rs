use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::AutoStarChannel, errors::StarboardResult, interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

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
            ctx.respond_str("Please run this command inside a server.", true).await?;
            return Ok(());
        };
        let guild_id_i64 = guild_id.get_i64();

        let mut tx = ctx.bot.pool.begin().await?;

        let Some(asc_from) = get_for_update(&mut ctx, &mut tx, guild_id_i64, &self.autostar_from).await? else {
            return Ok(());
        };
        let Some(asc_to) = get_for_update(&mut ctx, &mut tx, guild_id_i64, &self.autostar_to).await? else {
            return Ok(());
        };

        if !asc_from.premium_locked {
            ctx.respond_str(
                &format!("Autostar channel '{}' is not locked.", asc_from.name),
                true,
            )
            .await?;
            return Ok(());
        }
        if asc_to.premium_locked {
            ctx.respond_str(
                &format!("Autostar channel '{}' is already locked.", asc_to.name),
                true,
            )
            .await?;
            return Ok(());
        }

        sqlx::query!(
            "UPDATE autostar_channels SET premium_locked=true WHERE id=$1",
            asc_to.id,
        )
        .fetch_all(&mut tx)
        .await?;
        sqlx::query!(
            "UPDATE autostar_channels SET premium_locked=false WHERE id=$1",
            asc_from.id,
        )
        .fetch_all(&mut tx)
        .await?;

        tx.commit().await?;

        ctx.respond_str("Done.", true).await?;

        Ok(())
    }
}

async fn get_for_update(
    ctx: &mut CommandCtx,
    con: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guild_id: i64,
    name: &str,
) -> StarboardResult<Option<AutoStarChannel>> {
    let asc = sqlx::query_as!(
        AutoStarChannel,
        "SELECT * FROM autostar_channels WHERE guild_id=$1 AND name=$2 FOR UPDATE",
        guild_id,
        name,
    )
    .fetch_optional(con)
    .await?;
    let Some(asc) = asc else {
        ctx.respond_str(&format!("Autotstar channel '{name}' does not exist."), true).await?;
        return Ok(None);
    };

    Ok(Some(asc))
}
