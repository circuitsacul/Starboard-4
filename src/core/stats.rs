use crate::{
    client::bot::StarboardBot,
    database::{Member, Starboard},
    errors::StarboardResult,
};

#[derive(Default)]
pub struct MemberStats {
    pub xp: f32,
    pub given_upvotes: i64,
    pub given_downvotes: i64,
    pub received_upvotes: i64,
    pub received_downvotes: i64,
}

impl MemberStats {
    pub async fn get(
        pool: &sqlx::PgPool,
        guild_id: i64,
        user_id: i64,
    ) -> StarboardResult<Option<Self>> {
        let mut stats = Self::default();

        let starboards = Starboard::list_by_guild(pool, guild_id).await?;
        if starboards.is_empty() {
            return Ok(None);
        }

        for sb in starboards {
            if sb.settings.private {
                continue;
            }

            let given_upvotes = Self::given_upvotes(pool, user_id, sb.id).await?;
            let given_downvotes = Self::given_downvotes(pool, user_id, sb.id).await?;
            let received_upvotes = Self::received_upvotes(pool, user_id, sb.id).await?;
            let received_downvotes = Self::received_downvotes(pool, user_id, sb.id).await?;

            stats.given_upvotes += given_upvotes;
            stats.given_downvotes += given_downvotes;
            stats.received_upvotes += received_upvotes;
            stats.received_downvotes += received_downvotes;

            stats.xp += (received_upvotes - received_downvotes) as f32 * sb.settings.xp_multiplier;
        }

        Ok(Some(stats))
    }

    async fn given_downvotes(
        pool: &sqlx::PgPool,
        user_id: i64,
        starboard_id: i32,
    ) -> StarboardResult<i64> {
        Ok(sqlx::query!(
            "SELECT count(*) FROM votes WHERE starboard_id=$1 AND user_id=$2
            AND is_downvote=true",
            starboard_id,
            user_id,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap())
    }

    async fn given_upvotes(
        pool: &sqlx::PgPool,
        user_id: i64,
        starboard_id: i32,
    ) -> StarboardResult<i64> {
        Ok(sqlx::query!(
            "SELECT count(*) FROM votes WHERE starboard_id=$1 AND user_id=$2
            AND is_downvote=false",
            starboard_id,
            user_id,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap())
    }

    async fn received_downvotes(
        pool: &sqlx::PgPool,
        user_id: i64,
        starboard_id: i32,
    ) -> StarboardResult<i64> {
        Ok(sqlx::query!(
            "SELECT count(*) FROM votes WHERE starboard_id=$1
            AND target_author_id=$2 AND is_downvote=true",
            starboard_id,
            user_id,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap())
    }

    async fn received_upvotes(
        pool: &sqlx::PgPool,
        user_id: i64,
        starboard_id: i32,
    ) -> StarboardResult<i64> {
        Ok(sqlx::query!(
            "SELECT count(*) FROM votes WHERE starboard_id=$1
            AND target_author_id=$2 AND is_downvote=false",
            starboard_id,
            user_id,
        )
        .fetch_one(pool)
        .await?
        .count
        .unwrap())
    }
}

pub async fn refresh_xp(bot: &StarboardBot, guild_id: i64, user_id: i64) -> StarboardResult<()> {
    let Some(stats) = MemberStats::get(&bot.pool, guild_id, user_id).await? else {
        return Ok(());
    };

    Member::set_xp(&bot.pool, user_id, guild_id, stats.xp).await?;

    Ok(())
}
