use leptos::*;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

/// TODO: validate channel existence and type
/// TODO: validate name
#[server(CreateStarboard, "/api")]
pub async fn create_starboard(
    cx: Scope,
    guild_id: Id<GuildMarker>,
    channel_id: Id<ChannelMarker>,
    name: String,
) -> Result<(), ServerFnError> {
    use database::Starboard;
    use leptos_actix::redirect;

    use crate::site::routes::servers::id::api::can_manage_guild;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);

    let sb = Starboard::create(&db, &name, channel_id.get() as _, guild_id.get() as _).await?;
    let Some(sb) = sb else {
        return Err(ServerFnError::ServerError(
            "That name is already in use.".into(),
        ));
    };

    redirect(
        cx,
        &format!("/servers/{}/starboards/{}", guild_id, &sb.id.to_string()),
    );

    Ok(())
}
