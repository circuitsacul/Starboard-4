use std::str::FromStr;

use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_mention::Mention;
use twilight_model::{
    channel::message::ReactionType,
    id::{
        marker::{EmojiMarker, GuildMarker},
        Id,
    },
};

use crate::client::bot::StarboardBot;

/// Get rid of the Variation-Selector-16 codepoint that is sometimes present in user
/// input. https://emojipedia.org/variation-selector-16/
pub fn clean_emoji(unicode: &str) -> &str {
    unicode.strip_suffix('\u{fe0f}').unwrap_or(unicode)
}

#[derive(Clone)]
pub struct SimpleEmoji {
    raw: String,
    as_id: Option<Id<EmojiMarker>>,
}

impl PartialEq for SimpleEmoji {
    fn eq(&self, other: &Self) -> bool {
        clean_emoji(&other.raw) == clean_emoji(&self.raw)
    }
}

impl PartialEq<String> for SimpleEmoji {
    fn eq(&self, other: &String) -> bool {
        clean_emoji(&self.raw) == clean_emoji(other)
    }
}

pub trait EmojiCommon: Sized {
    type FromOut;
    type Stored;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String;
    fn from_user_input(
        input: String,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
    ) -> Self::FromOut;
    fn into_stored(self) -> Self::Stored;
    fn from_stored(stored: Self::Stored) -> Self;
}

impl SimpleEmoji {
    pub fn reactable(&self) -> RequestReactionType {
        if let Some(emoji_id) = self.as_id {
            RequestReactionType::Custom {
                name: None,
                id: emoji_id,
            }
        } else {
            RequestReactionType::Unicode { name: &self.raw }
        }
    }
}

impl EmojiCommon for SimpleEmoji {
    type FromOut = Option<Self>;
    type Stored = String;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String {
        if let Some(emoji_id) = self.as_id {
            match bot.cache.is_emoji_animated(guild_id, emoji_id) {
                None => self.raw,
                Some(true) => format!("<a:name:{emoji_id}>"),
                Some(false) => emoji_id.mention().to_string(),
            }
        } else {
            self.raw
        }
    }

    fn from_stored(raw: Self::Stored) -> Self {
        let as_id = match Id::<EmojiMarker>::from_str(&raw) {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        Self { raw, as_id }
    }

    fn into_stored(self) -> Self::Stored {
        self.raw
    }

    fn from_user_input(
        input: String,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
    ) -> Option<Self> {
        let cleaned_input = clean_emoji(&input).to_string();

        if emojis::get(&input).is_some() || emojis::get(&cleaned_input).is_some() {
            Some(Self {
                raw: input,
                as_id: None,
            })
        } else {
            let input = input.rsplit_once(':')?.1;
            let input = &input[..input.len() - 1];
            let as_id = Id::<EmojiMarker>::from_str(input).ok()?;

            if !bot.cache.guild_emoji_exists(guild_id, as_id) {
                return None;
            }

            Some(Self {
                raw: input.to_string(),
                as_id: Some(as_id),
            })
        }
    }
}

impl EmojiCommon for Vec<SimpleEmoji> {
    type FromOut = Self;
    type Stored = Vec<String>;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String {
        let mut arr = Vec::new();
        for emoji in self {
            arr.push(emoji.into_readable(bot, guild_id));
        }
        if arr.is_empty() {
            "no emojis".to_string()
        } else {
            arr.join(", ")
        }
    }

    fn from_stored(stored: Self::Stored) -> Self {
        let mut arr = Vec::new();
        for piece in stored {
            arr.push(SimpleEmoji::from_stored(piece));
        }
        arr
    }

    fn into_stored(self) -> Self::Stored {
        let mut arr = Vec::new();
        for emoji in self {
            arr.push(emoji.into_stored());
        }
        arr
    }

    fn from_user_input(input: String, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> Self {
        let mut arr = Vec::new();
        for piece in input.split(' ') {
            let emoji = SimpleEmoji::from_user_input(piece.to_string(), bot, guild_id);
            if let Some(emoji) = emoji {
                arr.push(emoji);
            }
        }
        arr
    }
}

impl From<ReactionType> for SimpleEmoji {
    fn from(reaction: ReactionType) -> Self {
        match reaction {
            ReactionType::Custom { id, .. } => SimpleEmoji {
                raw: id.to_string(),
                as_id: Some(id),
            },
            ReactionType::Unicode { name } => SimpleEmoji {
                raw: name,
                as_id: None,
            },
        }
    }
}
