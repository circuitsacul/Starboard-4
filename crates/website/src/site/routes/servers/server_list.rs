use leptos::*;
use leptos_router::*;
use twilight_model::user::CurrentUserGuild;

#[server(GetGuilds, "/api")]
pub async fn get_guilds(cx: Scope) -> Result<Vec<CurrentUserGuild>, ServerFnError> {
    use super::get_manageable_guilds;

    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    let mut guilds: Vec<_> = guilds.iter().map(|(_, v)| v.clone()).collect();
    guilds.sort_by(|l, r| l.name.cmp(&r.name));

    Ok(guilds)
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
