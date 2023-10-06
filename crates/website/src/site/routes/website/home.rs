use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <div class="hero">
            Hello 2!
        </div>
    }
}
