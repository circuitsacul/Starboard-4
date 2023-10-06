use database::Starboard;
use leptos::*;

use crate::site::components::form::{ErrorNote, Label, ValidationErrors};

#[component]
pub fn Regex<E: SignalWith<Value = ValidationErrors> + Copy + 'static>(
    errs: E,
    sb: Starboard,
    hidden: Memo<bool>,
) -> impl IntoView {
    view! {
        <div class:hidden=hidden>
            <div>
                <Label for_="matches">"Matches"</Label>
                <input
                    type="text"
                    class="input input-bordered w-full"
                    value=sb.settings.matches
                    name="matches"
                    id="matches"
                />
                <ErrorNote errs=errs key="matches"/>
            </div>
            <div>
                <Label for_="not_matches">"Not Matches"</Label>
                <input
                    type="text"
                    class="input input-bordered w-full"
                    value=sb.settings.not_matches
                    name="not_matches"
                    id="not_matches"
                />
                <ErrorNote errs=errs key="not_matches"/>
            </div>
        </div>
    }
}
