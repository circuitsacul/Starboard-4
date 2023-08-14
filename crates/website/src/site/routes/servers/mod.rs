mod api;
pub mod id;
pub mod server_list;

use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};

pub type BaseGuildsResource =
    Resource<(), Result<HashMap<Id<GuildMarker>, CurrentUserGuild>, ServerFnError>>;

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    let guilds: BaseGuildsResource =
        create_resource(cx, move || (), move |_| self::api::get_guilds(cx));
    provide_context(cx, guilds);

    let user = expect_context::<super::UserResource>(cx);

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
