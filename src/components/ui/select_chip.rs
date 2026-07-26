use dioxus::prelude::*;

use super::icons::IconChevronDown;

#[component]
pub fn SelectChip(
    #[props(into)] label: String,
    #[props(default, into)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "ui-chip rounded-squircle-sm inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-text-secondary hover:text-text transition-colors cursor-pointer {class}",
            onclick: move |evt| onclick.call(evt),
            span { "{label}" }
            IconChevronDown { class: "opacity-70" }
        }
    }
}
