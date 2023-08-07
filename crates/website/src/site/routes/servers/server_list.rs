use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};

#[server(GetGuilds, "/api")]
pub async fn get_guilds(
    cx: Scope,
) -> Result<HashMap<Id<GuildMarker>, CurrentUserGuild>, ServerFnError> {
    use crate::auth::context::AuthContext;

    let Some(auth) = AuthContext::get(cx) else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    let new_guilds: HashMap<_, _> = auth
        .http
        .current_user_guilds()
        .await?
        .models()
        .await?
        .into_iter()
        .map(|g| (g.id, g))
        .collect();

    Ok(new_guilds)
}

#[component]
pub fn ServerList(cx: Scope) -> impl IntoView {
    let guilds = create_resource(cx, move || (), move |_| get_guilds(cx));

    view! { cx,
        <A href="945149610484195398" class="link">
            "Go to server"
        </A>
        <Suspense fallback=|| ()>{move || guilds.with(cx, |d| format!("{d:?}"))}</Suspense>
    }
}
