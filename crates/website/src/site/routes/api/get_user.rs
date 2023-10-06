use leptos::*;
use twilight_model::user::CurrentUser;

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<CurrentUser, ServerFnError> {
    use crate::auth::context::AuthContext;

    let Some(acx) = AuthContext::get() else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    Ok(acx.user.clone())
}
