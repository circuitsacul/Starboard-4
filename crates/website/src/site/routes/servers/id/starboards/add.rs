use leptos::*;
use leptos_router::*;

use crate::site::{components::Popup, routes::servers::id::GuildContext};

use super::CreateStarboardAction;

#[component]
pub fn Add(cx: Scope) -> impl IntoView {
    let guild = expect_context::<GuildContext>(cx);
    let create_sb = expect_context::<CreateStarboardAction>(cx);

    view! { cx,
        <Suspense fallback=|| ()>
            <ActionForm action=create_sb>
                <Popup
                    actions=move || {
                        view! { cx,
                            <div class="flex-1"></div>
                            <A class="btn btn-ghost" href="..">
                                "Cancel"
                            </A>
                            <button type="submit" class="btn btn-primary">
                                "Create"
                            </button>
                        }
                    }

                    title=|| "Create Starboard"
                >
                    {move || {
                        let Some(Ok(Some(g))) = guild.read(cx) else { return None;
                    };
                        let tview = 
                        view! { cx,
                            <input type="hidden" name="guild_id" value=g.http.id.to_string()/>
                            "hi"
                        };
                        Some(tview)
                    }}

                </Popup>
            </ActionForm>
        </Suspense>
    }
}
