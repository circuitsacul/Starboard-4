use crate::constants;

pub fn validate_regex(input: String, is_premium: bool) -> Result<Option<String>, String> {
    if !is_premium {
        return Err("The `matches` and `not-matches` settings require premium.".to_string());
    }

    if input.len() > constants::MAX_REGEX_LENGTH as usize {
        return Err(format!(
            "The `matches` and `not-matches` settings must be under {} characters.",
            constants::MAX_REGEX_LENGTH,
        ));
    }

    if input == ".*" {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}
