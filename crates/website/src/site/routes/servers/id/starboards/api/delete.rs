pub use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

#[server(DeleteStarboard, "/api")]
pub async fn delete_starboard(
    cx: Scope,
    guild_id: Id<GuildMarker>,
    starboard_id: i32,
) -> Result<(), ServerFnError> {
    use crate::site::routes::servers::id::api::can_manage_guild;
    use leptos_actix::redirect;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);

    database::Starboard::delete_by_id(&db, guild_id.get() as _, starboard_id).await?;

    redirect(cx, &format!("/servers/{guild_id}/starboards"));

    Ok(())
}
