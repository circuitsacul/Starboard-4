use leptos::*;

use crate::site::routes::servers::id::components::ChannelPicker;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    view! { cx,
        <ChannelPicker propagate=false single=true id="non_propagating"/>
        <ChannelPicker propagate=true single=false id="propagating"/>
    }
}
