pub mod website;
pub mod dashboard;

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
            <main>
                <Route path="" view=website::home::Home/>

                <Route path="/*any" view=errors::not_found::NotFound/>
            </main>
        </Route>
    }
}

#[component(transparent)]
fn DashboardRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path="/dashboard" view=dashboard::Dashboard>
            <main>
                <Route path="" view=dashboard::server_list::ServerList/>
                // <Route path="/server/:id" view=dashboard::server::Server>
                //     <Route path="" view=dashboard::server::overview::Overview/>
                //     <Route path="/starboards" view=dashboard::starboards::Starboards>
                //         <Route path=":id" view=dashboard::starboards::Requirements/>
                //         <Route path=":id/behavior" view=dashboard::starboards::Behavior/>

                //     </Route>
                // </Route>

                <Route path="/*any" view=errors::not_found::NotFound/>
            </main>
        </Route>
    }
}
