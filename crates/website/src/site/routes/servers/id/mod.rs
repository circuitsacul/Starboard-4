mod api;
mod components;
pub mod overview;
mod sidebar;
pub mod starboards;

use std::collections::HashMap;

use sidebar::{SideBar, Tab};

use database::DbGuild;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::Channel,
    guild::Guild,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use crate::site::components::ToastedSusp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
    pub channels: HashMap<Id<ChannelMarker>, Channel>,
}

pub type GuildContext = Resource<Option<Id<GuildMarker>>, Result<Option<GuildData>, ServerFnError>>;
pub type GuildIdContext = Memo<Option<Id<GuildMarker>>>;

#[derive(Params, PartialEq)]
struct Props {
    guild_id: u64,
}

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let location = use_location(cx);
    let params = use_params::<Props>(cx);

    let guild_id: GuildIdContext = create_memo(cx, move |_| {
        params.with(|p| p.as_ref().ok().map(|p| Id::new(p.guild_id)))
    });

    let guild: GuildContext = create_resource(
        cx,
        move || guild_id.get(),
        move |guild_id| async move {
            let Some(guild_id) = guild_id else {
                return Err(ServerFnError::Args("Invalid request.".to_string()));
            };
            self::api::get_guild(cx, guild_id).await
        },
    );

    provide_context(cx, guild);
    provide_context(cx, guild_id);

    let tab = create_memo(cx, move |_| {
        match location.pathname.get().split('/').nth(3).unwrap_or("") {
            "starboards" => Tab::Starboards,
            "overrides" => Tab::Overrides,
            "filters" => Tab::Filters,
            "permroles" => Tab::PermRoles,
            "awardroles" => Tab::AwardRoles,
            "autostar" => Tab::AutoStar,
            _ => Tab::Overview,
        }
    });

    view! { cx,
        <ToastedSusp fallback=|| ()>
            {move || guild.with(cx, |g| g.as_ref().map(|_| ()).map_err(|e| e.clone()))}
        </ToastedSusp>
        <InviteModal/>
        <SideBar active=tab/>
    }
}

#[component]
fn InviteModal(cx: Scope) -> impl IntoView {
    let guild = expect_context::<GuildContext>(cx);
    let guild_id = expect_context::<GuildIdContext>(cx);
    let url = create_memo(cx, move |_| {
        guild_id
            .get()
            .map(|id| format!("/api/redirect?guild_id={id}"))
    });

    let visible = move |cx: Scope| guild.with(cx, |g| matches!(g, Ok(None))).unwrap_or(false);

    view! { cx,
        <Suspense fallback=|| ()>
            <Show
                when=move || visible(cx)
                fallback=|_| ()
            >
                <dialog class="modal modal-open">
                    <form method="dialog" class="modal-box">
                        <h3 class="font-bold text-lg">"Server Needs Setup"</h3>
                        <p class="py-4">"Please add Starboard to this server to continue."</p>
                        <div class="modal-action">
                            <A class="btn btn-ghost" href="..">
                                "Go Back"
                            </A>
                            <a class="btn btn-primary" href=move || url.get() rel="external">
                                "Invite"
                            </a>
                        </div>
                    </form>
                </dialog>
            </Show>
        </Suspense>
    }
}
