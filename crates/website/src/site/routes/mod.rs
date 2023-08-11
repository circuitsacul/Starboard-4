pub mod servers;
pub mod website;

use leptos::*;
use leptos_router::*;
use twilight_model::user::CurrentUser;

use crate::site::components::ToastProvider;

use super::errors;

pub type UserResource = Resource<(), Result<CurrentUser, ServerFnError>>;

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<CurrentUser, ServerFnError> {
    use crate::auth::context::AuthContext;
    let Some(acx) = AuthContext::get(cx) else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    Ok(acx.user.clone())
}

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    let user: UserResource = create_resource(cx, || (), move |_| get_user(cx));
    provide_context(cx, user);

    view! { cx,
        <ToastProvider>
            <Router>
                <Routes>
                    <Route path="" view=website::Website>
                        <Route path="" view=website::home::Home/>

                        <DashboardRoutes/>

                        <Route path="/*any" view=errors::not_found::NotFound/>
                    </Route>
                </Routes>
            </Router>
        </ToastProvider>
    }
}

#[component(transparent)]
fn DashboardRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/servers" view=servers::Servers>
            <Route path="" view=servers::server_list::ServerList/>
            <Route path=":guild_id" view=servers::id::Server>
                <Route path="" view=servers::id::overview::Overview/>
                <Route path="/starboards" view=servers::id::starboards::Starboards>
                    <Route path="" view=move |_| ()/>
                    <Route path=":starboard_id" view=servers::id::starboards::id::Starboard/>
                </Route>

                <Route path="/*any" view=errors::not_found::NotFound/>
            </Route>

            <Route path="/*any" view=errors::not_found::NotFound/>
        </Route>
    }
}
