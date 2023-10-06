mod api;
pub mod servers;
pub mod website;

use leptos::*;
use leptos_router::*;
use twilight_model::user::CurrentUser;

use crate::site::components::ToastProvider;

use super::errors;

pub type UserResource = Resource<(), Result<CurrentUser, ServerFnError>>;

#[component]
pub fn Index() -> impl IntoView {
    let user: UserResource = create_resource(|| (), move |_| self::api::get_user());
    provide_context(user);

    view! {
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
fn DashboardRoutes() -> impl IntoView {
    view! {
        <Route path="/servers" view=servers::Servers>
            <Route path="" view=servers::server_list::ServerList/>
            <Route path=":guild_id" view=servers::id::Server>
                <Route path="" view=servers::id::overview::Overview/>
                <Route path="/starboards" view=servers::id::starboards::Starboards>
                    <Route path="" view=move || ()/>
                    <Route path="add" view=servers::id::starboards::add::Add/>
                    <Route path=":starboard_id" view=servers::id::starboards::id::Starboard/>
                </Route>

                <Route path="/*any" view=errors::not_found::NotFound/>
            </Route>

            <Route path="/*any" view=errors::not_found::NotFound/>
        </Route>
    }
}
