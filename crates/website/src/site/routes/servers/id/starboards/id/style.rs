use common::constants;
use database::Starboard;
use leptos::*;

#[component]
pub fn Style(cx: Scope, sb: Starboard, hidden: Memo<bool>) -> impl IntoView {
    view! {
        cx,
        <div class:hidden=hidden class="p-4">
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                    <Label for_="">"Display Emoji"</Label>
                    <input type="hidden" name="display_emoji" id="display_emoji"/>
                    <button type="button" class="btn btn-ghost btn-square">
                        {sb.settings.display_emoji.clone().unwrap_or_else(|| "".into())}
                    </button>
                </div>

                <div>
                    <Label for_="color">"Embed Color"</Label>
                    <input
                        type="color"
                        name="color"
                        id="color"
                        value=format!("#{:X}", sb.settings.color.unwrap_or(constants::BOT_COLOR as i32))
                    />
                </div>

                <div class="col-span-full">
                    <Label for_="go_to_message">"Go to Message"</Label>
                    <select name="go_to_message" id="go_to_message" class="select select-bordered">
                        <option value="0" selected=sb.settings.go_to_message==0>"Disabled"</option>
                        <option value="1" selected=sb.settings.go_to_message==1>"Link inside embed"</option>
                        <option value="2" selected=sb.settings.go_to_message==2>"Button"</option>
                        <option value="3" selected=sb.settings.go_to_message==3>"Link mention"</option>
                    </select>
                </div>
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-8">
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="ping_author"
                        id="ping_author"
                        checked=sb.settings.ping_author
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="ping_author">"Ping Author"</Label>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="use_server_profile"
                        id="use_server_profile"
                        checked=sb.settings.use_server_profile
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="use_server_profile">"Use Server Profile"</Label>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="use_webhook"
                        id="use_webhook"
                        checked=sb.settings.use_webhook
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="use_webhook">"Use Webhook"</Label>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="extra_embeds"
                        id="extra_embeds"
                        checked=sb.settings.extra_embeds
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="extra_embeds">"Extra Embeds"</Label>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="replied_to"
                        id="replied_to"
                        checked=sb.settings.replied_to
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="replied_to">"Show Replied To"</Label>
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        name="attachments_list"
                        id="attachments_list"
                        checked=sb.settings.attachments_list
                        class="checkbox checkbox-primary"
                    />
                    <Label for_="attachments_list">"Show Attachments List"</Label>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Label(cx: Scope, for_: &'static str, children: Children) -> impl IntoView {
    view! {cx,
        <label class="label" for=for_><span class="label-text">{children(cx)}</span></label>
    }
}
