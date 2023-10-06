pub mod add;
mod api;
pub mod id;

use leptos::{logging::log, *};
use leptos_icons::*;
use leptos_router::*;

use crate::site::components::{
    form::ValidationErrors, toast, Card, CardList, CardSkeleton, Toast, ToastedSusp,
};

use super::GuildIdContext;

pub type CreateStarboardAction =
    Action<self::api::CreateStarboard, Result<ValidationErrors, ServerFnError>>;
pub type DeleteStarboardAction = Action<self::api::DeleteStarboard, Result<(), ServerFnError>>;

#[component]
pub fn Starboards() -> impl IntoView {
    let create_sb: CreateStarboardAction = create_server_action::<self::api::CreateStarboard>();
    let delete_sb: DeleteStarboardAction = create_server_action::<self::api::DeleteStarboard>();
    provide_context(create_sb);
    provide_context(delete_sb);

    let guild_id = expect_context::<GuildIdContext>();

    let starboards = create_resource(
        move || {
            (
                guild_id.get(),
                (create_sb.version().get(), delete_sb.version().get()),
            )
        },
        move |(guild_id, _)| async move {
            let Some(guild_id) = guild_id else {
                return Err(ServerFnError::ServerError("No guild ID.".to_string()));
            };
            self::api::get_starboards(guild_id).await
        },
    );

    create_effect(move |_| {
        if let Some(Err(why)) = create_sb.value().get() {
            if matches!(why, ServerFnError::Deserialization(_)) {
                return;
            }
            toast(Toast::error(why));
        }
    });
    create_effect(move |_| {
        if let Some(Err(why)) = delete_sb.value().get() {
            if matches!(why, ServerFnError::Deserialization(_)) {
                return;
            }
            toast(Toast::error(why));
        }
    });

    let starboards_view = move || {
        starboards.get().map(|sb| {
            sb.map(|sb| {
                let sb = store_value(sb);
                view! {
                    <For
                        each=move || sb.with_value(|sb| sb.clone())
                        key=|sb| sb.0
                        children=move |sb| {
                            view! {<Card title=sb.1.name href=sb.0.to_string()/> }
                        }
                    />
                }
            })
            .map_err(|e| e.clone())
        })
    };

    view! {
        <Outlet/>
        <CardList>
            <div class="flex justify-end">
                <A href="add" class="btn btn-outline">
                    <Icon icon=crate::icon!(FaPlusSolid)/>
                    "New Starboard"
                </A>
            </div>

            <ToastedSusp fallback=move || {
                view! {
                    <For each=|| 0..10 key=|t| *t children=move |_| view! { <CardSkeleton/> }/>
                }
            }>{starboards_view}</ToastedSusp>
        </CardList>
    }
}
