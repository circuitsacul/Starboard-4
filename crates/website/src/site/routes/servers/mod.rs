pub mod id;
pub mod server_list;

use leptos::*;
use leptos_router::*;

use super::UserRes;

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    let user = expect_context::<UserRes>(cx);

    view! { cx,
        <Suspense fallback=|| ()>
            {move || {
                user.with(
                    cx,
                    |user| {
                        if user.is_err() {
                            Some(view! { cx, <Redirect path="/auth/redirect"/> })
                        } else {
                            None
                        }
                    },
                )
            }}

        </Suspense>
        <Outlet/>
    }
}
