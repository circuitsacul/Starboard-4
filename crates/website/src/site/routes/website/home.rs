use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Home"/>
        <div class="hero">
            Hello 2!
        </div>
    }
}
