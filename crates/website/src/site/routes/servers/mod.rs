pub mod id;
pub mod server_list;

use leptos::*;
use leptos_router::*;
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::Arc;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};

#[cfg(feature = "ssr")]
use crate::auth::context::Guilds;

use super::UserResource;

pub type BaseGuildsResource =
    Resource<(), Result<HashMap<Id<GuildMarker>, CurrentUserGuild>, ServerFnError>>;

#[cfg(feature = "ssr")]
pub async fn get_manageable_guilds(cx: Scope) -> Option<Arc<Guilds>> {
    use twilight_model::guild::Permissions;

    use crate::auth::context::AuthContext;

    let acx = AuthContext::get(cx)?;

    if let Some(guilds) = acx.guilds.lock().unwrap().clone() {
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

    *acx.guilds.lock().unwrap() = Some(guilds.clone());

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

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    let guilds: BaseGuildsResource = create_resource(cx, move || (), move |_| get_guilds(cx));
    provide_context(cx, guilds);

    let user = expect_context::<UserResource>(cx);

    let red = move |cx| {
        user.with(cx, |u| {
            if u.is_err() {
                create_effect(cx, |_| {
                    window().location().assign("/api/redirect").unwrap();
                })
            }
        });
    };
    view! { cx,
        <Suspense fallback=|| ()>{move || red(cx)}</Suspense>
        <Outlet/>
    }
}
