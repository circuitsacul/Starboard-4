use leptos::*;

#[cfg(feature = "ssr")]
use actix_web::{
    cookie::{Cookie, SameSite},
    http::header::SET_COOKIE,
};
#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;
#[cfg(feature = "ssr")]
use oauth2::{
    http::HeaderValue, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};

#[cfg(feature = "ssr")]
use crate::{jwt_key, oauth_client};

#[cfg(feature = "ssr")]
use super::jwt::AuthClaims;

#[cfg(feature = "ssr")]
fn secure_cookie(name: &str, value: &str, samesite: bool) -> HeaderValue {
    let cookie = Cookie::build(name, value)
        .http_only(true)
        .secure(true)
        .same_site(if samesite {
            SameSite::Strict
        } else {
            SameSite::Lax
        })
        .path("/")
        .finish();
    HeaderValue::from_str(&cookie.to_string()).unwrap()
}

#[server(BeginAuthFlow, "/api")]
pub async fn begin_auth_flow(cx: leptos::Scope) -> Result<String, ServerFnError> {
    let client = oauth_client(cx);

    let response = expect_context::<ResponseOptions>(cx);

    let (url, csrf) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        .url();

    response.insert_header(
        SET_COOKIE,
        secure_cookie("ExpectedOAuth2State", csrf.secret(), false),
    );

    Ok(url.to_string())
}

#[server(FinishAuthFlow, "/api")]
pub async fn finish_auth_flow(
    cx: leptos::Scope,
    code: String,
    state: String,
) -> Result<(), ServerFnError> {
    let req = expect_context::<actix_web::HttpRequest>(cx);
    let response = expect_context::<ResponseOptions>(cx);
    let client = oauth_client(cx);
    let jwt_key = jwt_key(cx);

    if req
        .cookie("ExpectedOAuth2State")
        .map(|c| c.value().to_string())
        != Some(state)
    {
        return Err(ServerFnError::ServerError("Invalid state".to_string()));
    }

    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?
        .access_token()
        .to_owned();

    let http = twilight_http::Client::new(format!("Bearer {}", token.secret()));
    let user = http.current_user().await?.model().await?;

    let jwt = AuthClaims::new(user.id, token).sign(&jwt_key);

    response.insert_header(SET_COOKIE, secure_cookie("SessionKey", &jwt, true));

    Ok(())
}
