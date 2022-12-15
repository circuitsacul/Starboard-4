//! Parsing and validation for different types of mentions.

use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::{
    client::bot::StarboardBot,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

/// Parse channel ids from user input.
///
/// Only includes valid, textable, guild-bound channels.
pub fn textable_channel_ids<T: FromIterator<i64>>(
    bot: &StarboardBot,
    guild_id: i64,
    inp: &str,
) -> T {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r#"\d+"#).unwrap();
    }

    let valid_channels: HashSet<i64> = bot.cache.guilds.with(&guild_id.into_id(), |_, g| {
        g.as_ref()
            .unwrap()
            .channels
            .keys()
            .copied()
            .map(|k| k.get_i64())
            .collect()
    });

    RE.find_iter(inp)
        .map(|val| val.as_str().parse().unwrap())
        .filter(|id| valid_channels.contains(id))
        .collect()
}
