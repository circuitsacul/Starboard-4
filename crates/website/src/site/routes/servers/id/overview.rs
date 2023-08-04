use leptos::*;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let guild = expect_context::<super::GuildContext>(cx);

    view! { cx,
        <Suspense fallback=|| {
            view! { cx, "None" }
        }>{move || format!("{:?}", dbg!(guild.read(cx)))}</Suspense>
    }
}
