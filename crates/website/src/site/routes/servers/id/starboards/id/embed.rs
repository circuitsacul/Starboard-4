use database::Starboard;
use leptos::*;

#[component]
pub fn Embed(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " embed"</p> }
}
