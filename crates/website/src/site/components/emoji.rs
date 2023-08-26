use leptos::*;

use crate::site::routes::servers::id::GuildContext;

#[component]
pub fn EmojiButton<I: ToString>(
    cx: Scope,
    id: &'static str,
    name: &'static str,
    initial: I,
) -> impl IntoView {
    // TODO: custom emojis
    let _guild = use_context::<GuildContext>(cx);

    let value = create_rw_signal(cx, initial.to_string());
    let div_ref = create_node_ref::<html::Div>(cx);

    view! {cx,
        <input
            id=id
            name=name
            type="hidden"
            prop:value=value
            on:change=move |e| value.set(event_target_value(&e))
        />
        <button
            type="button"
            id=format!("picker_button_{id}")
            class="btn btn-ghost btn-square text-2xl"
            on:click=move |_| {div_ref.get().map(|elm| elm.style("display", "block"));}
        >
            {move || {
                let v = value.get();
                if v.is_empty() {
                    "+".to_string()
                } else {
                    v
                }
            }}
        </button>
        <div
            ref=div_ref
            id=format!("picker_container_{id}")
            class="fixed"
            style="display: none"
        />
        <script>
            {format!(
                r#"
                    new EmojiMart.Picker(
                        {{
                            parent: picker_container_{id},
                            onEmojiSelect: (emoji) => {{
                                {id}.value = emoji.native;
                                var changeEvent = document.createEvent("HTMLEvents");
                                changeEvent.initEvent("change", true, false);
                                {id}.dispatchEvent(changeEvent);
                                picker_container_{id}.style.display = "none";
                            }},
                            onClickOutside: () => picker_container_{id}.style.display = "none"
                        }}
                    );
                "#
            )}
        </script>
    }
}
