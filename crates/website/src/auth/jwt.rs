use jwt_simple::prelude::*;
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthClaims {
    /// The ID of the authenticated user.
    pub user_id: Id<UserMarker>,
    /// The oauth2 token of the authenticated user.
    ///
    /// This is stored, unencrypted, in the cookies
    /// of the clients browser. This is safe because
    /// the oauth2 tokens we generate only ever have
    /// the "identify" and "guilds" scope.
    pub user_token: AccessToken,
}

impl AuthClaims {
    pub fn new(user_id: Id<UserMarker>, user_token: AccessToken) -> Self {
        Self {
            user_id,
            user_token,
        }
    }

    pub fn sign(self, key: &HS256Key) -> String {
        let claims = Claims::with_custom_claims(self, Duration::from_hours(2));
        key.authenticate(claims).unwrap()
    }

    pub fn verify(token: &str, key: &HS256Key) -> Option<JWTClaims<Self>> {
        key.verify_token::<Self>(token, None).ok()
    }
}

/// Generate a new secret key for signing
/// JWT claims.
///
/// There is a new secret every restart,
/// meaning that after website restarts,
/// users will need to sign in again.
pub fn new_secret() -> HS256Key {
    HS256Key::generate()
}
