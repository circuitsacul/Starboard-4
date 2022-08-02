use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

use crate::{
    client::bot::StarboardBot,
    database::{
        helpers::settings::overrides::call_with_override_settings, Starboard, StarboardOverride,
        StarboardSettings,
    },
    errors::{StarboardError, StarboardResult},
    unwrap_id,
};

#[derive(Debug)]
pub struct StarboardConfig {
    pub starboard: Starboard,
    pub overrides: Vec<StarboardOverride>,
    pub resolved: StarboardSettings,
}

macro_rules! update_from_override {
    ($settings: expr, $override: expr, $($field: ident),*) => {
        {
            $(
                match $override.$field {
                    Option::None => {},
                    Option::Some(value) => $settings.$field = value,
                }
            )*
        }
    };
}

impl StarboardConfig {
    pub fn new(
        starboard: Starboard,
        overrides: Vec<StarboardOverride>,
    ) -> serde_json::Result<Self> {
        let mut settings = starboard.settings.clone();
        for ov in overrides.iter() {
            call_with_override_settings!(update_from_override, settings, ov.get_overrides()?)
        }

        Ok(Self {
            starboard,
            overrides,
            resolved: settings,
        })
    }

    pub async fn list_for_channel(
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<Vec<Self>, StarboardError> {
        let starboards = Starboard::list_by_guild(&bot.pool, unwrap_id!(guild_id)).await?;
        let mut configs = Vec::new();

        let channel_id = unwrap_id!(channel_id);
        for sb in starboards.into_iter() {
            let overrides =
                StarboardOverride::list_by_starboard_and_channel(&bot.pool, sb.id, channel_id)
                    .await?;
            configs.push(Self::new(sb, overrides)?);
        }

        Ok(configs)
    }

    pub async fn is_guild_vote_emojis(
        bot: &StarboardBot,
        guild_id: i64,
        emoji_raw: &String,
    ) -> StarboardResult<bool> {
        if let Some(is_vote_emoji) = bot.cache.guild_vote_emojis.with(&guild_id, |_, emojis| {
            emojis.as_ref().map(|emojis| emojis.contains(emoji_raw))
        }) {
            Ok(is_vote_emoji)
        } else {
            let mut emojis = Vec::new();
            let starboards = Starboard::list_by_guild(&bot.pool, guild_id).await?;
            for sb in starboards {
                emojis.extend(sb.settings.upvote_emojis);
                emojis.extend(sb.settings.downvote_emojis);

                let configs = StarboardOverride::list_by_starboard(&bot.pool, sb.id).await?;
                for c in configs {
                    let ov = c.get_overrides()?;
                    if let Some(upvote_emojis) = ov.upvote_emojis {
                        emojis.extend(upvote_emojis);
                    }
                    if let Some(downvote_emojis) = ov.downvote_emojis {
                        emojis.extend(downvote_emojis);
                    }
                }
            }

            let is_vote_emoji = emojis.contains(emoji_raw);
            // cache the value
            bot.cache.guild_vote_emojis.insert(guild_id, emojis);

            Ok(is_vote_emoji)
        }
    }
}
