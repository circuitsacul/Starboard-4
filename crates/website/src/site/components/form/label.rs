use leptos::*;

#[component]
pub fn Label(cx: Scope, for_: &'static str, children: Children) -> impl IntoView {
    view! { cx,
        <label class="label" for=for_>
            <span class="label-text">{children(cx)}</span>
        </label>
    }
}
