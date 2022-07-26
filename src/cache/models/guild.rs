use twilight_model::id::{marker::EmojiMarker, Id};

use crate::utils::async_dash::AsyncDashSet;

pub struct CachedGuild {
    pub emojis: AsyncDashSet<Id<EmojiMarker>>,
}
