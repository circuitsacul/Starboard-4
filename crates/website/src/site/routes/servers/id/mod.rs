pub mod overview;

use database::DbGuild;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use twilight_model::guild::Guild;

#[cfg(feature = "ssr")]
use twilight_model::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub db: DbGuild,
    pub http: Guild,
}

pub type GuildContext = Resource<u64, Option<GuildData>>;

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: u64) -> Result<Option<GuildData>, ServerFnError> {
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
    let params = use_params::<Props>(cx);
    let id = move || params.with(|p| p.as_ref().unwrap().id);
    let guild = create_local_resource(cx, id, move |id| async move {
        get_guild(cx, id).await.ok().flatten()
    });
    provide_context(cx, guild);

    let red = move || {
        guild.with(cx, |g| {
            if !g.is_some() {
                Some(Redirect(
                    cx,
                    RedirectProps::builder().path("/servers").build(),
                ))
            } else {
                None
            }
        })
    };

    view! { cx,
        <Suspense fallback=|| ()>{red}</Suspense>
        <nav>
            <ServerNavBar/>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}

#[component]
fn ServerNavBar(cx: Scope) -> impl IntoView {
    let guild = expect_context::<GuildContext>(cx);

    let title = move || guild.with(cx, |g| g.as_ref().map(|g| g.http.name.to_owned()));
    view! { cx,
        <div class="navbar">
            <div>
                <A href="/servers" class="btn btn-sm btn-ghost normal-case">
                    <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                    {title}
                </A>
            </div>
        </div>
    }
}
