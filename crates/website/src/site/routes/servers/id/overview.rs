use leptos::*;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let guild = expect_context::<super::GuildContext>(cx);

    let content = move || guild.with(cx, |g| format!("{g:?}"));
    view! { cx, <Suspense fallback=|| ()>{content}</Suspense> }
}
