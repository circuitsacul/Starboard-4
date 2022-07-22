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
    ) -> Result<Vec<Self>, SqlxOrSerdeError> {
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
}

pub enum SqlxOrSerdeError {
    Sqlx(sqlx::Error),
    Serde(serde_json::Error),
}

impl From<sqlx::Error> for SqlxOrSerdeError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

impl From<serde_json::Error> for SqlxOrSerdeError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}
