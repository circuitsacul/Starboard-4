use std::collections::{HashMap, HashSet};

use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker},
    Id,
};

use super::channel::CachedChannel;

pub struct CachedGuild {
    pub emojis: HashSet<Id<EmojiMarker>>,
    /// all textable channels except for threads
    pub channels: HashMap<Id<ChannelMarker>, CachedChannel>,
    pub active_thread_parents: HashMap<Id<ChannelMarker>, Id<ChannelMarker>>,
}
