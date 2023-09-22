use common::constants;
use errors::ErrToStr;

#[derive(Clone, Copy)]
pub enum NameErr {
    TooLong,
    TooShort,
}

impl ErrToStr for NameErr {
    fn to_bot_str(&self) -> String {
        match self {
            Self::TooLong => format!(
                "The name cannot be longer than {} characters.",
                constants::MAX_NAME_LENGTH
            ),
            Self::TooShort => "The name must be at least 3 characters.".into(),
        }
    }
    fn to_web_str(&self) -> String {
        match self {
            Self::TooLong => format!("Too long (max {}).", constants::MAX_NAME_LENGTH),
            Self::TooShort => "Too short (min 3).".into(),
        }
    }
}

pub fn validate_name(name: &str) -> Result<String, NameErr> {
    if name.len() > constants::MAX_NAME_LENGTH as usize {
        return Err(NameErr::TooLong);
    }

    let filtered: String = name
        .replace(' ', "-")
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_lowercase() || *c == '_' || *c == '-')
        .collect();

    if filtered.len() < 3 {
        Err(NameErr::TooShort)
    } else {
        Ok(filtered)
    }
}
