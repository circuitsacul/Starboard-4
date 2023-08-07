use std::time::Duration;

use instant::Instant;
use leptos::*;
use leptos_icons::*;

#[derive(Clone, Copy)]
pub enum ToastType {
    Error,
    Warning,
    Info,
    Success,
}

impl ToastType {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Error => "alert-error",
            Self::Warning => "alert-warning",
            Self::Info => "alert-info",
            Self::Success => "alert-success",
        }
    }
}

#[derive(Clone)]
pub struct Toast {
    pub typ: ToastType,
    pub msg: String,
    pub ts: Instant,
    pub lifespan: Duration,
}

impl Toast {
    pub fn error(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Error,
            msg: msg.to_string(),
            ts: Instant::now(),
            lifespan: Duration::from_secs(5),
        }
    }

    pub fn warning(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Warning,
            msg: msg.to_string(),
            ts: Instant::now(),
            lifespan: Duration::from_secs(5),
        }
    }

    pub fn info(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Info,
            msg: msg.to_string(),
            ts: Instant::now(),
            lifespan: Duration::from_secs(5),
        }
    }

    pub fn success(msg: impl ToString) -> Self {
        Self {
            typ: ToastType::Info,
            msg: msg.to_string(),
            ts: Instant::now(),
            lifespan: Duration::from_secs(5),
        }
    }
}

pub type ToastCx = RwSignal<Vec<Toast>>;

pub fn toast(cx: Scope, toast: Toast) {
    let lifespan = toast.lifespan;
    let ts = toast.ts;
    let toasts = expect_context::<ToastCx>(cx);

    toasts.update(|toasts| {
        toasts.push(toast);
    });

    create_effect(cx, move |_| {
        set_timeout(
            move || {
                toasts.try_update(|toasts| {
                    toasts.retain(|t| t.ts != ts);
                });
            },
            lifespan,
        )
    });
}

#[component]
pub fn ToastProvider(cx: Scope, children: Children) -> impl IntoView {
    let toasts: ToastCx = create_rw_signal(cx, Vec::<Toast>::new());
    provide_context(cx, toasts);

    let close = move |ts: Instant| {
        toasts.update(|toasts| toasts.retain(|t| t.ts != ts));
    };

    view! { cx,
        <div class="toast toast-end z-50">
            <For
                each=move || toasts.get()
                key=|t| t.ts
                view=move |cx, t| {
                    let t = store_value(cx, t);
                    view! { cx,
                        <div class=format!("z-50 alert {}", t.with_value(| t | t.typ.as_class()))>
                            <span>{t.with_value(|t| t.msg.clone())}</span>
                            <button
                                class="btn btn-circle btn-sm btn-ghost"
                                on:click=move |_| t.with_value(|t| close(t.ts))
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
