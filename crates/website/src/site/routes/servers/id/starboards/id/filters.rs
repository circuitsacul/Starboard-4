use database::Starboard;
use leptos::*;

#[component]
pub fn Filters(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " filters"</p> }
}
