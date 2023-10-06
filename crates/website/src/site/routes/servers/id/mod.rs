mod api;
mod components;
pub mod overview;
mod sidebar;
pub mod starboards;

use sidebar::{SideBar, Tab};

use database::DbGuild;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Guild,
    id::{marker::GuildMarker, Id},
};

use crate::site::components::{Popup, ToastedSusp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
}

pub type GuildContext = Resource<Option<Id<GuildMarker>>, Result<Option<GuildData>, ServerFnError>>;
pub type GuildIdContext = Memo<Option<Id<GuildMarker>>>;

#[derive(Params, PartialEq)]
struct Props {
    guild_id: u64,
}

#[component]
pub fn Server() -> impl IntoView {
    let location = use_location();
    let params = use_params::<Props>();

    let guild_id: GuildIdContext =
        create_memo(move |_| params.with(|p| p.as_ref().ok().map(|p| Id::new(p.guild_id))));

    let guild: GuildContext = create_resource(
        move || guild_id.get(),
        move |guild_id| async move {
            let Some(guild_id) = guild_id else {
                return Err(ServerFnError::Args("Invalid request.".to_string()));
            };
            self::api::get_guild(guild_id).await
        },
    );

    provide_context(guild);
    provide_context(guild_id);

    let tab = create_memo(
        move |_| match location.pathname.get().split('/').nth(3).unwrap_or("") {
            "starboards" => Tab::Starboards,
            "overrides" => Tab::Overrides,
            "filters" => Tab::Filters,
            "permroles" => Tab::PermRoles,
            "awardroles" => Tab::AwardRoles,
            "autostar" => Tab::AutoStar,
            _ => Tab::Overview,
        },
    );

    view! {
        <ToastedSusp fallback=|| ()>
            {move || guild.with(
                |g| g.as_ref().map(|v| v.as_ref().map(|_| ()).map_err(|e| e.clone()))
            )}
        </ToastedSusp>
        <InviteModal/>
        <SideBar active=tab/>
    }
}

#[component]
fn InviteModal() -> impl IntoView {
    let guild = expect_context::<GuildContext>();
    let guild_id = expect_context::<GuildIdContext>();
    let url = create_memo(move |_| {
        guild_id
            .get()
            .map(|id| format!("/api/redirect?guild_id={id}"))
    });

    let visible = move || guild.with(|g| matches!(g, Some(Ok(None))));

    view! {
        <Suspense fallback=|| ()>
            <Show when=move || visible() fallback=|| ()>
                <Popup
                    title=|| "Server Needs Setup"
                    actions=move || {
                        view! {
                            <div class="flex-1"></div>
                            <A class="btn btn-ghost" href="..">
                                "Go Back"
                            </A>
                            <a class="btn btn-primary" href=move || url.get() rel="external">
                                "Invite"
                            </a>
                        }
                    }
                >

                    "Please add Starboard to this server to continue."
                </Popup>
            </Show>
        </Suspense>
    }
}
