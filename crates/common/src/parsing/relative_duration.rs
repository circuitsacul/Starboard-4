use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

use errors::ErrToStr;

use crate::constants;

#[derive(Debug, Clone)]
pub enum RelativeDurationParseErr {
    UnparsableToken(String),
    UnknownUnit((String, String)),
}

impl ErrToStr for RelativeDurationParseErr {
    fn to_bot_str(&self) -> String {
        match self {
            Self::UnparsableToken(token) => {
                format!("I couldn't interpret `{token}` as a unit of time.")
            }
            Self::UnknownUnit((unit, token)) => {
                format!("I don't know what `{unit}` is (you said `{token}{unit}`).")
            }
        }
    }

    fn to_web_str(&self) -> String {
        match self {
            Self::UnparsableToken(token) => format!("Couldn't parse '{token}' as a unit of time."),
            Self::UnknownUnit((unit, value)) => {
                format!("Couldn't interpret '{unit}' as a unit of time (from '{value}{unit}').")
            }
        }
    }
}

fn normalize_unit(unit: &str) -> &str {
    if unit == "s" {
        return unit;
    }
    let unit = unit.strip_suffix('s').unwrap_or(unit);
    match unit {
        "second" => "s",
        "minute" => "m",
        "hour" => "h",
        "day" => "d",
        "week" => "w",
        "month" => "mo",
        "year" => "y",
        _ => unit,
    }
}

fn unit_conversion(unit: &str) -> Option<i64> {
    match unit {
        "s" => Some(1),
        "m" => Some(60),
        "h" => Some(60 * 60),
        "d" => Some(60 * 60 * 24),
        "w" => Some(60 * 60 * 24 * 7),
        "mo" => Some(constants::MONTH_SECONDS),
        "y" => Some(constants::YEAR_SECONDS),
        _ => None,
    }
}

pub fn parse_relative_duration(inp: &str) -> Result<i64, RelativeDurationParseErr> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>\w+)$").unwrap();
    }

    let mut seconds = 0;
    let mut carry = None;
    for raw_token in inp.trim().split(' ').map(|t| t.trim()) {
        let token;
        if let Some(carry_val) = carry {
            token = Cow::Owned(format!("{carry_val}{raw_token}"));
            carry = None;
        } else if raw_token.chars().all(char::is_numeric) {
            carry = Some(raw_token);
            continue;
        } else {
            token = Cow::Borrowed(raw_token);
        }

        let found = match RE.captures(&token) {
            None => return Err(RelativeDurationParseErr::UnparsableToken(token.into())),
            Some(found) => found,
        };

        let value: i64 = match found.name("value").unwrap().as_str().parse() {
            Err(_) => return Err(RelativeDurationParseErr::UnparsableToken(token.into())),
            Ok(value) => value,
        };
        let unit = normalize_unit(found.name("unit").unwrap().as_str());
        let conversion = match unit_conversion(unit) {
            None => {
                return Err(RelativeDurationParseErr::UnknownUnit((
                    unit.into(),
                    value.to_string(),
                )))
            }
            Some(conversion) => conversion,
        };

        seconds += value * conversion;
    }

    Ok(seconds)
}
