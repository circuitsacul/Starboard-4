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

pub struct SimpleEmoji {
    pub is_custom: bool,
    pub raw: String,
    pub as_id: Option<Id<EmojiMarker>>,
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
        if self.is_custom {
            RequestReactionType::Custom {
                name: None,
                id: self.as_id.unwrap(),
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
        if self.is_custom {
            let emoji_id = self.as_id.unwrap();
            if bot.cache.guild_emoji_exists(guild_id, emoji_id) {
                emoji_id.mention().to_string()
            } else {
                self.raw
            }
        } else {
            self.raw
        }
    }

    fn from_stored(raw: String) -> Self {
        let as_id = match Id::<EmojiMarker>::from_str(&raw) {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        Self {
            is_custom: as_id.is_some(),
            raw,
            as_id,
        }
    }

    fn into_stored(self) -> String {
        self.raw
    }

    fn from_user_input(
        input: String,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
    ) -> Option<Self> {
        // Get rid of the Variation-Selector-16 codepoint that is sometimes present in user
        // input. https://emojipedia.org/variation-selector-16/
        let input = input
            .strip_suffix('\u{fe0f}')
            .map_or_else(|| input.to_string(), |s| s.to_string());

        if emojis::get(&input).is_some() {
            Some(Self {
                is_custom: false,
                raw: input,
                as_id: None,
            })
        } else {
            let input: String = input.chars().filter(char::is_ascii_digit).collect();
            let as_id = Id::<EmojiMarker>::from_str(&input).ok()?;

            if !bot.cache.guild_emoji_exists(guild_id, as_id) {
                return None;
            }

            Some(Self {
                is_custom: true,
                raw: input,
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

    fn into_stored(self) -> Vec<String> {
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
                is_custom: true,
                raw: id.to_string(),
                as_id: Some(id),
            },
            ReactionType::Unicode { name } => SimpleEmoji {
                is_custom: false,
                raw: name,
                as_id: None,
            },
        }
    }
}
