use leptos::*;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::site::routes::servers::id::GuildData;

#[cfg(feature = "ssr")]
pub async fn can_manage_guild(cx: Scope, guild_id: Id<GuildMarker>) -> Result<(), ServerFnError> {
    use crate::site::routes::servers::api::get_manageable_guilds;

    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };
    if !guilds.contains_key(&guild_id) {
        return Err(ServerFnError::ServerError(
            "You don't have permission to manage this server.".to_string(),
        ));
    }

    Ok(())
}

#[server(GetGuild, "/api")]
pub async fn get_guild(
    cx: Scope,
    guild_id: Id<GuildMarker>,
) -> Result<Option<GuildData>, ServerFnError> {
    use database::DbGuild;

    can_manage_guild(cx, guild_id).await?;

    let db = crate::db(cx);
    let http = crate::bot_http(cx);

    let http_guild = match http.guild(guild_id).await {
        Ok(res) => res.model().await?,
        Err(why) => {
            if errors::get_status(&why) == Some(404) {
                return Ok(None);
            } else {
                return Err(why.into());
            }
        }
    };
    let db_guild = match DbGuild::create(&db, guild_id.get() as i64).await? {
        Some(v) => v,
        None => DbGuild::get(&db, guild_id.get() as i64)
            .await?
            .expect("guild wasn't deleted"),
    };

    Ok(Some(GuildData {
        db: db_guild,
        http: http_guild,
    }))
}
