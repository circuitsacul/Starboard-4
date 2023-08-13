pub mod behavior;
pub mod embed;
pub mod filters;
pub mod regex;
pub mod requirements;
pub mod style;

use behavior::Behavior;
use embed::Embed;
use filters::Filters;
use regex::Regex;
use requirements::Requirements;
use style::Style;

use leptos::*;
use leptos_router::*;
use twilight_model::id::Id;

use crate::site::{components::FullScreenPopup, routes::servers::id::get_flat_guild};

use super::get_starboard;

#[derive(PartialEq)]
pub struct StarboardId(pub i32);
pub type StarboardIdContext = Memo<Option<StarboardId>>;

#[derive(Params, PartialEq, Clone)]
struct Props {
    starboard_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Behavior,
    Embed,
    Filters,
    Regex,
    Requirements,
    Style,
}

impl Tab {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Behavior => "Behavior",
            Self::Embed => "Embed",
            Self::Filters => "Filters",
            Self::Regex => "Regex",
            Self::Requirements => "Requirements",
            Self::Style => "Style",
        }
    }
}

#[component]
pub fn Starboard(cx: Scope) -> impl IntoView {
    let params = use_params::<Props>(cx);
    let sb_id: StarboardIdContext = create_memo(cx, move |_| {
        params.with(|p| p.as_ref().ok().map(|p| StarboardId(p.starboard_id)))
    });

    provide_context(cx, sb_id);

    let get_sb = move |cx| {
        let Ok(params) = params.get() else {
            return None;
        };
        get_starboard(cx, params.starboard_id)
    };
    let get_title = move |cx| {
        let Some(sb) = get_sb(cx) else {
            return "".to_string();
        };

        let channel = 'out: {
            let Some(guild) = get_flat_guild(cx) else {
                break 'out "unknown channel".to_string();
            };
            match guild.channels.get(&Id::new(sb.channel_id as _)) {
                Some(channel) => match &channel.name {
                    Some(n) => format!("#{n}"),
                    None => "unknown channel".to_string(),
                },
                None => "deleted channel".to_string(),
            }
        };

        format!("Starboard '{}' in {}", sb.name, channel)
    };

    let current_tab = create_rw_signal(cx, Tab::Requirements);

    view! {cx,
        <FullScreenPopup
            title=move || view! {cx,
                <Suspense fallback=|| ()>{move || get_title(cx)}</Suspense>
            }
            actions=move || view! {cx,
                <div class="btn btn-outline btn-error">"Delete"</div>
                <div class="flex-1"/>
                <A href=".." class="btn btn-ghost">"Cancel"</A>
                <div class="btn btn-primary">"Save"</div>
            }
        >
            <ul class="menu menu-horizontal flex space-x-1">
                <TabButton tab=Tab::Requirements sig=current_tab/>
                <TabButton tab=Tab::Behavior sig=current_tab/>
                <TabButton tab=Tab::Style sig=current_tab/>
                <TabButton tab=Tab::Embed sig=current_tab/>
                <TabButton tab=Tab::Regex sig=current_tab/>
                <TabButton tab=Tab::Filters sig=current_tab/>
            </ul>
            <Suspense fallback=|| ()>
                {move || {
                    let Some(sb) = get_sb(cx) else {
                        return None;
                    };

                    let tview = match current_tab.get() {
                        Tab::Behavior => view! {cx, <Behavior sb=sb/>},
                        Tab::Embed => view! {cx, <Embed sb=sb/>},
                        Tab::Filters => view! {cx, <Filters sb=sb/>},
                        Tab::Regex => view! {cx, <Regex sb=sb/>},
                        Tab::Requirements => view! {cx, <Requirements sb=sb/>},
                        Tab::Style => view! {cx, <Style sb=sb/>},
                    };
                    Some(tview)
                }}
            </Suspense>
            <Outlet/>
        </FullScreenPopup>
    }
}

#[component]
pub fn TabButton(cx: Scope, tab: Tab, sig: RwSignal<Tab>) -> impl IntoView {
    view! {cx,
        <li>
            <button
                on:click=move |_| sig.set(tab)
                class=move || if sig.get() == tab { "active" } else { "" }
            >
                {tab.as_str()}
            </button>
        </li>
    }
}
