pub mod overview;
mod sidebar;
pub mod starboards;

use sidebar::{SideBar, Tab};

use database::DbGuild;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use twilight_model::{guild::Guild, id::Id, user::CurrentUserGuild};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
}

pub type GuildContext = Resource<Option<u64>, Result<Option<GuildData>, ServerFnError>>;

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: u64) -> Result<Option<GuildData>, ServerFnError> {
    use twilight_model::id::Id;

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

pub type BaseGuildCx = Memo<Option<CurrentUserGuild>>;

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let location = use_location(cx);
    let params = use_params::<Props>(cx);
    let guild_id = create_memo(cx, move |_| params.with(|p| p.as_ref().ok().map(|p| p.id)));

    let guilds = expect_context::<super::GuildsRes>(cx);
    let base_guild: BaseGuildCx = create_memo(cx, move |_| {
        let Some(guild_id) = guild_id.get() else {
            return None;
        };

        guilds
            .with(cx, |guilds| {
                guilds
                    .as_ref()
                    .ok()
                    .map(|guilds| guilds.get(&Id::new(guild_id)).cloned())
            })
            .flatten()
            .flatten()
    });
    provide_context(cx, base_guild);

    let guild: GuildContext = create_resource(
        cx,
        move || guild_id.get(),
        move |id| async move {
            let Some(id) = id else {
                return Ok(None);
            };
            get_guild(cx, id).await
        },
    );
    provide_context(cx, guild);

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
