use std::time::Duration;

pub const ZWS: &str = "\u{200B}";

pub const EMBED_DARK_BG: u32 = 0x2F3136;

pub const BOT_COLOR: u32 = 0xFFE19C;

// Cache size
pub const MAX_MESSAGES: u32 = 10_000;
pub const MAX_NAMES: u32 = 100;

// Cooldowns
pub const AUTOSTAR_COOLDOWN: (u32, Duration) = (5, Duration::from_secs(20));

// Common Validation
pub const MAX_NAME_LENGTH: u32 = 32;
pub const MIN_NAME_LENGTH: u32 = 3;

// AutoStar Validation
pub const MAX_MAX_CHARS: i16 = 5_000;
pub const MAX_MIN_CHARS: i16 = 5_000;

// Starboard Validation
pub const MIN_REQUIRED: i16 = -1;
pub const MAX_REQUIRED: i16 = 500;
pub const MIN_REQUIRED_REMOVE: i16 = -500;
pub const MAX_REQUIRED_REMOVE: i16 = 490;
pub const MIN_XP_MULTIPLIER: f32 = -10.0;
pub const MAX_XP_MULTIPLIER: f32 = 10.0;
pub const MAX_COOLDOWN_CAPACITY: i16 = 3600; // seconds in an hour
pub const MAX_COOLDOWN_PERIOD: i16 = MAX_COOLDOWN_CAPACITY;
