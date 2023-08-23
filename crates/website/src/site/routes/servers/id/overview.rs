use leptos::*;

use crate::site::routes::servers::id::components::ChannelPicker;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    view! { cx,
        <ChannelPicker propagate=false id="non_propagating"/>
        <ChannelPicker propagate=true id="propagating"/>
    }
}
