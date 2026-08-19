use dioxus::prelude::*;
use super::icons::IconSearch;
#[component]
pub fn SignalInput(
    mut value: Signal<String>,
    #[props(default, into)]
    placeholder: String,
    #[props(default, into)]
    class: String,
    #[props(default = "text".to_string(), into)]
    r#type: String,
) -> Element {
    rsx! {
        input {
            r#type: "{r#type}",
            class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none {class}",
            value: "{value}",
            placeholder: "{placeholder}",
            oninput: move |evt: FormEvent| {
                let next = evt.value();
                value.with_mut(|buf| buf.clone_from(&next));
            },
        }
    }
}
#[component]
pub fn StaticInput(
    #[props(into)]
    value: String,
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
        input {
            r#type: "text",
            class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none {class}",
            value: "{value}",
            readonly: true,
        }
    }
}
#[component]
pub fn SignalTextarea(
    mut value: Signal<String>,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
) -> Element {
    rsx! {
        textarea {
            class: "ui-input ui-squircle min-h-24 w-full resize-y px-4 py-3 text-sm outline-none {class}",
            value: "{value}",
            placeholder: "{placeholder}",
            oninput: move |evt: FormEvent| {
                let next = evt.value();
                value.with_mut(|buf| buf.clone_from(&next));
            },
        }
    }
}

#[component]
pub fn SearchInput(
    mut value: Signal<String>,
    #[props(default = "Search...".to_string(), into)]
    placeholder: String,
    #[props(default, into)]
    class: String,
) -> Element {
    rsx! {
        div { class: "relative flex min-w-0 {class}",
            span { class: "pointer-events-none absolute inset-y-0 left-3.5 z-10 flex items-center text-text-muted",
                IconSearch {}
            }
            SignalInput { value, placeholder, class: "h-9 pl-10" }
        }
    }
}
