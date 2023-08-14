use leptos::*;
use twilight_model::user::CurrentUserGuild;

use crate::site::routes::servers::{
    id::{GuildContext, GuildData, GuildIdContext},
    BaseGuildsResource,
};

#[component]
pub fn BaseGuildSuspense<F, FIV, C, CIV>(cx: Scope, fallback: F, child: C) -> impl IntoView
where
    F: Fn() -> FIV + 'static,
    FIV: IntoView,
    C: Fn(CurrentUserGuild) -> CIV + 'static,
    CIV: IntoView,
{
    let fallback = store_value(cx, fallback);
    let child = store_value(cx, child);

    view! { cx,
        <Suspense fallback=move || {
            fallback.with_value(|f| f())
        }>
            {move || match get_base_guild(cx) {
                Some(g) => child.with_value(|f| f(g)).into_view(cx),
                None => fallback.with_value(|f| f()).into_view(cx),
            }}

        </Suspense>
    }
}

#[component]
pub fn FlatGuildSuspense<F, FIV, C, CIV>(cx: Scope, fallback: F, child: C) -> impl IntoView
where
    F: Fn() -> FIV + 'static,
    FIV: IntoView,
    C: Fn(GuildData) -> CIV + 'static,
    CIV: IntoView,
{
    let fallback = store_value(cx, fallback);
    let child = store_value(cx, child);

    view! { cx,
        <Suspense fallback=move || {
            fallback.with_value(|f| f())
        }>
            {move || match get_flat_guild(cx) {
                Some(g) => child.with_value(|f| f(g)).into_view(cx),
                None => fallback.with_value(|f| f()).into_view(cx),
            }}

        </Suspense>
    }
}

pub fn get_flat_guild(cx: Scope) -> Option<GuildData> {
    let guild = expect_context::<GuildContext>(cx);

    guild.read(cx).and_then(|res| res.ok()).flatten()
}

pub fn get_base_guild(cx: Scope) -> Option<CurrentUserGuild> {
    let base_guilds = expect_context::<BaseGuildsResource>(cx);
    let guild_id = expect_context::<GuildIdContext>(cx);

    base_guilds
        .with(cx, |guilds| {
            let Ok(guilds) = guilds else {
                return None;
            };

            guilds.get(&guild_id.get()?).cloned()
        })
        .flatten()
}
