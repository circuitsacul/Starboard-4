pub mod id;
pub mod server_list;

use leptos::*;
use leptos_router::*;

use super::UserRes;

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    let user = expect_context::<UserRes>(cx);

    let red = move || {
        user.with(cx, |u| {
            if u.is_err() {
                create_effect(cx, |_| {
                    window().location().assign("/api/redirect").unwrap();
                })
            }
        });
    };
    view! { cx,
        <Suspense fallback=|| ()>{red}</Suspense>
        <Outlet/>
    }
}
