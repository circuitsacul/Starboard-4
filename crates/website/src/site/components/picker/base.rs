use leptos::*;

#[derive(Debug, Clone)]
pub struct PickerItem {
    pub view: View,
    pub value: String,
    pub children: Vec<PickerItem>,
    pub selected: RwSignal<bool>,
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
pub fn Picker(cx: Scope, data: Vec<PickerItem>) -> impl IntoView {
    view! {cx,
        <PickerInput data=data.clone()/>
        <ItemPills items=data/>
    }
}

#[component]
pub fn PickerInput(cx: Scope, data: Vec<PickerItem>) -> impl IntoView {
    let flat_data = flatten_items(data.clone());

    view! {cx,
        <select hidden>
            <For
                each=move || flat_data.clone()
                key=|p| p.value.clone()
                view=move |cx, p| view! {cx,
                    <option value=p.value selected=move || p.selected.get()/>
                }
            />
        </select>
    }
}

#[component]
pub fn ItemPills(cx: Scope, items: Vec<PickerItem>) -> impl IntoView {
    view! {cx,
        <For
            each=move || items.clone()
            key=|p| p.value.clone()
            view=move |cx, p| {
                let has_children = !p.children.is_empty();
                view! {cx,
                    <Show when=move || !p.selected.get() && has_children fallback=|_| ()>
                        <div style="display: block"/>
                    </Show>
                    <ItemPill item=p.clone()/>
                    <Show
                        when=move || !p.selected.get() && has_children
                        fallback=|_| ()
                    >
                        {
                            let items = p.children.clone();
                            move || {
                                view! {cx,
                                    <div class="ml-8">
                                        <ItemPills items=items.clone()/>
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
pub fn ItemPill(cx: Scope, item: PickerItem) -> impl IntoView {
    view! {cx,
        <button
            type="button"
            class="btn btn-xs m-1 normal-case"
            class=("btn-primary", item.selected)
            on:click=move |_| item.selected.update(|v| *v = !*v)
        >
            {item.view}
        </button>
    }
}
