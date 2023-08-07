use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use twilight_model::user::CurrentUserGuild;

#[server(GetGuilds, "/api")]
pub async fn get_guilds(cx: Scope) -> Result<Vec<CurrentUserGuild>, ServerFnError> {
    use super::get_manageable_guilds;

    let Some(guilds) = get_manageable_guilds(cx).await else {
        return Err(ServerFnError::ServerError("Unauthorized.".to_string()));
    };

    let mut guilds: Vec<_> = guilds.iter().map(|(_, v)| v.clone()).collect();
    guilds.sort_by(|l, r| l.name.cmp(&r.name));

    Ok(guilds)
}

#[component]
pub fn ServerList(cx: Scope) -> impl IntoView {
    let guilds = create_local_resource(cx, move || (), move |_| get_guilds(cx));

    let guild_cards = move || {
        guilds.with(cx, move |guilds| {
            guilds.clone().map(move |guilds| {
                view! { cx,
                    <For
                        each=move || guilds.clone()
                        key=|g| g.id
                        view=move |cx, g| view! { cx, <ServerCard guild=g/> }
                    />
                }
            })
        })
    };
    let susp = move || {
        view! { cx,
            <For
                each=move || 0..10
                key=|v| v.to_owned()
                view=move |cx, _| view! { cx, <ServerCardSkeleton/> }
            />
        }
    };
    view! { cx,
        <div class="flex justify-center">
            <div class="max-w-4xl m-12 mt-0">
                <Suspense fallback=susp>{guild_cards}</Suspense>
            </div>
        </div>
    }
}

#[component]
fn ServerCardSkeleton(cx: Scope) -> impl IntoView {
    view! { cx,
        <button class="btn btn-lg btn-block btn-ghost my-2 normal-case btn-disabled !bg-transparent animate-pulse">
            <div class="avatar">
                <div class="w-12 mask mask-squircle bg-gray-700 bg-opacity-30"></div>
            </div>
            <div class="flex-1">
                <div class="h-5 bg-gray-700 bg-opacity-30 rounded-full w-[400px]"></div>
            </div>
            <Icon icon=crate::icon!(FaChevronRightSolid)/>
        </button>
    }
}

#[component]
fn ServerCard(cx: Scope, guild: CurrentUserGuild) -> impl IntoView {
    let icon_url = guild
        .icon
        .map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon));

    view! { cx,
        <A href=guild.id.to_string()>
            <button class="btn btn-lg btn-block btn-ghost my-2 normal-case">
                {match icon_url {
                    Some(url) => {
                        view! { cx,
                            <div class="avatar">
                                <div class="w-12 mask mask-squircle">
                                    <img src=url/>
                                </div>
                            </div>
                        }
                    }
                    None => {

                        view! { cx,
                            <div class="avatar">
                                <div class="w-12 mask mask-squircle bg-gray-500"></div>
                            </div>
                        }
                    }
                }}
                <div class="flex-1 text-left">{guild.name}</div>
                <Icon icon=crate::icon!(FaChevronRightSolid)/>
            </button>
        </A>
    }
}
