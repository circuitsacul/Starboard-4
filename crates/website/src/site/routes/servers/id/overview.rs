use leptos::*;

use super::FlatGuildSuspense;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    view! { cx, <FlatGuildSuspense fallback=|| "loading..." child=|g| format!("{g:?}")/> }
}
