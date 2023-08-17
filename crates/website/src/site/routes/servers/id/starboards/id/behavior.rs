use database::Starboard;
use leptos::*;

use crate::site::components::form::ValidationErrors;

#[component]
pub fn Behavior<E: SignalWith<ValidationErrors> + Copy + 'static>(
    cx: Scope,
    errs: E,
    sb: Starboard,
    hidden: Memo<bool>,
) -> impl IntoView {
    view! {cx,
        <div class:hidden=hidden>
            {format!("{sb:?}")} " behavior"
        </div>
    }
}
