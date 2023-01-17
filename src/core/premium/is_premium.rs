use crate::{client::bot::StarboardBot, database::Guild, errors::StarboardResult};

pub async fn is_guild_premium(bot: &StarboardBot, guild_id: i64) -> StarboardResult<bool> {
    if let Some(guild) = Guild::get(&bot.pool, guild_id).await? {
        Ok(guild.premium_end.is_some())
    } else {
        Ok(false)
    }
}
