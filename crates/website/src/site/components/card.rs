use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn Card(cx: Scope, title: String, href: String) -> impl IntoView {
    view! {cx,
        <A
            href=href
            class="btn btn-lg btn-block btn-ghost my-2 normal-case !flex-nowrap"
        >
            <div class="flex-1 text-left truncate">{title}</div>
            <Icon icon=crate::icon!(FaPlusSolid)/>
        </A>
    }
}

#[component]
pub fn CardSkeleton(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="btn btn-lg btn-block btn-ghost my-2 btn-disabled !bg-transparent animate-pulse">
            <div class="flex-1"><div class="h-5 bg-gray-700/30 rounded-full w-full max-w-[250px]"/></div>
            <Icon icon=crate::icon!(FaPlusSolid)/>
        </div>
    }
}

#[component]
pub fn CardList(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="flex justify-center">
            <div class="max-w-4xl w-full p-1">
                {children(cx)}
            </div>
        </div>
    }
}
