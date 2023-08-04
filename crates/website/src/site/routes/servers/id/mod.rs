use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[server(GetGuild, "/api")]
pub async fn get_guild(cx: Scope, id: i64) -> Result<Option<database::DbGuild>, ServerFnError> {
    let db = crate::db(cx);

    Ok(database::DbGuild::get(&db, id).await?)
}

#[derive(Params, PartialEq)]
struct Props {
    id: i64,
}

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let params = use_params::<Props>(cx);
    let id = move || params.with(|p| p.as_ref().unwrap().id);
    let guild = create_resource(cx, id, move |id| get_guild(cx, id));

    view! { cx,
        <nav>
            <ServerNavBar/>
        </nav>
        <main>
            <Suspense fallback=move || {
                view! { cx, <p>"Loading..."</p> }
            }>
                {move || guild.with(cx, |g| format!("{g:?}")) }
            </Suspense>
        </main>
    }
}

#[component]
fn ServerNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar">
            <div>
                <A href=".." class="btn btn-sm btn-ghost">
                    <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                    "Back"
                </A>
            </div>
        </div>
    }
}
