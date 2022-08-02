use std::collections::{HashMap, HashSet};

use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker},
    Id,
};

pub struct CachedGuild {
    pub emojis: HashSet<Id<EmojiMarker>>,
    pub nsfw_channels: HashSet<Id<ChannelMarker>>,
    pub sfw_channels: HashSet<Id<ChannelMarker>>,
    pub active_thread_parents: HashMap<Id<ChannelMarker>, Id<ChannelMarker>>,
}
