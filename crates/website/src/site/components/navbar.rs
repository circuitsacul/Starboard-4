use leptos::{html::ToHtmlElement, *};
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    let loc = use_location(cx);
    let show_hamburger = create_memo(cx, move |_| {
        loc.pathname.get().trim_start_matches("/servers").len() > 1
    });

    let links = [
        ("Invite", common::constants::INVITE_URL),
        ("Support", common::constants::SUPPORT_URL),
        ("Docs", common::constants::DOCS_URL),
        ("Premium", common::constants::PATREON_URL),
        ("GitHub", common::constants::SOURCE_URL),
    ];

    let blur_active = move |cx| {
        document()
            .active_element()
            .map(|elm| elm.to_leptos_element(cx).blur())
    };

    view! { cx,
        <div class="navbar backdrop-blur bg-base-100/70 fixed z-[1]">
            {move || {
                if show_hamburger.get() {
                    Some(
                        view! { cx,
                            <label
                                for="dashboard-drawer"
                                class="btn btn-ghost btn-square lg:hidden mr-2"
                            >
                                <Icon icon=crate::icon!(FaBarsSolid)/>
                            </label>
                        },
                    )
                } else {
                    None
                }
            }}
            <div class="dropdown dropdown-hover lg:hidden">
                <button class="btn btn-ghost normal-case text-xl">
                    Starboard
                </button>

                <ul
                    class="menu dropdown-content rounded-box p-2 drop-shadow-lg bg-base-100"
                    on:click=move |_| {
                        let _ = blur_active(cx);
                    }
                >

                    <li>
                        <A href="">"Home"</A>
                    </li>
                    {move || {
                        links
                            .map(|link| {
                                view! { cx,
                                    <li>
                                        <a href=link.1 target="_blank">
                                            {link.0}
                                            <Icon icon=crate::icon!(FaArrowUpRightFromSquareSolid)/>
                                        </a>
                                    </li>
                                }
                            })
                    }}

                </ul>
            </div> <div class="hidden lg:flex flex-1 space-x-2">
                <A href="" class="btn btn-ghost normal-case text-xl">
                    "Starboard"
                </A>

                {move || {
                    links
                        .map(|link| {
                            view! { cx,
                                <a class="btn btn-ghost btn-sm" href=link.1 target="_blank">
                                    {link.0}
                                    <Icon icon=crate::icon!(FaArrowUpRightFromSquareSolid)/>
                                </a>
                            }
                        })
                }}

            </div> <div class="flex-1"></div> <div>
                <a class="btn btn-ghost" href="/servers">
                    <Icon icon=crate::icon!(FaGearSolid)/>
                    <span class="hidden sm:inline">"Servers"</span>
                </a>
            </div>
        </div>
        <div class="pt-16"></div>
    }
}
