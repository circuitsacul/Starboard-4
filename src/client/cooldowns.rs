use std::{sync::Arc, time::Duration};

use floodgate::FixedMapping;
use tokio::time::sleep;
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::constants;

use super::bot::StarboardBot;

pub struct Cooldowns {
    // restricts per-channel
    pub autostar_send: FixedMapping<Id<ChannelMarker>>,
}

impl Cooldowns {
    pub fn new() -> Self {
        let autostar_send = FixedMapping::new(
            constants::AUTOSTAR_COOLDOWN.0,
            constants::AUTOSTAR_COOLDOWN.1,
        );

        Self { autostar_send }
    }

    pub fn start(bot: Arc<StarboardBot>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(600)).await;

                let cooldown = &bot.cooldowns;
                dbg!(cooldown.autostar_send.cycle());
            }
        });
    }
}

impl Default for Cooldowns {
    fn default() -> Self {
        Self::new()
    }
}
