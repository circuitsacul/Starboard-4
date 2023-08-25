use leptos::*;
use leptos_router::*;

use crate::site::{
    components::Popup,
    routes::servers::id::{
        components::{ChannelPickerPopup, ChannelPickerProvider, SingleChannelPickerInput},
        GuildContext,
    },
};

use super::CreateStarboardAction;

#[component]
pub fn Add(cx: Scope) -> impl IntoView {
    let guild = expect_context::<GuildContext>(cx);
    let create_sb = expect_context::<CreateStarboardAction>(cx);

    view! { cx,
        <ChannelPickerProvider>
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
                            let tview = view! { cx,
                                <input type="hidden" name="guild_id" value=g.http.id.to_string()/>
                                <div class="flex flex-col items-start gap-4">
                                    <SingleChannelPickerInput id="channel_id"/>
                                    <input
                                        type="text"
                                        name="name"
                                        class="input input-bordered w-full"
                                    />
                                </div>
                            };
                            Some(tview)
                        }}

                    </Popup>
                </ActionForm>
            </Suspense>
            <ChannelPickerPopup propagate=false single=true id="channel_id" />
        </ChannelPickerProvider>
    }
}
