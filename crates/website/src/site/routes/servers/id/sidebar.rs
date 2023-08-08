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
            g.as_ref()
                .ok()
                .and_then(|g| g.as_ref())
                .map(|g| g.http.name.to_owned())
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
            <div class="drawer-side lg:top-16 lg:h-min z-40 lg:z-auto">
                <label for="my-drawer-2" class="drawer-overlay"></label>
                <div class="w-60 p-4 bg-base-100 text-base-content h-full lg:h-min">
                    <A
                        href="/servers"
                        class="btn btn-sm btn-ghost btn-block normal-case truncate !flex-nowrap"
                    >
                        <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                        <span class="truncate">
                            <Transition fallback=|| ()>{move || title(cx)}</Transition>
                        </span>
                    </A>
                    <div class="divider"></div>
                    <ul class="menu p-0 flex flex-col space-y-2">
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
        </div>
    }
}
