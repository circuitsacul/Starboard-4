use std::sync::Arc;

use floodgate::FixedMapping;
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::constants;

pub struct Cooldowns {
    // restricts per-channel
    pub autostar_send: Arc<FixedMapping<Id<ChannelMarker>>>,
}

impl Cooldowns {
    pub fn new() -> Self {
        let autostar_send = Arc::new(FixedMapping::new(
            constants::AUTOSTAR_COOLDOWN.0,
            constants::AUTOSTAR_COOLDOWN.1,
        ));
        FixedMapping::start(autostar_send.clone(), None);

        Self { autostar_send }
    }
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self::new()
    }
}
