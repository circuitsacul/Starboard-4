use leptos::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar z-30 backdrop-blur-md fixed">
            <div class="flex-1 space-x-2">
                <a href="/" class="btn btn-ghost normal-case text-xl">
                    Starboard
                </a>

                <a class="btn btn-ghost btn-sm" href=common::constants::INVITE_URL target="_blank">
                    Invite
                </a>
                <a class="btn btn-ghost btn-sm" href=common::constants::SUPPORT_URL target="_blank">
                    Support
                </a>
                <a class="btn btn-ghost btn-sm" href=common::constants::DOCS_URL target="_blank">
                    Docs
                </a>
                <a class="btn btn-ghost btn-sm" href=common::constants::PATREON_URL target="_blank">
                    Premium
                </a>
                <a class="btn btn-ghost btn-sm" href=common::constants::SOURCE_URL target="_blank">
                    GitHub
                </a>
            </div>

            <div>
                <a class="btn btn-primary" href="/servers">
                    Manage
                </a>
            </div>
        </div>
        <div class="pt-16"></div>
    }
}
