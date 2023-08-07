pub mod overview;
mod sidebar;
pub mod starboards;

use sidebar::{SideBar, Tab};

use database::DbGuild;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use twilight_model::guild::Guild;

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

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let location = use_location(cx);
    let params = use_params::<Props>(cx);
    let guild: GuildContext = create_resource(
        cx,
        move || params.with(|p| p.as_ref().ok().map(|p| p.id)),
        move |id| async move {
            let Some(id) = id else {
                return Ok(None);
            };
            get_guild(cx, id).await
        },
    );
    provide_context(cx, guild);

    let red = move || {
        guild.with(cx, |g| {
            if matches!(g, Ok(None)) {
                Some(Redirect(
                    cx,
                    RedirectProps::builder().path("/servers").build(),
                ))
            } else {
                None
            }
        })
    };

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
        <Suspense fallback=|| ()>{red}</Suspense>
        <SideBar active=tab/>
    }
}

#[component]
fn ServerNavBar(cx: Scope) -> impl IntoView {
    let guild = expect_context::<GuildContext>(cx);

    let title = move || {
        guild.with(cx, |g| {
            g.as_ref()
                .ok()
                .and_then(|g| g.as_ref())
                .map(|g| g.http.name.to_owned())
        })
    };
    view! { cx,
        <div class="navbar">
            <div>
                <A href="/servers" class="btn btn-sm btn-ghost normal-case">
                    <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                    <Suspense fallback=|| ()>{title}</Suspense>
                </A>
            </div>
        </div>
    }
}
