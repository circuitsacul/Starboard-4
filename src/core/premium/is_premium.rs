use crate::{client::bot::StarboardBot, database::DbGuild, errors::StarboardResult};

pub async fn is_guild_premium(bot: &StarboardBot, guild_id: i64) -> StarboardResult<bool> {
    if let Some(guild) = DbGuild::get(&bot.pool, guild_id).await? {
        Ok(guild.premium_end.is_some())
    } else {
        Ok(false)
    }
}
