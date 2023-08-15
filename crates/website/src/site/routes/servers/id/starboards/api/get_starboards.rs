use std::collections::HashMap;

use database::Starboard;
use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

#[server(GetStarboards, "/api")]
pub async fn get_starboards(
    cx: Scope,
    guild_id: Id<GuildMarker>,
) -> Result<HashMap<i32, Starboard>, ServerFnError> {
    use crate::site::routes::servers::id::api::can_manage_guild;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);

    Starboard::list_by_guild(&db, guild_id.get() as i64)
        .await
        .map_err(|e| e.into())
        .map(|v| v.into_iter().map(|s| (s.id, s)).collect())
}
