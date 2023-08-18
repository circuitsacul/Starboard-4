use database::Starboard;
use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

#[server(GetStarboard, "/api")]
pub async fn get_starboard(
    cx: Scope,
    guild_id: Id<GuildMarker>,
    starboard_id: i32,
) -> Result<Option<Starboard>, ServerFnError> {
    use crate::site::routes::servers::id::api::can_manage_guild;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);

    let sb = Starboard::get(&db, starboard_id).await?.and_then(|sb| {
        if sb.guild_id != guild_id.get() as i64 {
            None
        } else {
            Some(sb)
        }
    });

    Ok(sb)
}
