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

    let cb: NodeRef<html::Input> = create_node_ref(cx);
    let close = move || {
        cb.get_untracked().map(|cb| cb.set_checked(false)).unwrap();
    };

    let maybe_active = move |tab: Tab| if tab == active.get() { "active" } else { "" };

    view! { cx,
        <div class="drawer lg:drawer-open">
            <input _ref=cb id="dashboard-drawer" type="checkbox" class="drawer-toggle"/>
            <div class="drawer-content items-center">
                <Outlet/>
            </div>
            <div class="drawer-side lg:top-16 lg:h-min z-40 lg:z-auto">
                <label for="dashboard-drawer" class="drawer-overlay"></label>
                <div class="w-60 p-4 bg-base-100 text-base-content h-full lg:h-min">
                    <A
                        href="/servers"
                        class="btn btn-sm btn-ghost btn-block normal-case truncate !flex-nowrap"
                    >
                        <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                        <span class="truncate">
                            <Suspense fallback=|| ()>
                                {move || {
                                    guild.read(cx).map(|g| g.map(|g| g.map(|g| g.http.name.clone())))
                                }}
                            </Suspense>
                        </span>
                    </A>
                    <div class="divider"></div>
                    <ul class="menu p-0 flex flex-col space-y-2" on:click=move |_| close()>
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
