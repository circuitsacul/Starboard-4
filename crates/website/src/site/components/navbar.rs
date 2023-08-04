use leptos::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar">
            <div>
                <a href="/" class="btn btn-ghost normal-case text-xl">
                    Starboard
                </a>
            </div>

            <div class="flex-1 justify-center">
                <a class="btn btn-ghost btn-sm" href="https://docs.starboard.best" target="_blank">
                    Docs
                </a>
                <a
                    class="btn btn-ghost btn-sm"
                    href="https://patreon.com/circuitsacul"
                    target="_blank"
                >
                    Premium
                </a>
            </div>

            <div>
                <a class="btn btn-primary" href="/servers">
                    Manage
                </a>
            </div>
        </div>
    }
}
