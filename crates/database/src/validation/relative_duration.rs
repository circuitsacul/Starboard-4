use std::time::Duration;

use common::constants;

pub fn validate_relative_duration(
    newer_than: Option<i64>,
    older_than: Option<i64>,
) -> Result<(), String> {
    if let Some(newer_than) = newer_than {
        if let Some(older_than) = older_than {
            if older_than >= newer_than && older_than != 0 && newer_than != 0 {
                return Err("`older-than` must be less than `newer-than`.".to_string());
            }
        }
    }
    if let Some(older_than) = older_than {
        if older_than < 0 {
            return Err("`older-than` must be positive.".to_string());
        }
        if older_than > constants::MAX_OLDER_THAN {
            let ht =
                humantime::format_duration(Duration::from_secs(constants::MAX_OLDER_THAN as u64));
            return Err(format!("`older-than` cannot be greater than `{ht}`."));
        }
    }
    if let Some(newer_than) = newer_than {
        if newer_than < 0 {
            return Err("`newer-than` must be positive.".to_string());
        }
        if newer_than > constants::MAX_NEWER_THAN {
            let ht =
                humantime::format_duration(Duration::from_secs(constants::MAX_NEWER_THAN as u64));
            return Err(format!("`newer-than` cannot be greater than `{ht}`."));
        }
    }

    Ok(())
}
