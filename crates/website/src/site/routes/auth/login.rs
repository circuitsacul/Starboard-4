use leptos::*;
use leptos_router::*;

use crate::auth::oauth2::finish_auth_flow;

#[derive(Params, PartialEq, Clone)]
struct QueryParams {
    state: String,
    code: String,
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let res = create_local_resource(
        cx,
        move || use_query::<QueryParams>(cx).get().unwrap(),
        move |params| finish_auth_flow(cx, params.code.clone(), params.state.clone()),
    );

    view! { cx,
        <Suspense fallback=|| {
            view! { cx, "Logging you in..." }
        }>
            {move || {
                res.with(
                    cx,
                    |res| match res {
                        Ok(()) => {
                            if window().location().assign("/servers").is_err() {
                                "Something went wrong."
                            } else {
                                "Redirecting..."
                            }
                        }
                        Err(_) => "Something went wrong.",
                    },
                )
            }}

        </Suspense>
    }
    .into_view(cx)
}
