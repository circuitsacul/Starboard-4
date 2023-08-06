use actix_web::HttpRequest;
use jwt_simple::prelude::JWTClaims;
use leptos::*;
use twilight_http::Client;

use crate::{expect_config, jwt_key};

use super::jwt::AuthClaims;

#[derive(Debug)]
pub struct AuthContext {
    pub http: Client,
    pub claims: JWTClaims<AuthClaims>,
}

impl AuthContext {
    pub fn build_from_cx(cx: leptos::Scope) -> Option<Self> {
        let req = use_context::<HttpRequest>(cx)?;
        let key = jwt_key(cx);

        let Some(jwt_cookie) = req.cookie("SessionKey") else {
            return None;
        };

        let jwt = jwt_cookie.value();
        let claims = AuthClaims::verify(jwt, &key)?;

        Some(Self {
            http: Self::build_http(cx, claims.custom.user_token.secret().to_owned()),
            claims,
        })
    }

    pub fn build_http(cx: leptos::Scope, access_token: String) -> Client {
        let config = expect_config(cx);

        let mut client = Client::builder().token(format!("Bearer {access_token}"));
        if let Some(proxy) = config.proxy.clone() {
            client = client.proxy(proxy, true);
        }
        client.build()
    }
}
