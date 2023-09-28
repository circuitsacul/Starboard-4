use leptos::*;
use twilight_model::id::{marker::EmojiMarker, Id};

use crate::site::routes::servers::id::GuildContext;

#[component]
pub fn EmojiButton<I: ToString>(
    cx: Scope,
    id: &'static str,
    name: &'static str,
    initial: I,
) -> impl IntoView {
    let guild = use_context::<GuildContext>(cx);

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
            class="btn btn-ghost btn-sm btn-square text-xl"
            on:click=move |_| {div_ref.get().map(|elm| elm.style("display", "block"));}
        >
            {move || {
                let emoji = value.get();
                if emoji.is_empty() {
                    "+".into_view(cx)
                } else if let Ok(id) = emoji.parse::<Id<EmojiMarker>>() {
                    view! {cx,
                        <img
                            src=format!("https://cdn.discordapp.com/emojis/{id}")
                            style="max-width: 1em; max-height: 1em;"
                        />
                    }.into_view(cx)
                } else {
                    view! {cx,
                        <em-emoji native={emoji.clone()} fallback={emoji} set="twitter"/>
                    }.into_view(cx)
                }
            }}
        </button>
        <div
            ref=div_ref
            id=format!("picker_container_{id}")
            class="fixed"
            style="display: none"
        />
        <Suspense fallback=|| ()>
            {move || {
                let emojis = guild.and_then(|g| {
                    g.with(cx, |g| {
                        g.as_ref()
                            .ok()
                            .and_then(|g| g.as_ref().map(|g| g.http.emojis.clone()))
                    })
                    .flatten()
                });
                let custom = serde_json::to_string(&emojis.map(|emojis| {
                    emojis.into_iter().map(|emoji| {
                        serde_json::json!({
                            "name": emoji.name,
                            "id": emoji.id.to_string(),
                            "keywords": [emoji.name],
                            "skins": [{
                                "src": format!("https://cdn.discordapp.com/emojis/{}", emoji.id)
                            }]
                        })
                    }).collect::<Vec<_>>()
                }).unwrap_or_default()).unwrap();
                let custom_id = guild.and_then(
                    |g| g.with(cx, |g| g.as_ref().ok().and_then(
                        |g| g.as_ref().map(|g| g.http.id.to_string())
                    ))
                ).flatten().unwrap_or_default();
                view! {cx,
                    <script>
                        {format!(r#"
                            picker_container_{id}.replaceChildren(new EmojiMart.Picker(
                                {{
                                    set: 'twitter',
                                    custom: [{{
                                        id: "{custom_id}",
                                        name: "Custom",
                                        emojis: {custom},
                                    }}],
                                    maxFrequentRows: 0,
                                    onEmojiSelect: (emoji) => {{
                                        console.log(emoji);
                                        if (emoji.native !== undefined) {{
                                            {id}.value = emoji.native;
                                        }} else {{
                                            {id}.value = emoji.id;
                                        }}
                                        var changeEvent = document.createEvent("HTMLEvents");
                                        changeEvent.initEvent("change", true, false);
                                        {id}.dispatchEvent(changeEvent);
                                        picker_container_{id}.style.display = "none";
                                    }},
                                    onClickOutside: () => picker_container_{id}.style.display = "none"
                                }}
                            ));
                        "#)}
                    </script>
                }
            }}
        </Suspense>
    }
}
