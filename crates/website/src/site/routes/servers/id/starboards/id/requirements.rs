use database::Starboard;
use leptos::*;

#[component]
pub fn Requirements(cx: Scope, sb: Starboard, hidden: Memo<bool>) -> impl IntoView {
    view! { cx, <div class:hidden=hidden>{format!("{sb:?}")} " requirements"</div> }
}
