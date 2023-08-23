use leptos::*;
use leptos_icons::*;

#[derive(Clone)]
pub struct PickerItem {
    pub icon: Icon,
    pub name: String,
    pub value: String,
    pub children: Vec<PickerItem>,
    pub selected: RwSignal<bool>,
    pub search_visible: Option<Signal<bool>>,
}

fn recursive_set_search_visible<S>(cx: Scope, search: S, items: &mut [PickerItem])
where
    S: SignalWith<String> + Clone + Copy + 'static,
{
    for item in items {
        recursive_set_search_visible(cx, search, &mut item.children);
        let child_signals: Vec<_> = item
            .children
            .iter()
            .map(|item| item.search_visible.unwrap())
            .collect();
        let name = item.name.to_lowercase();
        let sig = Signal::derive(cx, move || {
            let children = child_signals.iter().any(|sig| sig.get());
            let this = search.with(|t| t.is_empty() || name.contains(t));
            children || this
        });
        item.search_visible.replace(sig);
    }
}

fn count_selected(items: &[PickerItem]) -> usize {
    let mut count = 0;

    for item in items {
        if item.selected.get() {
            count += 1;
        }
        count += count_selected(&item.children)
    }

    count
}

fn flatten_items(items: Vec<PickerItem>) -> Vec<PickerItem> {
    let mut result = Vec::new();

    for mut item in items {
        let children = std::mem::take(&mut item.children);
        let children = flatten_items(children);
        result.push(item);
        result.extend(children);
    }

    result
}

#[component]
pub fn Picker(
    cx: Scope,
    data: Vec<PickerItem>,
    propagate: bool,
    id: &'static str,
) -> impl IntoView {
    view! {cx,
        <PickerInput data=data.clone() id=id/>
        <Popup items=data propagate=propagate id=id/>
    }
}

#[component]
pub fn PickerInput(cx: Scope, data: Vec<PickerItem>, id: &'static str) -> impl IntoView {
    let flat_data = flatten_items(data.clone());
    let flat_data2 = flat_data.clone();

    view! {cx,
        <select hidden id=id name=id>
            <For
                each=move || flat_data.clone()
                key=|p| p.value.clone()
                view=move |cx, p| view! {cx,
                    <option value=p.value selected=move || p.selected.get()/>
                }
            />
        </select>
        <button onclick=format!("popup_{id}.showModal()") type="button" class="btn btn-ghost">
            {move || flat_data2.iter().filter(|c| c.selected.get()).count()}
        </button>
    }
}

#[component]
pub fn Popup(
    cx: Scope,
    mut items: Vec<PickerItem>,
    propagate: bool,
    id: &'static str,
) -> impl IntoView {
    let search = create_rw_signal(cx, "".to_string());
    recursive_set_search_visible(cx, search, &mut items);
    view! {cx,
        <dialog id=format!("popup_{id}") class="modal">
            <form method="dialog" class="modal-box h-screen max-w-sm">
                <input
                    type="text"
                    placeholder="Search"
                    class="input input-bordered w-full mb-2"
                    on:input=move |e| search.set(event_target_value(&e).to_lowercase())
                />
                <ItemPills
                    items=items.clone()
                    propagate=propagate
                    search=search
                    disabled=Signal::derive(cx, || false)
                />
            </form>
            <form method="dialog" class="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    }
}

#[component]
pub fn ItemPills<DisabledS, SearchS>(
    cx: Scope,
    items: Vec<PickerItem>,
    propagate: bool,
    disabled: DisabledS,
    search: SearchS,
) -> impl IntoView
where
    DisabledS: SignalGet<bool> + Clone + Copy + 'static,
    SearchS: SignalWith<String> + Clone + Copy + 'static,
{
    view! {cx,
        <For
            each=move || items.clone()
            key=|p| p.value.clone()
            view=move |cx, p| {
                let has_children = !p.children.is_empty();
                let search_visible = p.search_visible.unwrap();
                let show_children = create_rw_signal(cx, false);
                let children_shown = Signal::derive(
                    cx, move || show_children.get() || search.with(|t| !t.is_empty())
                );
                let pclone = p.clone();
                view! {cx,
                    <Show when=move || search_visible.get() fallback=|_| ()>
                        <div class="m-1 flex gap-x-1">
                            <Show
                                when=move || has_children
                                fallback=|cx| view! { cx, <div style="width: 1.5rem"></div>}
                            >
                                <button
                                    type="button"
                                    class="btn btn-xs btn-ghost btn-circle swap swap-rotate"
                                    class=("swap-active", move || !children_shown.get())
                                    on:click=move |_| show_children.update(|v| *v = !*v)
                                    disabled=search.with(|t| !t.is_empty())
                                >
                                    <Icon class="swap-on" icon=crate::icon!(FaChevronRightSolid)/>
                                    <Icon class="swap-off" icon=crate::icon!(FaChevronDownSolid)/>
                                </button>
                            </Show>
                            <ItemPill item=pclone.clone() disabled=disabled/>
                        </div>
                    </Show>
                    <Show
                        when=move || has_children && children_shown.get() && search_visible.get()
                        fallback=|_| ()
                    >
                        {
                            let child_disabled = Signal::derive(
                                cx,
                                move || disabled.get() || (p.selected.get() && propagate)
                            );
                            let items = p.children.clone();
                            move || {
                                view! {cx,
                                    <div class="ml-8">
                                        <ItemPills
                                            items=items.clone()
                                            propagate=propagate
                                            search=search
                                            disabled=child_disabled
                                        />
                                    </div>
                                }
                            }
                        }
                    </Show>
                }
            }
        />
    }
}

#[component]
pub fn ItemPill<S>(cx: Scope, item: PickerItem, disabled: S) -> impl IntoView
where
    S: SignalGet<bool> + Clone + Copy + 'static,
{
    view! {cx,
        <button
            type="button"
            class="btn btn-xs normal-case rounded-full"
            class=("btn-primary", item.selected)
            class=("btn-disabled", move || disabled.get())
            on:click=move |_| item.selected.update(|v| *v = !*v)
        >
            <Icon icon=item.icon/>
            {item.name.clone()}
            {move || match count_selected(&item.children) {
                0 => ().into_view(cx),
                c => view! { cx,
                    <div class=("text-primary", move || !item.selected.get() && !disabled.get())>
                        {format!(" ({c})")}
                    </div>
                }.into_view(cx)
            }}
        </button>
    }
}
