use std::time::Duration;

pub const BOT_COLOR: u32 = 0xFFE19C;

// Cache size
pub const MAX_MESSAGES: u32 = 10_000;
pub const MAX_AUTOSTAR_NAMES: u32 = 100;

// Cooldowns
pub const AUTOSTAR_COOLDOWN: (u32, Duration) = (5, Duration::from_secs(20));

// Common Validation
pub const MAX_NAME_LENGTH: u32 = 32;

// AutoStar Validation
pub const MAX_MAX_CHARS: u16 = 5_000;
pub const MAX_MIN_CHARS: u16 = 5_000;
