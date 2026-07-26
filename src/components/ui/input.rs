use dioxus::prelude::*;

use super::icons::IconSearch;

#[component]
pub fn Input(
    #[props(default, into)] value: String,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
    #[props(default, into)] r#type: String,
    #[props(default)] oninput: EventHandler<FormEvent>,
) -> Element {
    let input_type = if r#type.is_empty() {
        "text".to_string()
    } else {
        r#type
    };

    rsx! {
        input {
            r#type: "{input_type}",
            class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none {class}",
            value,
            placeholder,
            oninput: move |evt| oninput.call(evt),
        }
    }
}

#[component]
pub fn SearchInput(
    #[props(default, into)] value: String,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
    #[props(default)] oninput: EventHandler<FormEvent>,
) -> Element {
    let placeholder = if placeholder.is_empty() {
        "Search...".to_string()
    } else {
        placeholder
    };

    rsx! {
        div {
            class: "relative {class}",
            span {
                class: "pointer-events-none absolute inset-y-0 left-3.5 z-10 flex items-center text-text-muted",
                IconSearch {}
            }
            Input {
                value,
                placeholder,
                class: "pl-10",
                oninput,
            }
        }
    }
}

#[component]
pub fn TextArea(
    #[props(default, into)] value: String,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
    #[props(default)] rows: u32,
    #[props(default)] oninput: EventHandler<FormEvent>,
) -> Element {
    let rows = if rows == 0 { 3 } else { rows };

    rsx! {
        textarea {
            class: "ui-input rounded-squircle-lg w-full px-4 py-3 text-sm outline-none resize-y {class}",
            value,
            placeholder,
            rows,
            oninput: move |evt| oninput.call(evt),
        }
    }
}
