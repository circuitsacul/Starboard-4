use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct Props {
    id: i64,
}

#[component]
pub fn Server(cx: Scope) -> impl IntoView {
    let params = use_params::<Props>(cx);

    let id = move || params.with(|p| p.as_ref().unwrap().id);

    view! { cx,
        <nav>
            <ServerNavBar/>
        </nav>
        <main>{id}</main>
    }
}

#[component]
fn ServerNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="navbar">
            <div>
                <A href=".." class="btn btn-sm btn-ghost">
                    <Icon icon=crate::icon!(FaChevronLeftSolid)/>
                    Back
                </A>
            </div>
        </div>
    }
}
