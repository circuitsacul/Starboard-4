use database::Starboard;
use leptos::*;

use crate::site::components::form::{ErrorNote, Label, ValidationErrors};

#[component]
pub fn Requirements<E: SignalWith<ValidationErrors> + Copy + 'static>(
    cx: Scope,
    errs: E,
    sb: Starboard,
    hidden: Memo<bool>,
) -> impl IntoView {
    let required_enabled = create_rw_signal(cx, sb.settings.required.is_some());
    let required_remove_enabled = create_rw_signal(cx, sb.settings.required_remove.is_some());

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
        </div>
    }
}
