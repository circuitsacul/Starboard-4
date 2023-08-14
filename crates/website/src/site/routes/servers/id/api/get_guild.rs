use leptos::*;

use crate::site::routes::servers::id::GuildData;

#[cfg(feature = "ssr")]
pub async fn can_manage_guild(cx: Scope, id: u64) -> Result<(), ServerFnError> {
    use twilight_model::id::Id;

    use crate::site::routes::servers::api::get_manageable_guilds;

    if id == 0 {
        return Err(ServerFnError::ServerError(
            "ah yes, the 0 snowflake".to_string(),
        ));
    }

    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };
    if !guilds.contains_key(&Id::new(id)) {
        return Err(ServerFnError::ServerError(
            "You don't have permission to manage this server.".to_string(),
        ));
    }

    Ok(())
}

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: u64) -> Result<Option<GuildData>, ServerFnError> {
    use database::DbGuild;
    use twilight_model::id::Id;

    can_manage_guild(cx, id).await?;

    let db = crate::db(cx);
    let http = crate::bot_http(cx);

    let http_guild = match http.guild(Id::new(id)).await {
        Ok(res) => res.model().await?,
        Err(why) => {
            if errors::get_status(&why) == Some(404) {
                return Ok(None);
            } else {
                return Err(why.into());
            }
        }
    };
    let channels = http
        .guild_channels(Id::new(id))
        .await?
        .models()
        .await?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    let db_guild = match DbGuild::create(&db, id as i64).await? {
        Some(v) => v,
        None => DbGuild::get(&db, id as i64)
            .await?
            .expect("guild wasn't deleted"),
    };

    Ok(Some(GuildData {
        db: db_guild,
        http: http_guild,
        channels,
    }))
}
