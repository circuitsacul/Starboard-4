use database::Starboard;
use leptos::*;

#[component]
pub fn Regex(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " regex"</p> }
}
