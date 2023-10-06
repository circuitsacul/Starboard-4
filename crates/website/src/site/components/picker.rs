use leptos::*;
use leptos_icons::*;

#[derive(Clone)]
pub struct PickerItem {
    pub icon: Icon,
    pub name: String,
    pub value: String,
    pub children: Vec<PickerItem>,
    pub selectable: bool,
    pub selected: RwSignal<bool>,
    pub search_visible: Option<Signal<bool>>,
}

impl PartialEq for PickerItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn recursive_set_search_visible<S>(search: S, items: &mut [PickerItem])
where
    S: SignalWith<Value = String> + Clone + Copy + 'static,
{
    for item in items {
        recursive_set_search_visible(search, &mut item.children);
        let child_signals: Vec<_> = item
            .children
            .iter()
            .map(|item| item.search_visible.unwrap())
            .collect();
        let name = item.name.to_lowercase();
        let sig = Signal::derive(move || {
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

fn clip_name(name: &str) -> String {
    if name.len() > 22 {
        format!("{}...", &name[0..19])
    } else {
        name.to_owned()
    }
}

#[component]
pub fn PickerSingleInput(
    data: Vec<PickerItem>,
    name: &'static str,
    placeholder: &'static str,
) -> impl IntoView {
    let flat_data = store_value(flatten_items(data.clone()));
    let selected = create_memo(move |_| {
        flat_data.with_value(|d| d.iter().find(|i| i.selected.get()).cloned())
    });

    view! {
        <input
            type="hidden"
            name=name
            prop:value=move || selected.get().map(|v| v.value.clone()).unwrap_or("".into())
        />
        <button
            type="button"
            onclick=format!("popup_{name}.showModal()")
            on:click=move |_| { if let Some(v) = selected.get() { v.selected.set(false) } }
            class="btn btn-ghost btn-sm normal-case"
        >
            <Show when=move || selected.with(|v| v.is_some()) fallback=move || placeholder>
                {move || {
                    selected.get().map(|selected| {
                        view! {
                            <Icon icon=selected.icon/>
                            {selected.name}
                        }
                    })
                }}
            </Show>
        </button>
    }
}

#[component]
pub fn PickerMultiInput(
    data: Vec<PickerItem>,
    name: &'static str,
    placeholder: &'static str,
) -> impl IntoView {
    let flat_data = store_value(flatten_items(data.clone()));
    let selected = create_memo(move |_| {
        flat_data
            .with_value(|d| d.clone().into_iter().filter(|i| i.selected.get()))
            .collect::<Vec<_>>()
    });

    view! {
        <select hidden name=name>
            <For
                each=move || flat_data.with_value(|d| d.clone())
                key=|p| p.value.clone()
                children=move |p| view! {
                    <option value=p.value selected=move || p.selected.get()/>
                }
            />
        </select>
        <div
            class=concat!(
                "inline-flex flex-row flex-wrap border border-base-content border-opacity-20 ",
                "rounded-btn p-2 gap-1"
            )
        >
            <Show when=move || !selected.with(|v| v.is_empty()) fallback=move || placeholder>
                <For
                    each=move || selected.get()
                    key=|item| item.value.clone()
                    children=move |item| view! {
                        <ItemPill item=item disabled=Signal::derive(|| false) single=false/>
                    }
                />
            </Show>
            <button
                onclick=format!("popup_{name}.showModal()")
                type="button"
                class="btn btn-xs btn-ghost normal-case"
            >
                "+ Add"
            </button>
        </div>
    }
}

#[component]
pub fn PickerPopup(
    mut items: Vec<PickerItem>,
    propagate: bool,
    single: bool,
    name: &'static str,
) -> impl IntoView {
    let search = create_rw_signal("".to_string());
    recursive_set_search_visible(search, &mut items);
    view! {
        <dialog id=format!("popup_{name}") class="modal">
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
                    single=single
                    search=search
                    disabled=Signal::derive(|| false)
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
    items: Vec<PickerItem>,
    propagate: bool,
    single: bool,
    disabled: DisabledS,
    search: SearchS,
) -> impl IntoView
where
    DisabledS: SignalGet<Value = bool> + Clone + Copy + 'static,
    SearchS: SignalWith<Value = String> + Clone + Copy + 'static,
{
    view! {
        <For
            each=move || items.clone()
            key=|p| p.value.clone()
            children=move |p| {
                let selectable = p.selectable;
                let has_children = !p.children.is_empty();
                let search_visible = p.search_visible.unwrap();
                let id = store_value(format!("picker_item_{}", p.value));

                let p = store_value(p);

                let show_children = create_rw_signal(false);
                let children_shown = Signal::derive(
                    move || show_children.get() || search.with(|t| !t.is_empty())
                );
                view! {
                    <Show when=move || search_visible.get() fallback=|| ()>
                        <div class="m-1 flex gap-x-1">
                            <Show
                                when=move || has_children
                                fallback=|| view! {<div style="width: 1.5rem"></div>}
                            >
                                <button
                                    id=id.with_value(|id| id.clone())
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
                            <Show when=move || selectable fallback=|| ()>
                                <ItemPill item=p.with_value(|v| v.to_owned()) disabled=disabled single=single/>
                            </Show>
                            <Show when=move || !selectable fallback=|| ()>
                                <label for=id.with_value(|id| id.clone()) class="text-xs flex items-center gap-2 px-1">
                                    <Icon icon=p.with_value(|p| p.icon)/>
                                    {move || p.with_value(|v| v.name.to_owned())}
                                </label>
                            </Show>
                        </div>
                    </Show>
                    <Show
                        when=move || has_children && children_shown.get() && search_visible.get()
                        fallback=|| ()
                    >
                        {
                            let child_disabled = Signal::derive(
                                move || disabled.get() || (p.with_value(|p| p.selected.get()) && propagate)
                            );
                            let items = p.with_value(|p| p.children.clone());
                            move || {
                                view! {
                                    <div class="ml-8">
                                        <ItemPills
                                            items=items.clone()
                                            propagate=propagate
                                            single=single
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
pub fn ItemPill<S>(item: PickerItem, disabled: S, single: bool) -> impl IntoView
where
    S: SignalGet<Value = bool> + Clone + Copy + 'static,
{
    view! {
        <button
            type=if single {"submit"} else {"button"}
            class="btn btn-xs normal-case rounded-full"
            class=("btn-primary", item.selected)
            on:click=move |_| item.selected.update(|v| *v = !*v)
            disabled=move || disabled.get()
        >
            <Icon icon=item.icon/>
            {clip_name(&item.name)}
            {move || match count_selected(&item.children) {
                0 => ().into_view(),
                c => view! {
                    <div class=("text-primary", move || !item.selected.get() && !disabled.get())>
                        {format!(" ({c})")}
                    </div>
                }.into_view()
            }}
        </button>
    }
}
