#![allow(clippy::too_many_arguments)]

use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::site::components::form::ValidationErrors;

type Checkbox = Option<String>;

#[server(UpdateStarboard, "/api")]
pub async fn update_starboard(
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
    // requirements
    required: Option<i16>,
    required_remove: Option<i16>,
    upvote_emojis: String,
    downvote_emojis: String,
    self_vote: Checkbox,
    allow_bots: Checkbox,
    require_image: Checkbox,
    older_than: Option<String>,
    newer_than: Option<String>,
    matches: Option<String>,
    not_matches: Option<String>,
) -> Result<ValidationErrors, ServerFnError> {
    use common::constants;
    use database::{
        validation,
        validation::{
            regex::validate_regex,
            relative_duration::{validate_relative_duration, RelativeDurationErr},
        },
        DbGuild, Starboard,
    };
    use errors::ErrToStr;

    use crate::{site::routes::servers::id::api::can_manage_guild, validation::is_valid_emoji};

    can_manage_guild(guild_id).await?;

    let db = crate::db();
    let http = crate::bot_http();

    let Some(mut sb) = Starboard::get(&db, starboard_id).await? else {
        return Err(ServerFnError::ServerError("Not found.".into()));
    };
    if sb.guild_id != guild_id.get() as i64 {
        return Err(ServerFnError::ServerError("Not found".into()));
    }

    let guild = http.guild(guild_id).await?.model().await?;
    let db_guild = DbGuild::get(&db, sb.guild_id)
        .await?
        .expect("a db guild to exist");
    let premium = db_guild.premium_end.is_some();

    let mut errors = ValidationErrors::new();

    // parse, validate, and set values
    if let Some(val) = &display_emoji {
        if is_valid_emoji(val, &guild) {
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

    if let Some(required) = required {
        match validation::starboard_settings::validate_required(required, required_remove) {
            Ok(val) => sb.settings.required = Some(val),
            Err(why) => {
                errors.insert("required".into(), why.to_web_str());
            }
        };
    } else {
        sb.settings.required = None;
    }

    if let Some(required_remove) = required_remove {
        match validation::starboard_settings::validate_required_remove(required_remove, required) {
            Ok(val) => sb.settings.required_remove = Some(val),
            Err(why) => {
                errors.insert("required_remove".into(), why.to_web_str());
            }
        }
    } else {
        sb.settings.required_remove = None;
    }

    let upvote_emojis: Vec<_> = upvote_emojis
        .split(',')
        .map(|s| s.to_owned())
        .filter(|e| is_valid_emoji(e, &guild))
        .collect();
    let downvote_emojis: Vec<_> = downvote_emojis
        .split(',')
        .map(|s| s.to_owned())
        .filter(|e| is_valid_emoji(e, &guild))
        .collect();

    match validation::starboard_settings::validate_vote_emojis(
        &upvote_emojis,
        &downvote_emojis,
        premium,
    ) {
        Ok(()) => {
            sb.settings.upvote_emojis = upvote_emojis;
            sb.settings.downvote_emojis = downvote_emojis;
        }
        Err(why) => {
            errors.insert("upvote_emojis".into(), why.to_web_str());
            errors.insert("downvote_emojis".into(), why.to_web_str());
        }
    }

    'out: {
        let older_than = if let Some(older_than) = older_than {
            match common::parsing::relative_duration::parse_relative_duration(&older_than) {
                Ok(value) => Some(value),
                Err(why) => {
                    errors.insert("older_than".into(), why.to_web_str());
                    break 'out;
                }
            }
        } else {
            None
        };
        let newer_than = if let Some(newer_than) = newer_than {
            match common::parsing::relative_duration::parse_relative_duration(&newer_than) {
                Ok(value) => Some(value),
                Err(why) => {
                    errors.insert("newer_than".into(), why.to_web_str());
                    break 'out;
                }
            }
        } else {
            None
        };

        match validate_relative_duration(newer_than, older_than) {
            Ok(()) => {
                sb.settings.older_than = older_than.unwrap_or(0);
                sb.settings.newer_than = newer_than.unwrap_or(0);
            }
            Err(why @ RelativeDurationErr::OlderThanGreaterThanNewerThan) => {
                errors.insert("older_than".into(), why.to_web_str());
                errors.insert("newer_than".into(), why.to_web_str());
            }
            Err(
                why @ (RelativeDurationErr::OlderThanNegative
                | RelativeDurationErr::OlderThanTooLarge),
            ) => {
                errors.insert("older_than".into(), why.to_web_str());
            }
            Err(
                why @ (RelativeDurationErr::NewerThanNegative
                | RelativeDurationErr::NewerThanTooLarge),
            ) => {
                errors.insert("newer_than".into(), why.to_web_str());
            }
        }
    };

    sb.settings.self_vote = self_vote.is_some();
    sb.settings.allow_bots = allow_bots.is_some();
    sb.settings.require_image = require_image.is_some();

    if let Some(re) = matches {
        match validate_regex(re, premium) {
            Ok(val) => sb.settings.matches = val,
            Err(why) => {
                errors.insert("matches".into(), why.to_web_str());
            }
        }
    } else {
        sb.settings.matches = None;
    }

    if let Some(re) = not_matches {
        match validate_regex(re, premium) {
            Ok(val) => sb.settings.not_matches = val,
            Err(why) => {
                errors.insert("not_matches".into(), why.to_web_str());
            }
        }
    } else {
        sb.settings.not_matches = None;
    }

    // update settings and return errors
    sb.update_settings(&db).await?;
    Ok(errors)
}
