use crate::{
    client::bot::StarboardBot,
    constants,
    database::{AutoStarChannel, Starboard},
    errors::StarboardResult,
    utils::into_id::IntoId,
};

pub async fn refresh_premium_locks(
    bot: &StarboardBot,
    guild_id: i64,
    premium: bool,
) -> StarboardResult<()> {
    if premium {
        sqlx::query!(
            "UPDATE starboards SET premium_locked=false WHERE guild_id=$1",
            guild_id
        )
        .fetch_all(&bot.pool)
        .await?;

        let channel_ids = sqlx::query!(
            "UPDATE autostar_channels SET premium_locked=false WHERE guild_id=$1
            RETURNING channel_id",
            guild_id
        )
        .fetch_all(&bot.pool)
        .await?;
        for row in channel_ids {
            bot.cache
                .autostar_channel_ids
                .insert(row.channel_id.into_id());
        }

        return Ok(());
    }

    // starboards
    let count = Starboard::count_by_guild(&bot.pool, guild_id).await?;
    let to_lock = count - constants::MAX_STARBOARDS;
    if to_lock > 0 {
        let sb_ids = sqlx::query!(
            "SELECT id FROM starboards WHERE guild_id=$1 LIMIT $2",
            guild_id,
            to_lock
        )
        .fetch_all(&bot.pool)
        .await?;
        sqlx::query!(
            "UPDATE starboards SET premium_locked=true WHERE id=any($1)",
            &sb_ids.into_iter().map(|r| r.id).collect::<Vec<_>>()
        )
        .fetch_all(&bot.pool)
        .await?;
    }

    // autostar channels
    let count = AutoStarChannel::count_by_guild(&bot.pool, guild_id).await?;
    let to_lock = count - constants::MAX_AUTOSTAR;
    if to_lock > 0 {
        let asc_ids = sqlx::query!(
            "SELECT id FROM autostar_channels WHERE guild_id=$1 LIMIT $2",
            guild_id,
            to_lock
        )
        .fetch_all(&bot.pool)
        .await?;
        sqlx::query!(
            "UPDATE autostar_channels SET premium_locked=true WHERE id=any($1)",
            &asc_ids.into_iter().map(|r| r.id).collect::<Vec<_>>()
        )
        .fetch_all(&bot.pool)
        .await?;
    }

    Ok(())
}
