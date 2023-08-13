use database::Starboard;
use leptos::*;

#[component]
pub fn Behavior(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " behavior"</p> }
}
