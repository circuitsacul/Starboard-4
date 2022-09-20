use twilight_model::id::{marker::ChannelMarker, Id};

use crate::{constants, utils::cooldowns::FixedMapping};

pub struct Cooldowns {
    // restricts per-channel
    pub autostar_send: FixedMapping<Id<ChannelMarker>>,
}

impl Cooldowns {
    pub fn new() -> Self {
        Self {
            autostar_send: FixedMapping::new(
                constants::AUTOSTAR_COOLDOWN.0,
                constants::AUTOSTAR_COOLDOWN.1,
            ),
        }
    }
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self::new()
    }
}
