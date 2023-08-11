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

    view! {cx,
        <FullScreenPopup
            title=move || view! {cx,
                <Suspense fallback=|| ()>{move || get_title(cx)}</Suspense>
            }
            actions=move || view! {cx,
                <div class="btn btn-outline btn-error">Delete</div>
                <div class="flex-1"/>
                <A href=".." class="btn btn-ghost">Cancel</A>
                <div class="btn btn-primary">Save</div>
            }
        >
            <Suspense fallback=|| "loading...">
                {move || get_sb(cx).map(|sb| format!("{sb:?}"))}
            </Suspense>
        </FullScreenPopup>
    }
}
