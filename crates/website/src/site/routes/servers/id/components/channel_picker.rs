use std::collections::HashMap;

use leptos::*;
use leptos_icons::*;
use twilight_model::{
    channel::{Channel, ChannelType},
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use crate::site::{
    components::picker::{PickerItem, PickerMultiInput, PickerPopup, PickerSingleInput},
    routes::servers::id::{api::get_channels, GuildIdContext},
};

fn channel_sort_key(channel: &Channel) -> (i8, Option<i32>) {
    let typ = match channel.kind {
        ChannelType::GuildCategory => -1,
        ChannelType::GuildVoice | ChannelType::GuildStageVoice => 1,
        _ => 0,
    };

    (typ, channel.position)
}

fn channels_to_picker_items(
    cx: Scope,
    mut channels: Vec<Channel>,
    mut threads: Vec<Channel>,
) -> Vec<PickerItem> {
    channels.sort_by_key(channel_sort_key);
    // TODO: do threads have a position, or does this need to be by
    //       creation date/id/name
    threads.sort_by(|l, r| l.position.cmp(&r.position));

    let mut channel_threads = HashMap::<Id<ChannelMarker>, Vec<PickerItem>>::new();
    for t in threads {
        let name = t.name.unwrap_or("unknown".into());
        let item = PickerItem {
            icon: crate::icon!(FaMessageRegular),
            name,
            value: t.id.to_string(),
            children: Vec::new(),
            selected: create_rw_signal(cx, false),
            search_visible: None,
        };

        // SAFETY: all threads have a parent
        let parent = t.parent_id.unwrap();
        channel_threads
            .entry(parent)
            .or_insert_with(Vec::new)
            .push(item);
    }

    let mut lone_channels = Vec::<PickerItem>::new();
    let mut categories = Vec::<PickerItem>::new();
    let mut category_indices = HashMap::<Id<ChannelMarker>, usize>::new();

    for c in channels {
        let threads = channel_threads.remove(&c.id).unwrap_or_default();
        let mut item = PickerItem {
            icon: crate::icon!(FaHashtagSolid),
            name: c.name.unwrap_or("unknown".into()),
            value: c.id.to_string(),
            children: threads,
            selected: create_rw_signal(cx, false),
            search_visible: None,
        };

        match c.kind {
            ChannelType::GuildCategory => {
                item.icon = crate::icon!(FaBarsSolid);

                let idx = categories.len();
                categories.push(item);
                category_indices.insert(c.id, idx);
            }
            _ => {
                let category = match c.parent_id {
                    None => None,
                    Some(id) => category_indices.get(&id).copied(),
                };

                if let Some(category) = category {
                    // SAFETY: category indices are inserted
                    //         with the category item
                    categories[category].children.push(item);
                } else {
                    lone_channels.push(item);
                }
            }
        }
    }

    lone_channels.into_iter().chain(categories).collect()
}

pub type ChannelPickerResource =
    Resource<Option<Id<GuildMarker>>, Result<Vec<PickerItem>, ServerFnError>>;

#[component]
pub fn ChannelPickerProvider(cx: Scope, children: Children) -> impl IntoView {
    let guild_id = expect_context::<GuildIdContext>(cx);
    // local because PickerItem can't be Serialize/Deserialize
    let channels: ChannelPickerResource = create_local_resource(
        cx,
        move || guild_id.get(),
        move |guild_id| async move {
            let Some(guild_id) = guild_id else {
                return Err(ServerFnError::ServerError("No guild ID.".into()));
            };

            let (channels, threads) = get_channels(cx, guild_id).await?;
            Ok(channels_to_picker_items(cx, channels, threads))
        },
    );
    provide_context(cx, channels);

    view! {cx, {children(cx)}}
}

#[component]
pub fn ChannelPickerPopup(
    cx: Scope,
    propagate: bool,
    single: bool,
    id: &'static str,
) -> impl IntoView {
    let channels = expect_context::<ChannelPickerResource>(cx);

    view! {cx,
        <Suspense fallback=move || ()>
            {move || {
                channels.with(cx, |data| {
                    data.clone().map(|items| {
                        view! {cx,
                            <PickerPopup
                                items=items
                                propagate=propagate
                                single=single
                                id=id
                            />
                        }
                    })
                })
            }}
        </Suspense>
    }
}

#[component]
pub fn SingleChannelPickerInput(cx: Scope, id: &'static str) -> impl IntoView {
    let channels = expect_context::<ChannelPickerResource>(cx);

    view! {cx,
        <Suspense
            fallback=move || view! {cx,
                <button disabled class="btn btn-ghost btn-sm normal-case">
                    "Loading..."
                </button>
            }
        >
            {move || {
                channels.with(cx, |data| {
                    data.clone().map(|items| {
                        view! {cx,
                            <PickerSingleInput
                                data=items
                                id=id
                                placeholder="Select a channel"
                            />
                        }
                    })
                })
            }}
        </Suspense>
    }
}
#[component]
pub fn MultiChannelPickerInput(cx: Scope, id: &'static str) -> impl IntoView {
    let channels = expect_context::<ChannelPickerResource>(cx);

    view! {cx,
        <Suspense fallback=move || view! {cx,
            <div class=concat!(
                "inline-flex flex-row flex-wrap border border-base-content border-opacity-20 ",
                "rounded-btn p-2 gap-1"
            )>"Loading..."</div>
        }>
            {move || {
                channels.with(cx, |data| {
                    data.clone().map(|items| {
                        view! {cx,
                            <PickerMultiInput
                                data=items
                                id=id
                                placeholder="No channels selected"
                            />
                        }
                    })
                })
            }}
        </Suspense>
    }
}
