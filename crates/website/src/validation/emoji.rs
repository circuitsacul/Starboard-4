use std::str::FromStr;

use twilight_model::{guild::Guild, id::Id};

pub fn is_valid_emoji(emoji: &str, guild: &Guild) -> bool {
    if emojis::get(emoji).is_some() {
        return true;
    }

    let Ok(id) = Id::from_str(emoji) else {
        return false;
    };

    guild.emojis.iter().any(|e| e.id == id)
}
