use std::sync::Arc;

use actix_web::HttpRequest;
use jwt_simple::prelude::JWTClaims;
use leptos::*;
use twilight_http::Client;

use crate::{expect_auth_states, jwt_key};

use super::jwt::AuthClaims;

#[derive(Debug)]
pub struct AuthContext {
    pub http: Client,
    pub claims: JWTClaims<AuthClaims>,
}

impl AuthContext {
    pub fn provide(self, cx: leptos::Scope) -> Arc<Self> {
        let states = expect_auth_states(cx);
        let acx = Arc::new(self);
        states.insert(acx.claims.custom.user_id, acx.clone());
        acx
    }

    pub fn get(cx: leptos::Scope) -> Option<Arc<Self>> {
        let req = use_context::<HttpRequest>(cx)?;
        let key = jwt_key(cx);
        let Some(session) = req.cookie("SessionKey") else {
            return None;
        };
        let claims = AuthClaims::verify(session.value(), &key)?;

        let states = expect_auth_states(cx);
        states.with(&claims.custom.user_id, |_, state| {
            let Some(state) = state else {
                return None;
            };

            if claims.nonce != state.claims.nonce {
                return None;
            }

            Some(state.value().clone())
        })
    }

    pub fn build_http(access_token: &str) -> Client {
        let client = Client::builder().token(format!("Bearer {access_token}"));
        client.build()
    }
}
