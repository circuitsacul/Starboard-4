#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::{sync::Arc, time::Duration};

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
    use twilight_http::client::Client as HttpClient;
    use website::{app::*, auth::jwt};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    let config = Arc::new(common::config::Config::from_env());
    let db = Arc::new(database::DbClient::new(&config.db_url).await.unwrap());
    let mut http = HttpClient::builder()
        .token(config.token.clone())
        .timeout(Duration::from_secs(30));
    if let Some(proxy) = &config.proxy {
        http = http.proxy(proxy.to_owned(), true);
    }
    let http = Arc::new(http.build());
    let jwt_key = Arc::new(jwt::new_secret());

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        let db = db.clone();
        let db2 = db.clone();
        let config = config.clone();
        let config2 = config.clone();
        let http = http.clone();
        let http2 = http.clone();

        let oauth_client = BasicClient::new(
            ClientId::new(config.bot_id.to_string()),
            Some(ClientSecret::new(
                config
                    .client_secret
                    .clone()
                    .expect("CLIENT_SECRET required for website"),
            )),
            AuthUrl::new("https://discord.com/oauth2/authorize".to_string()).unwrap(),
            Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new(
                config
                    .redirect_url
                    .clone()
                    .expect("REDIRECT_URL required for website"),
            )
            .unwrap(),
        );

        let oauth_client = Arc::new(oauth_client);
        let oauth_client2 = oauth_client.clone();

        let jwt_key = jwt_key.clone();
        let jwt_key2 = jwt_key.clone();

        App::new()
            .route(
                "/api/{tail:.*}",
                leptos_actix::handle_server_fns_with_context(move |cx| {
                    provide_context(cx, db.clone());
                    provide_context(cx, config.clone());
                    provide_context(cx, http.clone());
                    provide_context(cx, oauth_client.clone());
                    provide_context(cx, jwt_key.clone());
                }),
            )
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), move |cx| {
                provide_context(cx, db2.clone());
                provide_context(cx, config2.clone());
                provide_context(cx, http2.clone());
                provide_context(cx, oauth_client2.clone());
                provide_context(cx, jwt_key2.clone());

                view! { cx, <App/> }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use website::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! { cx, <App/> }
    });
}
