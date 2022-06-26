use std::{borrow::Cow, str::FromStr, sync::Arc};

use async_trait::async_trait;
use twilight_mention::Mention;
use twilight_model::id::{marker::EmojiMarker, Id};

use crate::client::bot::StarboardBot;

pub struct SimpleEmoji {
    pub is_custom: bool,
    pub name: String,
    as_id: Option<Id<EmojiMarker>>,
}

#[async_trait]
pub trait EmojiCommon: Sized {
    type FromOut;
    type Stored;

    async fn into_readable(self, bot: &Arc<StarboardBot>) -> String;
    fn from_user_input(input: Cow<str>) -> Self::FromOut;
    fn from_stored(stored: Self::Stored) -> Self;
}

impl SimpleEmoji {
    pub fn into_reactable(self) -> String {
        if self.is_custom {
            format!("name:{}", self.name)
        } else {
            self.name
        }
    }
}

#[async_trait]
impl EmojiCommon for SimpleEmoji {
    type FromOut = Option<Self>;
    type Stored = String;

    async fn into_readable(self, bot: &Arc<StarboardBot>) -> String {
        if self.is_custom {
            match bot.cache.read().await.emoji(self.as_id.unwrap()) {
                None => self.name,
                Some(value) => value.id().mention().to_string(),
            }
        } else {
            self.name
        }
    }

    fn from_stored(name: String) -> Self {
        let as_id = match Id::<EmojiMarker>::from_str(&name) {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        Self {
            is_custom: as_id.is_some(),
            name,
            as_id,
        }
    }

    fn from_user_input(input: Cow<str>) -> Option<Self> {
        if emojis::get(&input).is_some() {
            Some(Self {
                is_custom: false,
                name: input.into_owned(),
                as_id: None,
            })
        } else {
            let input: String = input.chars().filter(|c| c.is_digit(10)).collect();
            let as_id = Id::<EmojiMarker>::from_str(&input).ok()?;
            Some(Self {
                is_custom: true,
                name: input,
                as_id: Some(as_id),
            })
        }
    }
}

#[async_trait]
impl EmojiCommon for Vec<SimpleEmoji> {
    type FromOut = Self;
    type Stored = Vec<String>;

    async fn into_readable(self, bot: &Arc<StarboardBot>) -> String {
        let mut arr = Vec::new();
        for emoji in self.into_iter() {
            arr.push(emoji.into_readable(bot).await)
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

    fn from_user_input(input: Cow<str>) -> Self {
        let mut arr = Vec::new();
        for piece in (&input).split(" ").into_iter() {
            let emoji = SimpleEmoji::from_user_input(Cow::Borrowed(piece));
            if let Some(emoji) = emoji {
                arr.push(emoji)
            }
        }
        arr
    }
}
