//! Parsing and validation for different types of mentions.

use std::collections::HashSet;

use lazy_static::lazy_static;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{client::bot::StarboardBot, errors::StarboardResult, utils::id_as_i64::GetI64};

/// Parse channel ids from user input.
///
/// Only includes valid, textable, guild-bound channels.
pub async fn textable_channel_ids(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    inp: &str,
) -> StarboardResult<HashSet<i64>> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r#"\d+"#).unwrap();
    }

    let mut ret = HashSet::new();

    for channel_id in RE.find_iter(inp).map(|val| val.as_str().parse().unwrap()) {
        if !bot
            .cache
            .guild_has_channel(bot, guild_id, channel_id)
            .await?
        {
            continue;
        }

        ret.insert(channel_id.get_i64());
    }

    Ok(ret)
}
