use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn FullScreenPopup<T, TIV, A, AIV>(
    cx: Scope,
    title: T,
    actions: A,
    children: Children,
) -> impl IntoView
where
    A: Fn() -> AIV + 'static,
    AIV: IntoView,
    T: Fn() -> TIV + 'static,
    TIV: IntoView,
{
    view! { cx,
        <dialog class="modal modal-open modal-bottom md:modal-middle pt-10 md:p-10">
            <div class="modal-box !h-full !w-full !max-w-4xl !max-h-full flex flex-col px-0">
                <div class="flex flex-row items-center px-6 pb-4">
                    <h3 class="font-bold text-lg">{title}</h3>
                    <div class="flex-1"></div>
                    <A class="btn btn-sm btn-circle btn-ghost" href="..">
                        <Icon icon=crate::icon!(FaXmarkSolid) width="1.3em" height="1.3em"/>
                    </A>
                </div>
                <div class="px-6 flex-1 overflow-y-auto">{children(cx)}</div>
                <div class="px-6 pt-4 flex flex-row gap-x-2">{actions}</div>
            </div>
        </dialog>
    }
}

#[component]
pub fn Popup<T, TIV, A, AIV>(cx: Scope, title: T, actions: A, children: Children) -> impl IntoView
where
    A: Fn() -> AIV + 'static,
    AIV: IntoView,
    T: Fn() -> TIV + 'static,
    TIV: IntoView,
{
    view! { cx,
        <dialog class="modal modal-open modal-bottom md:modal-middle pt-10 md:p-10">
            <div class="modal-box modal-scroll">
                <div class="flex flex-row items-center">
                    <h3 class="font-bold text-lg">{title}</h3>
                    <div class="flex-1"></div>
                    <A class="btn btn-sm btn-circle btn-ghost" href="..">
                        <Icon icon=crate::icon!(FaXmarkSolid) width="1.3em" height="1.3em"/>
                    </A>
                </div>
                <div class="py-4 flex-1">{children(cx)}</div>
                <div class="flex flex-row gap-x-2">{actions}</div>
            </div>
        </dialog>
    }
}
