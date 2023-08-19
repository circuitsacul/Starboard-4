use std::time::Duration;

use leptos::*;
use leptos_icons::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ToastType {
    Error,
    Warning,
    Info,
    Success,
}

#[derive(Clone)]
pub struct Toast {
    pub typ: ToastType,
    pub msg: String,
    pub id: u64,
}

impl Toast {
    pub fn error(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Error,
            msg: msg.to_string(),
            id: rand::random(),
        }
    }

    pub fn warning(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Warning,
            msg: msg.to_string(),
            id: rand::random(),
        }
    }

    pub fn info(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Info,
            msg: msg.to_string(),
            id: rand::random(),
        }
    }

    pub fn success(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Success,
            msg: msg.to_string(),
            id: rand::random(),
        }
    }
}

pub type ToastCx = RwSignal<Vec<Toast>>;

pub fn toast(cx: Scope, toast: Toast) {
    let id = toast.id;
    let toasts = expect_context::<ToastCx>(cx);

    create_effect(cx, move |_| {
        let toast = toast.clone();
        request_animation_frame(move || {
            toasts.update(|toasts| {
                toasts.push(toast);
            });
            set_timeout(
                move || {
                    toasts.try_update(|toasts| {
                        toasts.retain(|t| t.id != id);
                    });
                },
                Duration::from_secs(5),
            )
        });
    });
}

#[component]
pub fn ToastProvider(cx: Scope, children: Children) -> impl IntoView {
    let toasts: ToastCx = create_rw_signal(cx, Vec::new());
    provide_context(cx, toasts);

    let close = move |id: u64| {
        toasts.update(|toasts| toasts.retain(|t| t.id != id));
    };

    view! { cx,
        <div class="toast toast-end z-[1000] p-0 m-0 gap-0">
            <For
                each=move || toasts.get()
                key=|t| format!("toast_{}", t.id)
                view=move |cx, t| {
                    view! { cx,
                        <div
                            style="width: unset"
                            class=concat!(
                                "mb-4 mr-4 z-[1000] alert max-w-lg flex flex-nowrap ",
                                "bg-neutral-950/30 border-0 drop-shadow-xl"
                            )
                        >

                            {move || {
                                let (class, icon) = match t.typ {
                                    ToastType::Error => {
                                        ("text-error", crate::icon!(FaCircleExclamationSolid))
                                    }
                                    ToastType::Info => {
                                        ("text-info", crate::icon!(FaCircleInfoSolid))
                                    }
                                    ToastType::Warning => {
                                        ("text-warning", crate::icon!(FaTriangleExclamationSolid))
                                    }
                                    ToastType::Success => {
                                        ("text-success", crate::icon!(FaCheckSolid))
                                    }
                                };

                                view! { cx,
                                    <div class=class>
                                        <Icon icon=icon/>
                                    </div>
                                }
                            }}

                            <div class="whitespace-break-spaces">{t.msg.clone()}</div>
                            <button
                                class="btn btn-circle btn-sm btn-ghost"
                                on:click=move |_| close(t.id)
                            >
                                <Icon icon=crate::icon!(FaXmarkSolid)/>
                            </button>
                        </div>
                    }
                }
            />

        </div>
        {children(cx)}
    }
}

#[component(transparent)]
pub fn ToastedSusp<F, FIV>(cx: Scope, fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> FIV + 'static,
    FIV: IntoView,
{
    let children = store_value(cx, children);
    let fallback = store_value(cx, fallback);

    view! { cx,
        <Suspense fallback=move || fallback.with_value(|f| f())>
            <ErrorBoundary fallback=move |cx, errs| {
                for (_, err) in errs.get() {
                    toast(cx, Toast::error(err.to_string()));
                }
                fallback.with_value(|f| f())
            }>
                <div>{children.with_value(|c| c(cx))}</div>
            </ErrorBoundary>
        </Suspense>
    }
}
