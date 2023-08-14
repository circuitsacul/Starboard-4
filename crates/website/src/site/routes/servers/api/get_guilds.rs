use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use std::sync::Arc;

        use twilight_model::guild::Permissions;

        use crate::auth::context::{AuthContext, Guilds};
    }
}

use std::collections::HashMap;

use leptos::*;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};

#[cfg(feature = "ssr")]
pub async fn get_manageable_guilds(cx: Scope) -> Option<Arc<Guilds>> {
    let acx = AuthContext::get(cx)?;

    let _guard = acx.wlock.lock().await;
    if let Some(guilds) = acx.guilds.read().await.clone() {
        return Some(guilds);
    }

    let guilds: Arc<HashMap<_, _>> = Arc::new(
        acx.http
            .current_user_guilds()
            .await
            .ok()?
            .models()
            .await
            .ok()?
            .into_iter()
            .filter(|g| g.permissions.contains(Permissions::ADMINISTRATOR))
            .map(|g| (g.id, g))
            .collect(),
    );

    *acx.guilds.write().await = Some(guilds.clone());

    std::mem::drop(_guard);
    Some(guilds)
}

#[server(GetGuilds, "/api")]
pub async fn get_guilds(
    cx: Scope,
) -> Result<HashMap<Id<GuildMarker>, CurrentUserGuild>, ServerFnError> {
    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    Ok(guilds.iter().map(|(k, v)| (*k, v.clone())).collect())
}
