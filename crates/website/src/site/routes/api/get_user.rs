use leptos::*;
use twilight_model::user::CurrentUser;

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<CurrentUser, ServerFnError> {
    use crate::auth::context::AuthContext;

    let Some(acx) = AuthContext::get(cx) else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    Ok(acx.user.clone())
}
