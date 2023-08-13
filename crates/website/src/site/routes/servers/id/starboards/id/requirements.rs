use database::Starboard;
use leptos::*;

#[component]
pub fn Requirements(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " requirements"</p> }
}
