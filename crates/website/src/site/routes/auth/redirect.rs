use leptos::*;

use crate::auth::oauth2::begin_auth_flow;

#[component]
pub fn Redirect(cx: Scope) -> impl IntoView {
    let res = create_local_resource(cx, || (), move |_| begin_auth_flow(cx));

    view! { cx,
        <Suspense fallback=|| {
            view! { cx, "Redirecting..." }
        }>
            {move || {
                res
                    .with(
                        cx,
                        |url| match url {
                            Err(_) => "Something went wrong.",
                            Ok(url) => {
                                if window().location().assign(url).is_err() {
                                    "Something went wrong."
                                } else {
                                    "Redirecting..."
                                }
                            }
                        },
                    )
            }}

        </Suspense>
    }
}
