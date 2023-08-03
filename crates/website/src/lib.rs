pub mod app;

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
