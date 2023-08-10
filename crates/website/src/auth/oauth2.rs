use leptos::*;

#[cfg(feature = "ssr")]
use actix_web::web::Query;
#[cfg(feature = "ssr")]
use actix_web::{
    cookie::{Cookie, SameSite},
    http::header::SET_COOKIE,
};
#[cfg(feature = "ssr")]
use jwt_simple::prelude::MACLike;
#[cfg(feature = "ssr")]
use leptos_actix::redirect;
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use oauth2::{
    http::HeaderValue, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
use crate::{jwt_key, oauth_client};

#[cfg(feature = "ssr")]
use super::context::AuthContext;
#[cfg(feature = "ssr")]
use super::jwt::AuthClaims;

#[cfg(feature = "ssr")]
fn secure_cookie(name: &str, value: &str) -> HeaderValue {
    let cookie = Cookie::build(name, value)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();
    HeaderValue::from_str(&cookie.to_string()).unwrap()
}

#[server(BeginAuthFlow, "/api", "Url", "redirect")]
pub async fn begin_auth_flow(cx: leptos::Scope) -> Result<(), ServerFnError> {
    #[derive(Deserialize)]
    struct QueryParams {
        guild_id: Option<u64>,
    }

    let client = oauth_client(cx);

    let response = expect_context::<ResponseOptions>(cx);
    let req = expect_context::<actix_web::HttpRequest>(cx);
    let query = Query::<QueryParams>::from_query(req.query_string())?;

    let mut builder = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()));

    if let Some(id) = query.guild_id {
        builder = builder
            .add_scope(Scope::new("bot".to_string()))
            .add_extra_param("guild_id", id.to_string());
    }

    let (url, csrf) = builder.url();

    response.insert_header(
        SET_COOKIE,
        secure_cookie("ExpectedOAuth2State", csrf.secret()),
    );

    redirect(cx, url.as_ref());

    Ok(())
}

#[server(FinishAuthFlow, "/api", "Url", "login")]
pub async fn finish_auth_flow(cx: leptos::Scope) -> Result<(), ServerFnError> {
    #[derive(Deserialize)]
    struct QueryParams {
        state: String,
        code: String,
        guild_id: Option<u64>,
    }

    let req = expect_context::<actix_web::HttpRequest>(cx);
    let response = expect_context::<ResponseOptions>(cx);
    let client = oauth_client(cx);
    let jwt_key = jwt_key(cx);

    let query = Query::<QueryParams>::from_query(req.query_string())?;

    if !req
        .cookie("ExpectedOAuth2State")
        .map(|c| c.value() == query.state)
        .unwrap_or(false)
    {
        return Err(ServerFnError::ServerError("Invalid state".to_string()));
    }

    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await?
        .access_token()
        .to_owned();

    let http = twilight_http::Client::new(format!("Bearer {}", token.secret()));
    let user = http.current_user().await?.model().await?;

    let claims = AuthClaims::new(user.id).build();
    let jwt = jwt_key.authenticate(claims.clone()).unwrap();

    let acx = AuthContext::new(http, claims, user);
    acx.provide(cx);

    response.insert_header(SET_COOKIE, secure_cookie("SessionKey", &jwt));

    if let Some(id) = query.guild_id {
        redirect(cx, &format!("/servers/{id}"));
    } else {
        redirect(cx, "/servers");
    }

    Ok(())
}
