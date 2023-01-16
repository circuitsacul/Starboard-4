use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    client::bot::StarboardBot, database::Guild, errors::StarboardResult, utils::id_as_i64::GetI64,
};

pub async fn is_guild_premium(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
) -> StarboardResult<bool> {
    if let Some(guild) = Guild::get(&bot.pool, guild_id.get_i64()).await? {
        Ok(guild.premium_end.is_some())
    } else {
        Ok(false)
    }
}
