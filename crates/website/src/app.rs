use leptos::*;
use leptos_meta::*;

use crate::site::routes::Index;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Title formatter=|text| format!("{text} - Starboard")/>
        <Script src="https://cdn.jsdelivr.net/npm/emoji-mart@latest/dist/browser.js"/>

        <Index/>
    }
}
