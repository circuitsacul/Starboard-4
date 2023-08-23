use leptos::*;

use crate::site::routes::servers::id::components::ChannelPicker;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    view! { cx,
        <ChannelPicker/>
    }
}
