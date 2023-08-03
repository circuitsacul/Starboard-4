use leptos::*;
use leptos_meta::*;

use crate::site::routes::Index;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Title text="Welcome to Leptos"/>

        <Index/>
    }
}
