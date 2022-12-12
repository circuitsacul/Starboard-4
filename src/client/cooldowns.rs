use std::{sync::Arc, time::Duration};

use floodgate::{DynamicMapping, FixedMapping};
use tokio::time::sleep;
use twilight_model::id::{
    marker::{ChannelMarker, UserMarker},
    Id,
};

use crate::constants;

use super::bot::StarboardBot;

pub struct Cooldowns {
    cycle_period: Duration,
    // restricts per-channel
    pub autostar_send: FixedMapping<Id<ChannelMarker>>,
    pub starboard_custom_cooldown: DynamicMapping<(Id<UserMarker>, i32)>,
}

impl Cooldowns {
    pub fn new() -> Self {
        let cycle_period = Duration::from_secs(3600);

        let autostar_send = FixedMapping::new(
            constants::AUTOSTAR_COOLDOWN.0,
            constants::AUTOSTAR_COOLDOWN.1,
        );
        let starboard_custom_cooldown = DynamicMapping::new(cycle_period);

        Self {
            cycle_period,
            autostar_send,
            starboard_custom_cooldown,
        }
    }

    pub fn start(bot: Arc<StarboardBot>) {
        tokio::spawn(async move {
            let cooldown = &bot.cooldowns;
            loop {
                sleep(cooldown.cycle_period).await;

                cooldown.autostar_send.cycle();
                cooldown.starboard_custom_cooldown.cycle();
            }
        });
    }
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self::new()
    }
}
