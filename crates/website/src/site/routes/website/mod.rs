pub mod home;

use leptos::*;
use leptos_router::Outlet;

use crate::site::components::NavBar;

#[component]
pub fn Website(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <NavBar/>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}
