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
    user::CurrentUserGuild,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
}

pub type GuildContext = Resource<Option<u64>, Result<Option<GuildData>, ServerFnError>>;
pub type GuildIdContext = Memo<Option<Id<GuildMarker>>>;

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

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: u64) -> Result<Option<GuildData>, ServerFnError> {
    use crate::auth::context::AuthContext;
    use twilight_model::id::Id;

    let Some(acx) = AuthContext::get(cx) else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };
    if !acx
        .guilds
        .lock()?
        .as_ref()
        .map(|g| g.contains_key(&Id::new(id)))
        .unwrap_or(false)
    {
        return Err(ServerFnError::ServerError(
            "You don't have permission to manage this server.".to_string(),
        ));
    }

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
    let db_guild = match DbGuild::create(&db, id as i64).await? {
        Some(v) => v,
        None => DbGuild::get(&db, id as i64)
            .await?
            .expect("guild wasn't deleted"),
    };

    Ok(Some(GuildData {
        db: db_guild,
        http: http_guild,
    }))
}

#[derive(Params, PartialEq)]
struct Props {
    id: u64,
}

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let location = use_location(cx);
    let params = use_params::<Props>(cx);
    let guild_id: GuildIdContext = create_memo(cx, move |_| {
        params.with(|p| p.as_ref().ok().map(|p| Id::new(p.id)))
    });

    let guild: GuildContext = create_resource(
        cx,
        move || guild_id.get().map(|id| id.get()),
        move |id| async move {
            let Some(id) = id else {
                return Ok(None);
            };
            get_guild(cx, id).await
        },
    );

    provide_context(cx, guild);
    provide_context(cx, guild_id);

    let needs_invite = create_memo(cx, move |_| {
        guild.with(cx, |g| matches!(g, Ok(None))).unwrap_or(false)
    });

    let tab = create_memo(cx, move |_| {
        match location.pathname.get().split('/').last().unwrap_or("") {
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
        <InviteModal visible=needs_invite />
        <SideBar active=tab/>
    }
}

#[component]
fn InviteModal(cx: Scope, visible: Memo<bool>) -> impl IntoView {
    view! {cx,
        <dialog class=move || format!("modal {}", if visible.get() { "modal-open" } else { "" })>
            <form method="dialog" class="modal-box">
                <h3 class="font-bold text-lg">"Server Needs Setup"</h3>
                <p class="py-4">"Please add Starboard to this server to continue."</p>
                <div class="modal-action">
                    <A class="btn btn-ghost" href="..">"Go Back"</A>
                    <button class="btn btn-primary">"Invite"</button>
                </div>
            </form>
        </dialog>
    }
}
