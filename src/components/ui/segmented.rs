use dioxus::prelude::*;

#[component]
pub fn SegmentedControl(
    options: Vec<String>,
    selected: String,
    onselect: EventHandler<String>,
    #[props(default, into)] class: String,
) -> Element {
    rsx! {
        div {
            class: "ui-chip rounded-squircle inline-flex items-center gap-1 p-1 {class}",
            for option in options {
                {
                    let is_active = option == selected;
                    let option_for_click = option.clone();
                    let label = option.clone();
                    let option_class = if is_active {
                        "ui-btn ui-btn-secondary ui-segment-option ui-segment-option-active rounded-squircle-sm px-4 py-1.5 text-sm font-semibold cursor-pointer"
                    } else {
                        "ui-segment-option rounded-squircle-sm px-4 py-1.5 text-sm font-medium text-text-secondary cursor-pointer bg-transparent border-none"
                    };
                    rsx! {
                        button {
                            key: "{label}",
                            class: "{option_class}",
                            onclick: move |_| onselect.call(option_for_click.clone()),
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
