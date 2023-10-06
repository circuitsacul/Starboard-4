mod api;
pub mod behavior;
pub mod regex;
pub mod requirements;
pub mod style;

use behavior::Behavior;
use regex::Regex;
use requirements::Requirements;
use style::Style;

use leptos::*;
use leptos_router::*;

use crate::site::{
    components::{toast, FullScreenPopup, Toast},
    routes::servers::id::{GuildContext, GuildIdContext},
};

use super::DeleteStarboardAction;

#[derive(Params, PartialEq, Clone)]
struct Props {
    starboard_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Behavior,
    Regex,
    Requirements,
    Style,
}

impl Tab {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Behavior => "Behavior",
            Self::Regex => "Regex",
            Self::Requirements => "Requirements",
            Self::Style => "Style",
        }
    }
}

#[component]
pub fn Starboard() -> impl IntoView {
    let update_sb = create_server_action::<self::api::UpdateStarboard>();
    let errs = create_memo(move |_| match update_sb.value().get() {
        Some(Ok(v)) => v,
        _ => Default::default(),
    });

    let current_tab = create_rw_signal(Tab::Requirements);

    let params = use_params::<Props>();
    let guild_id = expect_context::<GuildIdContext>();
    let guild = expect_context::<GuildContext>();

    let sb = create_resource(
        move || {
            let sb_id = params.get_untracked().ok().map(|p| p.starboard_id);
            let guild_id = guild_id.get();
            (sb_id, guild_id)
        },
        move |(sb_id, guild_id)| async move {
            let Some(sb_id) = sb_id else {
                return Ok((None, None));
            };
            let Some(guild_id) = guild_id else {
                return Ok((None, None));
            };

            self::api::get_starboard(guild_id, sb_id).await
        },
    );

    let make_is_hidden = move |tab: Tab| create_memo(move |_| tab != current_tab.get());

    create_effect(move |_| {
        match update_sb.value().get() {
            Some(Ok(errs)) => {
                if errs.is_empty() {
                    toast(Toast::success("Settings saved."));
                } else {
                    toast(Toast::warning(
                        "Some settings were saved, but there were some errors as well.",
                    ));
                }
            }
            Some(Err(e)) => toast(Toast::error(e)),
            None => (),
        };
    });

    let get_title = move || {
        let (sb_name, ch_name) = sb
            .with(|sb| {
                sb.as_ref()
                    .map(|v| v.as_ref().ok())
                    .flatten()
                    .map(|(sb, ch)| (sb.as_ref().map(|v| v.name.clone()), ch.clone()))
            })
            .unwrap_or((None, None));

        let sb_name = sb_name.unwrap_or_else(|| "unknown".into());
        let ch_name = ch_name.unwrap_or_else(|| "unknown".into());
        format!("'{sb_name}' in #{ch_name}")
    };

    view! {
        <Suspense fallback=|| ()>
            <Show
                when=move || sb.with(|s| matches!(s, Some(Ok((None, _)))))
                fallback=|| ()
            >
                <Redirect path=".."/>
            </Show>
        </Suspense>

        <ActionForm action=update_sb>
            <FullScreenPopup
                title=move || view! {
                    <Suspense fallback=||()>
                        {move || get_title()}
                    </Suspense>
                }
                actions=move || {
                    view! {
                        <div
                            class="btn btn-outline btn-error"
                            onclick="delete_sb_modal.showModal()"
                        >
                            "Delete"
                        </div>
                        <div class="flex-1"></div>
                        <A href=".." class="btn btn-ghost">
                            "Cancel"
                        </A>
                        <button
                            type="submit"
                            class="btn btn-primary"
                            disabled=move || update_sb.pending().get()
                        >
                            <Show when=move || update_sb.pending().get() fallback=|| ()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            "Save"
                        </button>
                    }
                }
            >

                <ul class="tabs">
                    <TabButton tab=Tab::Requirements sig=current_tab/>
                    <TabButton tab=Tab::Behavior sig=current_tab/>
                    <TabButton tab=Tab::Style sig=current_tab/>
                    <TabButton tab=Tab::Regex sig=current_tab/>
                </ul>
                <Suspense fallback=||()>
                    {move || {
                        let sb = sb.get();
                        let guild = guild.get();

                        let Some(Ok((Some(sb), _))) = sb else { return None; };
                        let Some(Ok(Some(guild))) = guild else { return None; };

                        Some(view! {
                            <input type="hidden" name="guild_id" value=sb.guild_id.to_string()/>
                            <input type="hidden" name="starboard_id" value=sb.id.to_string()/>

                            <Behavior errs=errs sb=sb.clone() hidden=make_is_hidden(Tab::Behavior)/>
                            <Regex errs=errs sb=sb.clone() hidden=make_is_hidden(Tab::Regex)/>
                            <Requirements
                                errs=errs
                                sb=sb.clone()
                                guild=guild.http.clone()
                                hidden=make_is_hidden(Tab::Requirements)
                            />
                            <Style errs=errs sb=sb.clone() guild=guild.http.clone() hidden=make_is_hidden(Tab::Style)/>
                        })
                    }}
                </Suspense>

            </FullScreenPopup>
        </ActionForm>

        <DeletePopup sb_id=params.get().unwrap().starboard_id/>
        // <Suspense fallback=|| ()>
        //     {move || {
        //         params.get().map(move |p| view! {<DeletePopup sb_id=p.starboard_id/>})
        //     }}
        // </Suspense>
    }
}

#[component]
pub fn TabButton(tab: Tab, sig: RwSignal<Tab>) -> impl IntoView {
    view! {
        <li>
            <button
                on:click=move |_| sig.set(tab)
                class="tab tab-bordered"
                class=("tab-active", move || sig.get() == tab)
                type="button"
            >
                {tab.as_str()}
            </button>
        </li>
    }
}

#[component]
pub fn DeletePopup(sb_id: i32) -> impl IntoView {
    let action = expect_context::<DeleteStarboardAction>();

    let guild_id = expect_context::<GuildIdContext>();

    view! {
        <dialog id="delete_sb_modal" class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-xl">"Are you sure?"</h3>
                <p class="py-4">"This will permanently delete this starboard."</p>
                <div class="modal-action">
                    <button class="btn btn-ghost" onclick="delete_sb_modal.close()">
                        "Cancel"
                    </button>
                    <ActionForm action=action>
                        <Suspense fallback=|| ()>
                            <input
                                type="hidden"
                                name="guild_id"
                                value=guild_id.get().map(|v| v.to_string()).unwrap_or_default()
                            />
                            <input type="hidden" name="starboard_id" value=sb_id/>
                        </Suspense>
                        // TODO: get the leptos bug fixed with pending actions on redirect
                        <button
                            class="btn btn-error"
                            // disabled=move || action.pending().get()
                        >
                            // <Show when=move || action.pending().get() fallback=|| ()>
                            //     <span class="loading loading-spinner loading-sm"></span>
                            // </Show>
                            "Delete"
                        </button>
                    </ActionForm>
                </div>
            </div>
        </dialog>
    }
}
