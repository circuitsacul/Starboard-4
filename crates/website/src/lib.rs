pub mod app;
pub mod auth;
pub mod site;
pub mod utils;

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use auth::context::AuthContext;
#[cfg(feature = "ssr")]
use common::async_dash::AsyncDashMap;
#[cfg(feature = "ssr")]
use jwt_simple::prelude::HS256Key;
#[cfg(feature = "ssr")]
use twilight_http::Client;
#[cfg(feature = "ssr")]
use twilight_model::id::marker::UserMarker;
#[cfg(feature = "ssr")]
use twilight_model::id::Id;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "ssr")]
pub type AuthStates = Arc<AsyncDashMap<Id<UserMarker>, Arc<AuthContext>>>;

#[cfg(feature = "ssr")]
pub fn expect_auth_states(cx: leptos::Scope) -> AuthStates {
    leptos::expect_context(cx)
}

#[cfg(feature = "ssr")]
pub fn expect_config(cx: leptos::Scope) -> Arc<common::config::Config> {
    leptos::expect_context(cx)
}

#[cfg(feature = "ssr")]
pub fn jwt_key(cx: leptos::Scope) -> Arc<HS256Key> {
    leptos::expect_context(cx)
}

#[cfg(feature = "ssr")]
pub fn oauth_client(cx: leptos::Scope) -> Arc<oauth2::basic::BasicClient> {
    leptos::expect_context(cx)
}

#[cfg(feature = "ssr")]
pub fn db(cx: leptos::Scope) -> std::sync::Arc<database::DbClient> {
    leptos::use_context(cx).expect("database")
}

#[cfg(feature = "ssr")]
pub fn bot_http(cx: leptos::Scope) -> Arc<Client> {
    leptos::use_context(cx).expect("http client")
}

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
