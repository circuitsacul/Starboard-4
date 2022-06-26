use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use twilight_mention::Mention;
use twilight_model::id::{marker::EmojiMarker, Id};

use crate::client::bot::StarboardBot;

pub struct SimpleEmoji {
    pub is_custom: bool,
    pub raw: String,
    as_id: Option<Id<EmojiMarker>>,
}

#[async_trait]
pub trait EmojiCommon: Sized {
    type FromOut;
    type Stored;

    async fn into_readable(self, bot: &Arc<StarboardBot>, guild_id: i64) -> String;
    async fn from_user_input(
        input: String,
        bot: &Arc<StarboardBot>,
        guild_id: i64,
    ) -> Self::FromOut;
    fn into_stored(self) -> Self::Stored;
    fn from_stored(stored: Self::Stored) -> Self;
}

impl SimpleEmoji {
    pub fn into_reactable(self) -> String {
        if self.is_custom {
            format!("name:{}", self.raw)
        } else {
            self.raw
        }
    }
}

#[async_trait]
impl EmojiCommon for SimpleEmoji {
    type FromOut = Option<Self>;
    type Stored = String;

    async fn into_readable(self, bot: &Arc<StarboardBot>, guild_id: i64) -> String {
        if self.is_custom {
            match bot.cache.read().await.emoji(self.as_id.unwrap()) {
                None => self.raw,
                Some(value) => {
                    if value.guild_id() == guild_id {
                        value.id().mention().to_string()
                    } else {
                        self.raw
                    }
                }
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

    async fn from_user_input(
        input: String,
        bot: &Arc<StarboardBot>,
        guild_id: i64,
    ) -> Option<Self> {
        if emojis::get(&input).is_some() {
            Some(Self {
                is_custom: false,
                raw: input,
                as_id: None,
            })
        } else {
            let input: String = input.chars().filter(|c| c.is_digit(10)).collect();
            let as_id = Id::<EmojiMarker>::from_str(&input).ok()?;

            match bot.cache.read().await.emoji(as_id) {
                None => return None,
                Some(cached) => {
                    if cached.guild_id() == guild_id {
                    } else {
                        return None;
                    }
                }
            }

            Some(Self {
                is_custom: true,
                raw: input,
                as_id: Some(as_id),
            })
        }
    }
}

#[async_trait]
impl EmojiCommon for Vec<SimpleEmoji> {
    type FromOut = Self;
    type Stored = Vec<String>;

    async fn into_readable(self, bot: &Arc<StarboardBot>, guild_id: i64) -> String {
        let mut arr = Vec::new();
        for emoji in self.into_iter() {
            arr.push(emoji.into_readable(bot, guild_id).await)
        }
        if arr.len() == 0 {
            "no emojis".to_string()
        } else {
            arr.join(", ")
        }
    }

    fn from_stored(stored: Self::Stored) -> Self {
        let mut arr = Vec::new();
        for piece in stored.into_iter() {
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

    async fn from_user_input(input: String, bot: &Arc<StarboardBot>, guild_id: i64) -> Self {
        let mut arr = Vec::new();
        for piece in (&input).split(" ").into_iter() {
            let emoji = SimpleEmoji::from_user_input(piece.to_string(), bot, guild_id).await;
            if let Some(emoji) = emoji {
                arr.push(emoji)
            }
        }
        arr
    }
}
