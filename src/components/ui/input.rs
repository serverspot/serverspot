use dioxus::prelude::*;

use super::icons::IconSearch;

#[component]
pub fn SignalInput(
    mut value: Signal<String>,
    #[props(default = "")] placeholder: &'static str,
    #[props(default = "")] class: &'static str,
    #[props(default = "text")] r#type: &'static str,
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
    value: &'static str,
    #[props(default = "")] class: &'static str,
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
pub fn SearchInput(
    mut value: Signal<String>,
    #[props(default = "Search...")] placeholder: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        div {
            class: "relative flex min-w-0 {class}",
            span {
                class: "pointer-events-none absolute inset-y-0 left-3.5 z-10 flex items-center text-text-muted",
                IconSearch {}
            }
            SignalInput {
                value,
                placeholder,
                class: "h-9 pl-10",
            }
        }
    }
}
