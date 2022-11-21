use std::collections::HashMap;

use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker},
    Id,
};

use super::channel::CachedChannel;

pub struct CachedGuild {
    /// all custom emojis mapped to whether they are animated
    pub emojis: HashMap<Id<EmojiMarker>, bool>,
    /// all textable channels except for threads
    pub channels: HashMap<Id<ChannelMarker>, CachedChannel>,
    pub active_thread_parents: HashMap<Id<ChannelMarker>, Id<ChannelMarker>>,
}
