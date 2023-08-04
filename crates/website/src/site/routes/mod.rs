pub mod servers;
pub mod website;

use leptos::*;
use leptos_router::*;

use super::errors;

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <WebsiteRoutes/>
                <DashboardRoutes/>
            </Routes>
        </Router>
    }
}

#[component(transparent)]
fn WebsiteRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="" view=website::Website>
            <Route path="" view=website::home::Home/>

            <Route path="/*any" view=errors::not_found::NotFound/>
        </Route>
    }
}

#[component(transparent)]
fn DashboardRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/servers" view=servers::Servers>
            <Route path="" view=servers::server_list::ServerList/>
            <Route path=":id" view=servers::id::Server/>
            // <Route path="" view=dashboard::server::overview::Overview/>
            // <Route path="/starboards" view=dashboard::starboards::Starboards>
            // <Route path=":id" view=dashboard::starboards::Requirements/>
            // <Route path=":id/behavior" view=dashboard::starboards::Behavior/>

            // </Route>

            <Route path="/*any" view=errors::not_found::NotFound/>
        </Route>
    }
}
