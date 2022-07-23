use dashmap::DashSet;
use twilight_model::id::{marker::EmojiMarker, Id};

pub struct CachedGuild {
    pub emojis: DashSet<Id<EmojiMarker>>,
}
