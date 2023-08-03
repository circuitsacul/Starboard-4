pub mod home;

use leptos::*;

use crate::site::components::NavBar;

#[component]
pub fn Website(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <NavBar/>
        </nav>
    }
}
