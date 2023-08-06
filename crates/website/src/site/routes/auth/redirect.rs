use leptos::*;

use crate::auth::oauth2::begin_auth_flow;

#[component]
pub fn AuthRedirect(cx: Scope) -> impl IntoView {
    let res = create_blocking_resource(cx, || (), move |_| begin_auth_flow(cx));

    view! { cx,
        <Suspense fallback=|| "Redirecting...">
            <ErrorBoundary fallback=|_, _| {
                "Something went wrong."
            }>
                {move || {
                    res.with(
                        cx,
                        move |url| {
                            url
                                .clone()
                                .map(|url| {
                                    create_effect(
                                        cx,
                                        move |_| {
                                            window().location().assign(&url).unwrap();
                                        },
                                    );
                                    view! { cx, "Redirecting..." }
                                })
                        },
                    )
                }}

            </ErrorBoundary>
        </Suspense>
    }
}
