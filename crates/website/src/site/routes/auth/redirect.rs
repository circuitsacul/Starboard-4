use leptos::*;

use crate::auth::oauth2::begin_auth_flow;

#[component]
pub fn Redirect(cx: Scope) -> impl IntoView {
    let res = create_blocking_resource(cx, || (), move |_| begin_auth_flow(cx));

    view! { cx,
        <Suspense fallback=|| {
            view! { cx, "Redirecting..." }
        }>
            {move || {
                res.with(
                    cx,
                    |url| match url {
                        Err(_) => "Something went wrong.",
                        Ok(url) => {
                            let url = url.to_owned();
                            create_effect(
                                cx,
                                move |_| {
                                    window().location().assign(&url).unwrap();
                                },
                            );
                            "Redirecting..."
                        }
                    },
                )
            }}

        </Suspense>
    }
}
