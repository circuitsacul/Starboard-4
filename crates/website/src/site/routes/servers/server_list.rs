use leptos::*;
use leptos_router::*;

use crate::site::components::NavBar;

#[component]
pub fn ServerList(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <NavBar/>
        </nav>
        <main>
            <A href="945149610484195398" class="link">
                "Go to server"
            </A>
        </main>
    }
}
