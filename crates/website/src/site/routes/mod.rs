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
                <Route path="" view=website::Website>
                    <main>
                        <Route path="" view=website::home::Home/>

                        <Route path="/*any" view=errors::not_found::NotFound/>
                    </main>
                </Route>

                <Route path="/dashboard" view=dashboard::Dashboard>
                    <main>
                        <Route path="" view=dashboard::server_list::ServerList/>
                        // <Route path="/server/:id" view=dashboard::server::Server>
                        //     <Route path="" view=dashboard::server::overview::Overview/>
                        //     <Route path="/starboards" view=dashboard::starboards::Starboards>
                        //         <Route path=":id" view=dashboard::starboards::Requirements/>
                        //         <Route path=":id/behavior" view=dashboard::starboards::Behavior/>
                        //         // ...other setting categories
                        //         // requirements, behavior, embed, style, regex, filters
                        //     </Route>
                        // </Route>

                        <Route path="/*any" view=errors::not_found::NotFound/>
                    </main>
                </Route>
            </Routes>
        </Router>
    }
}
