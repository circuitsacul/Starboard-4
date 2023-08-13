use database::Starboard;
use leptos::*;

#[component]
pub fn Style(cx: Scope, sb: Starboard) -> impl IntoView {
    view! { cx, <p>{format!("{sb:?}")} " style"</p> }
}
