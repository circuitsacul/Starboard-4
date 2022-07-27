use std::collections::HashSet;

use twilight_model::id::{marker::EmojiMarker, Id};

pub struct CachedGuild {
    pub emojis: HashSet<Id<EmojiMarker>>,
}
