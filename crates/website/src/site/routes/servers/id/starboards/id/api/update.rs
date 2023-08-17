#![allow(clippy::too_many_arguments)]

use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::site::components::form::ValidationErrors;

type Checkbox = Option<String>;

#[server(UpdateStarboard, "/api")]
pub async fn update_starboard(
    cx: Scope,
    guild_id: Id<GuildMarker>,
    starboard_id: i32,
    // general style
    display_emoji: Option<String>,
    ping_author: Checkbox,
    use_server_profile: Checkbox,
    extra_embeds: Checkbox,
    use_webhook: Checkbox,
    // embed style
    color: Option<String>,
    go_to_message: i16,
    attachments_list: Checkbox,
    replied_to: Checkbox,
) -> Result<ValidationErrors, ServerFnError> {
    use common::constants;
    use database::Starboard;

    use crate::{site::routes::servers::id::api::can_manage_guild, validation::is_valid_emoji};

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);
    let http = crate::bot_http(cx);

    let Some(mut sb) = Starboard::get(&db, starboard_id).await? else {
        return Err(ServerFnError::ServerError("Not found.".into()));
    };
    if sb.guild_id != guild_id.get() as i64 {
        return Err(ServerFnError::ServerError("Not found".into()));
    }

    let guild = http.guild(guild_id).await?.model().await?;

    let mut errors = ValidationErrors::new();

    // parse, validate, and set values
    if let Some(val) = &display_emoji {
        if is_valid_emoji(val, guild) {
            sb.settings.display_emoji = display_emoji;
        } else {
            errors.insert("display_emoji".into(), "Invalid emoji.".into());
        }
    } else {
        sb.settings.display_emoji = None;
    }

    sb.settings.ping_author = ping_author.is_some();
    sb.settings.use_server_profile = use_server_profile.is_some();
    sb.settings.extra_embeds = extra_embeds.is_some();
    sb.settings.use_webhook = use_webhook.is_some();

    if let Some(val) = &color {
        let val = val.trim_start_matches('#');
        let val = i32::from_str_radix(val, 16).ok().and_then(|v| {
            if v > constants::HEX_MAX {
                None
            } else {
                Some(v)
            }
        });
        if let Some(val) = val {
            sb.settings.color = Some(val);
        } else {
            errors.insert("color".into(), "Invalid hex value passed.".into());
        }
    } else {
        sb.settings.color = None;
    }

    if [0, 1, 2, 3].contains(&go_to_message) {
        sb.settings.go_to_message = go_to_message;
    } else {
        errors.insert("go_to_message".into(), "Invalid value.".into());
    }

    sb.settings.attachments_list = attachments_list.is_some();
    sb.settings.replied_to = replied_to.is_some();

    // update settings and return errors
    sb.update_settings(&db).await?;
    Ok(errors)
}
