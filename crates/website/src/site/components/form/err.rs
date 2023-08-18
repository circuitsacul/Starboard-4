use std::collections::HashMap;

use leptos::*;

pub type ValidationErrors = HashMap<String, String>;

#[component]
pub fn ErrorNote<E: SignalWith<ValidationErrors> + 'static>(
    cx: Scope,
    errs: E,
    key: &'static str,
) -> impl IntoView {
    let err = Signal::derive(cx, move || errs.with(|errs| errs.get(key).cloned()));

    view! { cx,
        <Show when=move || err.get().is_some() fallback=|_| ()>
            <label for=key class="label">
                <span class="label-text-alt text-error">{move || err.get().unwrap()}</span>
            </label>
        </Show>
    }
}
