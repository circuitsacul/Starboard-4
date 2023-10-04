use std::time::Duration;

use database::Starboard;
use humantime::format_duration;
use leptos::*;
use twilight_model::guild::Guild;

use crate::site::components::{
    form::{ErrorNote, Label, ValidationErrors},
    MultiEmojiInput,
};

#[component]
pub fn Requirements<E: SignalWith<ValidationErrors> + Copy + 'static>(
    cx: Scope,
    errs: E,
    sb: Starboard,
    guild: Guild,
    hidden: Memo<bool>,
) -> impl IntoView {
    let required_enabled = create_rw_signal(cx, sb.settings.required.is_some());
    let required_remove_enabled = create_rw_signal(cx, sb.settings.required_remove.is_some());
    let newer_than_enabled = create_rw_signal(cx, sb.settings.newer_than > 0);
    let older_than_enabled = create_rw_signal(cx, sb.settings.older_than > 0);

    view! { cx,
        <div class:hidden=hidden>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                    <Label for_="required">"Required"</Label>
                    <div class="flex flex-row items-center gap-2">
                        <input
                            type="checkbox"
                            checked=required_enabled
                            on:input=move |e| required_enabled.set(event_target_checked(&e))
                            class="toggle toggle-primary"
                        />
                        <input
                            type="number"
                            name="required"
                            id="required"
                            value=sb.settings.required.unwrap_or(3)
                            class="input input-bordered input-sm"
                            disabled=move || !required_enabled.get()
                            /* TODO: min/max */
                        />
                    </div>
                    <ErrorNote errs=errs key="required"/>
                </div>

                <div>
                    <Label for_="required_remove">"Required to Remove"</Label>
                    <div class="flex flex-row items-center gap-2">
                        <input
                            type="checkbox"
                            checked=required_remove_enabled
                            on:input=move |e| required_remove_enabled.set(event_target_checked(&e))
                            class="toggle toggle-primary"
                        />
                        <input
                            type="number"
                            name="required_remove"
                            id="required_remove"
                            value=sb.settings.required_remove.unwrap_or(0)
                            class="input input-bordered input-sm"
                            disabled=move || !required_remove_enabled.get()
                            /* TODO: min/max */
                        />
                    </div>
                    <ErrorNote errs=errs key="required_remove"/>
                </div>

                <div>
                    <Label for_="upvote_emojis">"Upvote Emojis"</Label>
                    <MultiEmojiInput
                        id="upvote_emojis"
                        name="upvote_emojis"
                        initial=sb.settings.upvote_emojis
                        guild=guild.clone()
                    />
                    <ErrorNote errs=errs key="upvote_emojis"/>
                </div>
                <div>
                    <Label for_="downvote_emojis">"Downvote Emojis"</Label>
                    <MultiEmojiInput
                        id="downvote_emojis"
                        name="downvote_emojis"
                        initial=sb.settings.downvote_emojis
                        guild=guild.clone()
                    />
                    <ErrorNote errs=errs key="downvote_emojis"/>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        id="self_vote"
                        name="self_vote"
                        checked=sb.settings.self_vote
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="self_vote">"Self Vote"</Label>
                </div>
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="allow_bots"
                        id="allow_bots"
                        checked=sb.settings.allow_bots
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="allow_bots">"Allow Bots"</Label>
                </div>
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="require_image"
                        id="require_image"
                        checked=sb.settings.require_image
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="require_image">"Require Image"</Label>
                </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                    <Label for_="newer_than">"Newer Than"</Label>
                    <div class="flex flex-row items-center gap-2">
                        <input
                            type="checkbox"
                            checked=newer_than_enabled
                            on:input=move |e| newer_than_enabled.set(event_target_checked(&e))
                            class="toggle toggle-primary"
                        />
                        <input
                            type="input"
                            name="newer_than"
                            id="newer_than"
                            value=format_duration(Duration::from_secs(sb.settings.newer_than as _)).to_string()
                            class="input input-bordered input-sm"
                            disabled=move || !newer_than_enabled.get()
                        />
                    </div>
                    <ErrorNote errs=errs key="newer_than"/>
                </div>
                <div>
                    <Label for_="older_than">"Older Than"</Label>
                    <div class="flex flex-row items-center gap-2">
                        <input
                            type="checkbox"
                            checked=older_than_enabled
                            on:input=move |e| older_than_enabled.set(event_target_checked(&e))
                            class="toggle toggle-primary"
                        />
                        <input
                            type="input"
                            name="older_than"
                            id="older_than"
                            value=format_duration(Duration::from_secs(sb.settings.older_than as _)).to_string()
                            class="input input-bordered input-sm"
                            disabled=move || !older_than_enabled.get()
                        />
                    </div>
                    <ErrorNote errs=errs key="older_than"/>
                </div>
            </div>
        </div>
    }
}
