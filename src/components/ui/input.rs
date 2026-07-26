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
            class: "relative flex min-w-0 {class}",
            span {
                class: "pointer-events-none absolute inset-y-0 left-3.5 z-10 flex items-center text-text-muted",
                IconSearch {}
            }
            Input {
                value,
                placeholder,
                class: "h-9 pl-10",
                oninput,
            }
        }
    }
}
