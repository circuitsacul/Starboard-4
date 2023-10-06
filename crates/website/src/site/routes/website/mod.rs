pub mod home;

use leptos::*;
use leptos_router::Outlet;

use crate::site::components::NavBar;

#[component]
pub fn Website() -> impl IntoView {
    view! {
        <nav>
            <NavBar/>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}
