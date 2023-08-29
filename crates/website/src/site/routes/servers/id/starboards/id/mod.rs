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
    routes::servers::id::GuildIdContext,
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
pub fn Starboard(cx: Scope) -> impl IntoView {
    let update_sb = create_server_action::<self::api::UpdateStarboard>(cx);
    let errs = create_memo(cx, move |_| match update_sb.value().get() {
        Some(Ok(v)) => v,
        _ => Default::default(),
    });

    let current_tab = create_rw_signal(cx, Tab::Requirements);

    let params = use_params::<Props>(cx);
    let guild_id = expect_context::<GuildIdContext>(cx);

    let sb = create_resource(
        cx,
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

            self::api::get_starboard(cx, guild_id, sb_id).await
        },
    );

    let make_is_hidden = move |tab: Tab| create_memo(cx, move |_| tab != current_tab.get());

    create_effect(cx, move |_| {
        match update_sb.value().get() {
            Some(Ok(errs)) => {
                if errs.is_empty() {
                    toast(cx, Toast::success("Settings saved."));
                } else {
                    toast(
                        cx,
                        Toast::warning(
                            "Some settings were saved, but there were some errors as well.",
                        ),
                    );
                }
            }
            Some(Err(e)) => toast(cx, Toast::error(e)),
            None => (),
        };
    });

    let get_title = move |cx| {
        let (sb_name, ch_name) = sb
            .with(cx, |sb| {
                sb.as_ref()
                    .ok()
                    .map(|(sb, ch)| (sb.as_ref().map(|v| v.name.clone()), ch.clone()))
            })
            .flatten()
            .unwrap_or((None, None));

        let sb_name = sb_name.unwrap_or_else(|| "unknown".into());
        let ch_name = ch_name.unwrap_or_else(|| "unknown".into());
        format!("'{sb_name}' in #{ch_name}")
    };

    view! { cx,
        <Suspense fallback=|| ()>
            <Show
                when=move || sb.with(cx, |s| matches!(s, Ok((None, _)))).unwrap_or(false)
                fallback=|_| ()
            >
                <Redirect path=".."/>
            </Show>
        </Suspense>

        <ActionForm action=update_sb>
            <FullScreenPopup
                title=move || view! {cx,
                    <Suspense fallback=||()>
                        {move || get_title(cx)}
                    </Suspense>
                }
                actions=move || {
                    view! { cx,
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
                            class=("btn-disabled", update_sb.pending().get())
                        >
                            <Show when=move || update_sb.pending().get() fallback=|_| ()>
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
                        let Some(Ok((Some(sb), _))) = sb.read(cx) else { return None;
                    };
                        let tview = view! { cx,
                            <input type="hidden" name="guild_id" value=sb.guild_id.to_string()/>
                            <input type="hidden" name="starboard_id" value=sb.id.to_string()/>

                            <Behavior errs=errs sb=sb.clone() hidden=make_is_hidden(Tab::Behavior)/>
                            <Regex errs=errs sb=sb.clone() hidden=make_is_hidden(Tab::Regex)/>
                            <Requirements
                                errs=errs
                                sb=sb.clone()
                                hidden=make_is_hidden(Tab::Requirements)
                            />
                            <Style errs=errs sb=sb.clone() hidden=make_is_hidden(Tab::Style)/>
                        };
                        Some(tview)
                    }}
                </Suspense>

            </FullScreenPopup>
        </ActionForm>

        <Suspense fallback=|| ()>
            {move || {
                params.get().map(move |p| view! {cx, <DeletePopup sb_id=p.starboard_id/>})
            }}
        </Suspense>
    }
}

#[component]
pub fn TabButton(cx: Scope, tab: Tab, sig: RwSignal<Tab>) -> impl IntoView {
    view! { cx,
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
pub fn DeletePopup(cx: Scope, sb_id: i32) -> impl IntoView {
    let action = expect_context::<DeleteStarboardAction>(cx);

    let guild_id = expect_context::<GuildIdContext>(cx);

    view! {cx,
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
                        <button
                            class="btn btn-error"
                            disabled=move || action.pending().get()
                        >
                            <Show when=move || action.pending().get() fallback=|_| ()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            "Delete"
                        </button>
                    </ActionForm>
                </div>
            </div>
        </dialog>
    }
}
