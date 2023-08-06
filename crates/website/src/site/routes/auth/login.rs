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
        <Suspense fallback=|| "Logging you in...">
            <ErrorBoundary fallback=|_, _| {
                "Something went wrong."
            }>
                {move || {
                    res
                        .with(
                            cx,
                            move |res| {
                                res
                                    .clone()
                                    .map(move |_| {
                                        create_effect(
                                            cx,
                                            |_| {
                                                window().location().assign("/servers").unwrap();
                                            },
                                        );
                                        "Redirecting..."
                                    })
                            },
                        )
                }}

            </ErrorBoundary>
        </Suspense>
    }
    .into_view(cx)
}
