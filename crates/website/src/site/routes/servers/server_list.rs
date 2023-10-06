use crate::site::components::ToastedSusp;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use twilight_model::user::CurrentUserGuild;

#[component]
pub fn ServerList() -> impl IntoView {
    let guilds = expect_context::<super::BaseGuildsResource>();

    let guild_cards = move || {
        let guilds = guilds.get().map(|guilds| {
            guilds.map(|guilds| {
                let mut guilds: Vec<_> = guilds.into_values().collect();
                guilds.sort_by(|l, r| l.name.cmp(&r.name));
                guilds
            })
        });
        guilds.as_ref().map(move |guilds| {
            guilds
                .as_ref()
                .map(|guilds| {
                    let guilds = guilds.to_owned();
                    view! {
                        <For
                            each=move || guilds.clone()
                            key=|g| g.id
                            children=move |g| view! { <ServerCard guild=g.to_owned()/> }
                        />
                    }
                })
                .map_err(|e| (*e).to_owned())
        })
    };
    let susp = move || {
        view! {
            <For
                each=move || 0..10
                key=|v| v.to_owned()
                children=move |_| view! { <ServerCardSkeleton/> }
            />
        }
    };
    view! {
        <div class="flex justify-center">
            <div class="max-w-4xl w-full p-1">
                <ToastedSusp fallback=susp>
                    {guild_cards}
                </ToastedSusp>
            </div>
        </div>
    }
}

#[component]
fn ServerCardSkeleton() -> impl IntoView {
    view! {
        <button class="btn btn-lg btn-block btn-ghost my-2 normal-case !flex-nowrap btn-disabled !bg-transparent animate-pulse">
            <div class="avatar">
                <div class="w-12 mask mask-squircle bg-gray-700 bg-opacity-30"></div>
            </div>
            <div class="flex-1">
                <div class="h-5 bg-gray-700 bg-opacity-30 rounded-full w-full max-w-[250px]"></div>
            </div>
            <Icon icon=crate::icon!(FaChevronRightSolid)/>
        </button>
    }
}

#[component]
fn ServerCard(guild: CurrentUserGuild) -> impl IntoView {
    let icon_url = guild
        .icon
        .map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon));

    view! {
        <A
            href=guild.id.to_string()
            class="btn btn-lg btn-block btn-ghost my-2 normal-case !flex-nowrap"
        >
            {match icon_url {
                Some(url) => {
                    view! {
                        <div class="avatar">
                            <div class="w-12 mask mask-squircle">
                                <img src=url/>
                            </div>
                        </div>
                    }
                }
                None => {
                    view! {
                        <div class="avatar">
                            <div class="w-12 mask mask-squircle bg-gray-500"></div>
                        </div>
                    }
                }
            }}

            <div class="flex-1 text-left truncate">{guild.name}</div>
            <Icon icon=crate::icon!(FaChevronRightSolid)/>
        </A>
    }
}
