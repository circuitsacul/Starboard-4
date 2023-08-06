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
    let res = create_blocking_resource(
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
                            create_effect(
                                cx,
                                move |_| {
                                    window().location().assign("/servers").unwrap();
                                },
                            );
                            "Redirecting..."
                        }
                        Err(_) => "Something went wrong.",
                    },
                )
            }}

        </Suspense>
    }
    .into_view(cx)
}
