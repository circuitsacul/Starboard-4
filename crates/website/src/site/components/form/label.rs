use leptos::*;

#[component]
pub fn Label(for_: &'static str, children: Children) -> impl IntoView {
    view! {
        <label class="label" for=for_>
            <span class="label-text">{children()}</span>
        </label>
    }
}
