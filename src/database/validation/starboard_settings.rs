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
