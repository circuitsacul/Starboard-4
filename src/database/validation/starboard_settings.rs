//! Validation for certain starboard settings that are shared between
//! starboards and overrides, but not elsewhere and thus don't deserve
//! their own file.

use std::{collections::HashSet, time::Duration};

use crate::constants;

pub fn none_or_number(val: String) -> Result<Option<i16>, String> {
    if val == "none" {
        return Ok(None);
    }

    let ret = val.parse::<i16>();
    match ret {
        Ok(val) => Ok(Some(val)),
        Err(_) => Err(format!("I couldn't interpret {val} as a number.")),
    }
}

pub fn validate_required(val: String, required_remove: Option<i16>) -> Result<Option<i16>, String> {
    let Some(val) = none_or_number(val)? else {
        return Ok(None);
    };

    if let Some(required_remove) = required_remove {
        if val <= required_remove {
            return Err("`required` must be greater than `required-remove`.".to_string());
        }
    }

    if val < constants::MIN_REQUIRED {
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
        Ok(Some(val))
    }
}

pub fn validate_required_remove(val: String, required: Option<i16>) -> Result<Option<i16>, String> {
    let Some(val) = none_or_number(val)? else {
        return Ok(None);
    };

    if let Some(required) = required {
        if val >= required {
            return Err("`required-remove` must be less than `required`.".to_string());
        }
    }

    if val < constants::MIN_REQUIRED_REMOVE {
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
        Ok(Some(val))
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

pub fn validate_vote_emojis(
    upvote: &[String],
    downvote: &[String],
    premium: bool,
) -> Result<(), String> {
    let unique_upvote: HashSet<_> = upvote.iter().collect();
    let unique_downvote: HashSet<_> = downvote.iter().collect();

    if unique_upvote
        .intersection(&unique_downvote)
        .next()
        .is_some()
    {
        return Err(
            "`upvote-emojis` and `downvote-emojis` cannot share the same emojis.".to_string(),
        );
    }

    let limit = if premium {
        constants::MAX_PREM_VOTE_EMOJIS
    } else {
        constants::MAX_VOTE_EMOJIS
    };

    if unique_upvote.len() + unique_downvote.len() > limit {
        return Err(format!(
            concat!(
                "You cannot have more than {} upvote and downvote emojis per starbard. ",
                "The premium limit is {}.",
            ),
            limit,
            constants::MAX_PREM_VOTE_EMOJIS,
        ));
    }

    Ok(())
}

pub fn validate_relative_duration(newer_than: i64, older_than: i64) -> Result<(), String> {
    if older_than >= newer_than && older_than != 0 && newer_than != 0 {
        return Err("`older-than` must be less than `newer-than`.".to_string());
    }
    if older_than < 0 {
        return Err("`older-than` must be positive.".to_string());
    }
    if newer_than < 0 {
        return Err("`newer-than` must be positive.".to_string());
    }
    if older_than > constants::MAX_OLDER_THAN {
        let ht = humantime::format_duration(Duration::from_secs(constants::MAX_OLDER_THAN as u64));
        return Err(format!("`older-than` cannot be greater than `{ht}`."));
    }
    if newer_than > constants::MAX_NEWER_THAN {
        let ht = humantime::format_duration(Duration::from_secs(constants::MAX_NEWER_THAN as u64));
        return Err(format!("`newer-than` cannot be greater than `{ht}`."));
    }

    Ok(())
}
