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
    view! {cx,
        <dialog class="modal modal-open modal-bottom md:modal-middle pt-10 md:p-10">
            <div class="modal-box !h-full !w-full !max-w-4xl !max-h-full flex flex-col overflow-hidden">
                <div class="flex flex-row items-center">
                    <h3 class="font-bold text-lg">{title}</h3>
                    <div class="flex-1"/>
                    <A class="btn btn-sm btn-circle btn-ghost" href="..">
                        <Icon icon=crate::icon!(FaXmarkSolid) width="1.3em" height="1.3em"/>
                    </A>
                </div>
                <div class="my-4 overflow-scroll flex-1">
                    {children(cx)}
                </div>
                <div class="flex flex-row space-x-2">
                    {actions}
                </div>
            </div>
        </dialog>
    }
}
