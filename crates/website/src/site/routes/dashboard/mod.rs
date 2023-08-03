pub mod server_list;

use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <p>"dashboard navbar"</p>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}
