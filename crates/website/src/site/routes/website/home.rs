use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click class="btn">
            "Click Me: "
            {count}
        </button>
    }
}
