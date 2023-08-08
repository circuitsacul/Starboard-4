use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Starboards,
    Overrides,
    Filters,
    PermRoles,
    AwardRoles,
    AutoStar,
}

#[component]
pub fn SideBar(cx: Scope, active: Memo<Tab>) -> impl IntoView {
    let guild = expect_context::<super::GuildContext>(cx);

    let title = move |cx| {
        guild.with(cx, |g| {
            g.as_ref().ok().and_then(|g| g.as_ref()).map(|g| {
                let name = &g.http.name;
                if name.len() > 23 {
                    format!("{}...", &name[0..20])
                } else {
                    name.to_owned()
                }
            })
        })
    };

    let maybe_active = move |tab: Tab| if tab == active.get() { "active" } else { "" };

    view! { cx,
        <div class="drawer lg:drawer-open">
            <input id="my-drawer-2" type="checkbox" class="drawer-toggle"/>
            <div class="drawer-content items-center">
                <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">
                    "Open drawer"
                </label>

                <Outlet/>
            </div>
            <div class="drawer-side">
                <label for="my-drawer-2" class="drawer-overlay"></label>
                <ul class="menu p-4 w-60 h-full bg-base-100 text-base-content flex flex-col space-y-2">
                    <li>
                        <A href="/servers" class="btn btn-sm btn-ghost normal-case">
                            <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                            <Transition fallback=|| ()>{move || title(cx)}</Transition>
                        </A>
                    </li>

                    <div class="divider"></div>

                    <li>
                        <A class=move || maybe_active(Tab::Overview) href="">
                            "Overview"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::Starboards) href="starboards">
                            "Starboards"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::Overrides) href="overrides">
                            "Overrides"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::Filters) href="filters">
                            "Filters"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::PermRoles) href="permroles">
                            "PermRoles"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::AwardRoles) href="awardroles">
                            "Award Roles"
                        </A>
                    </li>
                    <li>
                        <A class=move || maybe_active(Tab::AutoStar) href="autostar">
                            "Autostar Channels"
                        </A>
                    </li>
                </ul>

            </div>
        </div>
    }
}
