//! Validation for certain starboard settings that are shared between
//! starboards and overrides, but not elsewhere and thus don't deserve
//! their own file.

use crate::constants;

pub fn validate_required(val: i16, required_remove: i16) -> Result<(), String> {
    if val <= required_remove {
        Err("`required` must be greater than `required-remove`.".to_string())
    } else if val < constants::MIN_REQUIRED {
        Err(format!(
            "`required` cannot be less than {}.",
            constants::MIN_REQUIRED
        ))
    } else if val > constants::MAX_REQUIRED {
        Err(format!(
            "`required` cannot be greater than {}.",
            constants::MAX_REQUIRED
        ))
    } else {
        Ok(())
    }
}

pub fn validate_required_remove(val: i16, required: i16) -> Result<(), String> {
    if val >= required {
        Err("`required-remove` must be less than `required`.".to_string())
    } else if val < constants::MIN_REQUIRED_REMOVE {
        Err(format!(
            "`required-remove` cannot be less than {}.",
            constants::MIN_REQUIRED_REMOVE
        ))
    } else if val > constants::MAX_REQUIRED_REMOVE {
        Err(format!(
            "`required-remove` cannot be greater than {}.",
            constants::MAX_REQUIRED_REMOVE
        ))
    } else {
        Ok(())
    }
}

pub fn validate_xp_multiplier(val: f32) -> Result<(), String> {
    if val > constants::MAX_XP_MULTIPLIER {
        Err(format!(
            "`xp-multiplier` cannot be greater than {}.",
            constants::MAX_XP_MULTIPLIER
        ))
    } else if val < constants::MIN_XP_MULTIPLIER {
        Err(format!(
            "`xp-multiplier` cannot be less than {}.",
            constants::MIN_XP_MULTIPLIER
        ))
    } else {
        Ok(())
    }
}

pub fn validate_cooldown(capacity: i16, period: i16) -> Result<(), String> {
    if capacity <= 0 || period <= 0 {
        Err("The capacity and period for the cooldown must be greater than 0.".to_string())
    } else if capacity > constants::MAX_COOLDOWN_CAPACITY {
        Err(format!(
            "The capacity cannot be greater than {}.",
            constants::MAX_COOLDOWN_CAPACITY
        ))
    } else if period > constants::MAX_COOLDOWN_PERIOD {
        Err(format!(
            "The period cannot be greater than {}.",
            constants::MAX_COOLDOWN_PERIOD
        ))
    } else {
        Ok(())
    }
}
