use crate::constants;

pub fn validate_name(name: &String) -> Result<String, String> {
    if name.len() > constants::MAX_NAME_LENGTH as usize {
        return Err(format!(
            "The name cannot be longer than {} characters.",
            constants::MAX_NAME_LENGTH
        ));
    }

    let filtered: String = name
        .replace(' ', "-")
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_lowercase() || *c == '_' || *c == '-')
        .collect();

    if filtered.len() < 3 {
        Err("The name must be at least 3 characters (special characters are excluded).".to_string())
    } else {
        Ok(filtered)
    }
}
