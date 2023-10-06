mod api;
pub mod id;
pub mod server_list;

use std::collections::HashMap;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};

pub type BaseGuildsResource =
    Resource<(), Result<HashMap<Id<GuildMarker>, CurrentUserGuild>, ServerFnError>>;

#[component]
pub fn Servers() -> impl IntoView {
    let guilds: BaseGuildsResource = create_resource(move || (), move |_| self::api::get_guilds());
    provide_context(guilds);

    let user = expect_context::<super::UserResource>();

    let red = move || {
        user.with(|u| {
            if matches!(u, Some(Err(_))) {
                create_effect(|_| {
                    window().location().assign("/api/redirect").unwrap();
                });
            }
        });
    };
    view! {
        <Title text="Dashboard"/>
        <Suspense fallback=|| ()>{move || red()}</Suspense>
        <Outlet/>
    }
}
