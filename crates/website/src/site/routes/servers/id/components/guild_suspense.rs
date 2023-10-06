use leptos::*;
use twilight_model::user::CurrentUserGuild;

use crate::site::routes::servers::{
    id::{GuildContext, GuildData, GuildIdContext},
    BaseGuildsResource,
};

#[component]
pub fn BaseGuildSuspense<F, FIV, C, CIV>(fallback: F, child: C) -> impl IntoView
where
    F: Fn() -> FIV + 'static,
    FIV: IntoView,
    C: Fn(CurrentUserGuild) -> CIV + 'static,
    CIV: IntoView,
{
    let fallback = store_value(fallback);
    let child = store_value(child);

    view! {
        <Suspense fallback=move || {
            fallback.with_value(|f| f())
        }>
            {move || match get_base_guild() {
                Some(g) => child.with_value(|f| f(g)).into_view(),
                None => fallback.with_value(|f| f()).into_view(),
            }}

        </Suspense>
    }
}

#[component]
pub fn FlatGuildSuspense<F, FIV, C, CIV>(fallback: F, child: C) -> impl IntoView
where
    F: Fn() -> FIV + 'static,
    FIV: IntoView,
    C: Fn(GuildData) -> CIV + 'static,
    CIV: IntoView,
{
    let fallback = store_value(fallback);
    let child = store_value(child);

    view! {
        <Suspense fallback=move || {
            fallback.with_value(|f| f())
        }>
            {move || match get_flat_guild() {
                Some(g) => child.with_value(|f| f(g)).into_view(),
                None => fallback.with_value(|f| f()).into_view(),
            }}

        </Suspense>
    }
}

pub fn get_flat_guild() -> Option<GuildData> {
    let guild = expect_context::<GuildContext>();

    guild.get().and_then(|res| res.ok()).flatten()
}

pub fn get_base_guild() -> Option<CurrentUserGuild> {
    let base_guilds = expect_context::<BaseGuildsResource>();
    let guild_id = expect_context::<GuildIdContext>();

    base_guilds.with(|guilds| {
        let Some(Ok(guilds)) = guilds else {
            return None;
        };

        guilds.get(&guild_id.get()?).cloned()
    })
}
