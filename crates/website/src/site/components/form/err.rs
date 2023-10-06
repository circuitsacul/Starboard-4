use std::collections::HashMap;

use leptos::*;

pub type ValidationErrors = HashMap<String, String>;

#[component]
pub fn ErrorNote<E: SignalWith<Value = ValidationErrors> + 'static>(
    errs: E,
    key: &'static str,
) -> impl IntoView {
    let err = Signal::derive(move || errs.with(|errs| errs.get(key).cloned()));

    view! {
        <Show when=move || err.get().is_some() fallback=|| ()>
            <label class="label">
                <span class="label-text-alt text-error">
                    {move || err.get().unwrap_or_else(|| "".into())}
                </span>
            </label>
        </Show>
    }
}
