use database::Starboard;
use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

#[server(GetStarboard, "/api")]
pub async fn get_starboard(
    cx: Scope,
    guild_id: Id<GuildMarker>,
    starboard_id: i32,
) -> Result<(Option<Starboard>, Option<String>), ServerFnError> {
    use crate::site::routes::servers::id::api::can_manage_guild;
    use errors::get_status;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);
    let http = crate::bot_http(cx);

    let Some(sb) = Starboard::get(&db, starboard_id).await? else {
        return Ok((None, None));
    };
    if sb.guild_id != guild_id.get() as i64 {
        return Ok((None, None));
    }

    // TODO: caching
    let channel = http.channel(Id::new(sb.channel_id as _)).await;
    let name = match channel {
        Ok(resp) => resp.model().await?.name,
        Err(why) => {
            if get_status(&why) == Some(404) {
                None
            } else {
                return Err(why.into());
            }
        }
    };

    Ok((Some(sb), name))
}
