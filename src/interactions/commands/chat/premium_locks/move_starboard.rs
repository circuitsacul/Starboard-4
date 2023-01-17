use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{models::starboard::starboard_from_record, Starboard},
    errors::StarboardResult,
    interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "move-starboard",
    desc = "Move a lock from one starboard to another."
)]
pub struct MoveStarboard {
    /// The starboard to move the lock from.
    #[command(rename = "from", autocomplete = true)]
    starboard_from: String,
    /// The starboard to move the lock to.
    #[command(rename = "to", autocomplete = true)]
    starboard_to: String,
}

impl MoveStarboard {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let Some(guild_id) = ctx.interaction.guild_id else {
            ctx.respond_str("Please run this command inside a server.", true).await?;
            return Ok(());
        };
        let guild_id_i64 = guild_id.get_i64();

        let mut tx = ctx.bot.pool.begin().await?;

        let Some(sb_from) = get_for_update(&mut ctx, &mut tx, guild_id_i64, &self.starboard_from).await? else {
            return Ok(());
        };
        let Some(sb_to) = get_for_update(&mut ctx, &mut tx, guild_id_i64, &self.starboard_to).await? else {
            return Ok(());
        };

        if !sb_from.premium_locked {
            ctx.respond_str(
                &format!("Starboard '{}' is not locked.", sb_from.name),
                true,
            )
            .await?;
            return Ok(());
        }
        if sb_to.premium_locked {
            ctx.respond_str(
                &format!("Starboard '{}' is already locked.", sb_to.name),
                true,
            )
            .await?;
            return Ok(());
        }

        sqlx::query!(
            "UPDATE starboards SET premium_locked=true WHERE id=$1",
            sb_to.id,
        )
        .fetch_all(&mut tx)
        .await?;
        sqlx::query!(
            "UPDATE starboards SET premium_locked=false WHERE id=$1",
            sb_from.id,
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
) -> StarboardResult<Option<Starboard>> {
    let sb = sqlx::query!(
        "SELECT * FROM starboards WHERE guild_id=$1 AND name=$2 FOR UPDATE",
        guild_id,
        name,
    )
    .fetch_optional(con)
    .await?;

    let Some(sb) = sb else {
        ctx.respond_str(&format!("Starboard '{name}' does not exist."), true).await?;
        return Ok(None);
    };

    Ok(Some(starboard_from_record!(sb)))
}
