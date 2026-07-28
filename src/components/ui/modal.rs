use dioxus::prelude::*;

use super::icons::IconClose;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Modal(
    open: Signal<bool>,
    #[props(into)] title: String,
    #[props(default, into)] description: String,
    #[props(default)] size: ModalSize,
    children: Element,
    #[props(default)] footer: Option<Element>,
) -> Element {
    if !open() {
        return rsx! {};
    }

    let mut open_backdrop = open;
    let mut open_close = open;
    let mut open_keys = open;

    let panel_class = match size {
        ModalSize::Md => "ui-modal-panel",
        ModalSize::Lg => "ui-modal-panel ui-modal-panel-lg",
    };

    rsx! {
        div {
            class: "ui-modal-root",
            role: "presentation",
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    open_keys.set(false);
                }
            },

            button {
                class: "ui-modal-backdrop",
                r#type: "button",
                aria_label: "Close dialog",
                onclick: move |_| open_backdrop.set(false),
            }

            div {
                class: "{panel_class}",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "ui-modal-title",
                tabindex: "0",

                div {
                    class: "flex items-start justify-between gap-3 px-5 pt-5 sm:px-6 sm:pt-6",
                    div {
                        class: "min-w-0 pr-2",
                        h2 {
                            id: "ui-modal-title",
                            class: "text-xl font-semibold tracking-tight text-text",
                            "{title}"
                        }
                        if !description.is_empty() {
                            p {
                                class: "mt-1.5 max-w-xl text-sm leading-relaxed text-text-muted",
                                "{description}"
                            }
                        }
                    }
                    button {
                        class: "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 shrink-0 cursor-pointer items-center justify-center p-0 text-text-muted",
                        r#type: "button",
                        aria_label: "Close",
                        onclick: move |_| open_close.set(false),
                        IconClose {}
                    }
                }

                div {
                    class: "px-5 py-5 sm:px-6",
                    {children}
                }

                if let Some(footer) = footer {
                    div {
                        class: "flex flex-wrap items-center justify-end gap-2 px-5 pb-5 sm:px-6 sm:pb-6",
                        {footer}
                    }
                }
            }
        }
    }
}
