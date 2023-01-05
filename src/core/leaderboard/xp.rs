use crate::{
    client::bot::StarboardBot,
    database::{Member, Starboard},
    errors::StarboardResult,
};

pub async fn refresh_xp(bot: &StarboardBot, guild_id: i64, user_id: i64) -> StarboardResult<bool> {
    let xp = calculate_xp(bot, guild_id, user_id).await?;

    Member::set_xp(&bot.pool, user_id, guild_id, xp).await?;

    Ok(true)
}

pub async fn calculate_xp(bot: &StarboardBot, guild_id: i64, user_id: i64) -> StarboardResult<f32> {
    let starboards = Starboard::list_by_guild(&bot.pool, guild_id).await?;

    let mut total: f32 = 0.;

    for sb in starboards {
        if sb.settings.private {
            continue;
        }

        total += count_votes(bot, user_id, sb.id).await? as f32 * sb.settings.xp_multiplier;
    }

    Ok(total)
}

async fn count_votes(bot: &StarboardBot, user_id: i64, starboard_id: i32) -> StarboardResult<i32> {
    let upvotes = sqlx::query!(
        r#"SELECT count(*) FROM votes WHERE starboard_id=$1
        AND target_author_id=$2 AND is_downvote=false"#,
        starboard_id,
        user_id,
    )
    .fetch_one(&bot.pool)
    .await?
    .count
    .unwrap();
    let downvotes = sqlx::query!(
        r#"SELECT count(*) FROM votes WHERE starboard_id=$1
        AND target_author_id=$2 AND is_downvote=true"#,
        starboard_id,
        user_id,
    )
    .fetch_one(&bot.pool)
    .await?
    .count
    .unwrap();

    Ok(upvotes as i32 - downvotes as i32)
}
