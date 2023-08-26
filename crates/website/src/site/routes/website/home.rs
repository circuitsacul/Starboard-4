use leptos::*;

use crate::site::components::EmojiButton;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="hero">
            Hello 2!
        </div>
    }
}
