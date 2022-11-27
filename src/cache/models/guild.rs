use std::{collections::HashMap, sync::Arc};

use twilight_model::id::{
    marker::{ChannelMarker, EmojiMarker, RoleMarker, UserMarker},
    Id,
};

use super::{channel::CachedChannel, member::CachedMember};

pub struct CachedGuild {
    /// all custom emojis mapped to whether they are animated
    pub emojis: HashMap<Id<EmojiMarker>, bool>,
    /// all textable channels except for threads
    pub channels: HashMap<Id<ChannelMarker>, CachedChannel>,
    pub members: HashMap<Id<UserMarker>, Arc<CachedMember>>,
    pub role_positions: HashMap<Id<RoleMarker>, i64>,
    pub active_thread_parents: HashMap<Id<ChannelMarker>, Id<ChannelMarker>>,
}
