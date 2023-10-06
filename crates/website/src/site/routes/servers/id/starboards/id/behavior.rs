use database::Starboard;
use leptos::*;

use crate::site::components::form::ValidationErrors;

#[component]
pub fn Behavior<E: SignalWith<Value = ValidationErrors> + Copy + 'static>(
    errs: E,
    sb: Starboard,
    hidden: Memo<bool>,
) -> impl IntoView {
    view! { <div class:hidden=hidden>{format!("{sb:?}")} " behavior"</div> }
}
