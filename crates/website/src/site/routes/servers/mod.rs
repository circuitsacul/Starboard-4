pub mod id;
pub mod server_list;

use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Servers(cx: Scope) -> impl IntoView {
    view! { cx, <Outlet/> }
}
