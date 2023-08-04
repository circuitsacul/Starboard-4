pub mod app;
pub mod site;
pub mod utils;

#[cfg(feature = "ssr")]
pub fn db(cx: leptos::Scope) -> std::sync::Arc<database::DbClient> {
    leptos::use_context(cx).expect("database")
}

#[cfg(feature = "ssr")]
pub fn http(cx: leptos::Scope) -> std::sync::Arc<twilight_http::client::Client> {
    leptos::use_context(cx).expect("http client")
}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
