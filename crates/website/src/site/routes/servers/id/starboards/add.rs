use leptos::*;
use leptos_router::*;

use crate::site::{
    components::{form::ErrorNote, Popup},
    routes::servers::id::{
        components::{ChannelPickerPopup, ChannelPickerProvider, SingleChannelPickerInput},
        GuildContext,
    },
};

use super::CreateStarboardAction;

#[component]
pub fn Add() -> impl IntoView {
    let guild = expect_context::<GuildContext>();
    let create_sb = expect_context::<CreateStarboardAction>();
    let errs = create_memo(move |_| match create_sb.value().get() {
        Some(Ok(v)) => v,
        _ => Default::default(),
    });

    view! {
        <ChannelPickerProvider categories=false>
            <ActionForm action=create_sb>
                <Popup
                    actions=move || {
                        view! {
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
                    <Suspense fallback=|| ()>
                        {move || guild.get().and_then(|v| v.ok().flatten()).map(|g| {
                            view! {
                                <input
                                    type="hidden"
                                    name="guild_id"
                                    value=g.http.id.to_string()
                                />
                            }
                        })}
                    </Suspense>
                    <div class="flex flex-col items-start gap-4">
                        <div class="w-full">
                            <SingleChannelPickerInput name="channel_id"/>
                            <ErrorNote errs=errs key="channel_id"/>
                        </div>
                        <div class="w-full">
                            <input
                                type="text"
                                name="name"
                                placeholder="Name"
                                class="input input-bordered w-full"
                            />
                            <ErrorNote errs=errs key="name"/>
                        </div>
                    </div>
                </Popup>
            </ActionForm>
            <ChannelPickerPopup propagate=false single=true name="channel_id" />
        </ChannelPickerProvider>
    }
}
