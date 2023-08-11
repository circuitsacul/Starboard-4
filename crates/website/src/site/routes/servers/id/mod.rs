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
    user::CurrentUserGuild,
};

use crate::site::components::ToastedSusp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
    pub channels: HashMap<Id<ChannelMarker>, Channel>,
}

pub type GuildContext = Resource<Option<u64>, Result<Option<GuildData>, ServerFnError>>;
pub type GuildIdContext = Memo<Option<Id<GuildMarker>>>;

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
    let base_guilds = expect_context::<super::BaseGuildsResource>(cx);
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

#[cfg(feature = "ssr")]
pub async fn can_manage_guild(cx: Scope, id: u64) -> Result<(), ServerFnError> {
    if id == 0 {
        return Err(ServerFnError::ServerError(
            "ah yes, the 0 snowflake".to_string(),
        ));
    }

    use crate::site::routes::servers::get_manageable_guilds;

    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };
    if !guilds.contains_key(&Id::new(id)) {
        return Err(ServerFnError::ServerError(
            "You don't have permission to manage this server.".to_string(),
        ));
    }

    Ok(())
}

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: u64) -> Result<Option<GuildData>, ServerFnError> {
    use twilight_model::id::Id;

    can_manage_guild(cx, id).await?;

    let db = crate::db(cx);
    let http = crate::bot_http(cx);

    let http_guild = match http.guild(Id::new(id)).await {
        Ok(res) => res.model().await?,
        Err(why) => {
            if errors::get_status(&why) == Some(404) {
                return Ok(None);
            } else {
                return Err(why.into());
            }
        }
    };
    let channels = http
        .guild_channels(Id::new(id))
        .await?
        .models()
        .await?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    let db_guild = match DbGuild::create(&db, id as i64).await? {
        Some(v) => v,
        None => DbGuild::get(&db, id as i64)
            .await?
            .expect("guild wasn't deleted"),
    };

    Ok(Some(GuildData {
        db: db_guild,
        http: http_guild,
        channels,
    }))
}

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
        move || guild_id.get().map(|id| id.get()),
        move |id| async move {
            let Some(id) = id else {
                return Err(ServerFnError::Args("Invalid request.".to_string()));
            };
            get_guild(cx, id).await
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
            <dialog class=move || format!("modal {}", if visible(cx) { "modal-open" } else { "" })>
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
        </Suspense>
    }
}
