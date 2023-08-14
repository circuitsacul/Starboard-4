#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;

use leptos::*;
use twilight_model::id::{Id, marker::GuildMarker};

pub type ValidationErrors = HashMap<String, String>;

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
) -> Result<Result<(), ValidationErrors>, ServerFnError> {
    Ok(Ok(()))
}
