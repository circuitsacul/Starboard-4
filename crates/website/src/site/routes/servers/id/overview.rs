use leptos::*;

use crate::site::components::ToastedSusp;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let guild = expect_context::<super::GuildContext>(cx);

    let content = move || {
        guild.with(cx, |g| {
            g.as_ref()
                .map(|g| format!("{g:?}"))
                .map_err(|e| e.to_owned())
        })
    };
    view! { cx, <ToastedSusp fallback=|| ()>{content}</ToastedSusp> }
}
