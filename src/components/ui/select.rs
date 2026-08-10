use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

fn next_dropdown_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

fn use_dropdown_dismiss(mut open: Signal<bool>, dropdown_id: u64) {
    use_effect(move || {
        if !open() {
            return;
        }

        spawn(async move {
            let mut eval = document::eval(
                r#"
                const id = await dioxus.recv();
                await new Promise((resolve) => {
                    const onKey = (event) => {
                        if (event.key === "Escape") {
                            cleanup();
                            resolve(true);
                        }
                    };
                    const onPointer = (event) => {
                        const root = document.querySelector(`[data-ui-dropdown="${id}"]`);
                        if (root && root.contains(event.target)) {
                            return;
                        }
                        cleanup();
                        resolve(true);
                    };
                    const cleanup = () => {
                        document.removeEventListener("keydown", onKey, true);
                        document.removeEventListener("pointerdown", onPointer, true);
                    };
                    requestAnimationFrame(() => {
                        document.addEventListener("keydown", onKey, true);
                        document.addEventListener("pointerdown", onPointer, true);
                    });
                });
                true
                "#,
            );

            let _ = eval.send(dropdown_id.to_string());
            if eval.join::<bool>().await.unwrap_or(false) {
                open.set(false);
            }
        });
    });
}

#[component]
pub fn SignalSelect(
    mut value: Signal<String>,
    options: Vec<SelectOption>,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
) -> Element {
    let mut open = use_signal(|| false);
    let dropdown_id = use_hook(next_dropdown_id);
    use_dropdown_dismiss(open, dropdown_id);

    let current = value();
    let selected_label = options
        .iter()
        .find(|option| option.value == current)
        .map(|option| option.label.as_str())
        .filter(|label| !label.is_empty());
    let show_placeholder = selected_label.is_none() && current.is_empty();
    let display: &str = selected_label.unwrap_or_else(|| {
        if !current.is_empty() {
            current.as_str()
        } else if placeholder.is_empty() {
            "Select…"
        } else {
            placeholder.as_str()
        }
    });
    let is_open = open();

    rsx! {
        div {
            class: if is_open { "ui-dropdown is-open {class}" } else { "ui-dropdown {class}" },
            "data-ui-dropdown": "{dropdown_id}",
            button {
                r#type: "button",
                class: if is_open { "ui-dropdown-trigger ui-squircle is-open" } else { "ui-dropdown-trigger ui-squircle" },
                "aria-haspopup": "listbox",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    let next = !*open.peek();
                    open.set(next);
                },
                span { class: if show_placeholder { "ui-dropdown-value is-placeholder" } else { "ui-dropdown-value" },
                    "{display}"
                }
                span { class: "ui-dropdown-caret", "aria-hidden": "true" }
            }
            if is_open {
                div { class: "ui-dropdown-menu ui-squircle", role: "listbox",
                    for option in options.iter() {
                        {
                            let selected = option.value == current;
                            let option_value = option.value.clone();
                            rsx! {
                                button {
                                    key: "{option.value}",
                                    r#type: "button",
                                    class: if selected { "ui-dropdown-option is-selected" } else { "ui-dropdown-option" },
                                    role: "option",
                                    "aria-selected": if selected { "true" } else { "false" },
                                    onclick: move |_| {
                                        value.set(option_value.clone());
                                        open.set(false);
                                    },
                                    "{option.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SignalMultiSelect(
    mut value: Signal<Vec<String>>,
    options: Vec<SelectOption>,
    #[props(default, into)] placeholder: String,
    #[props(default, into)] class: String,
) -> Element {
    let mut open = use_signal(|| false);
    let dropdown_id = use_hook(next_dropdown_id);
    use_dropdown_dismiss(open, dropdown_id);

    let selected = value();
    let selected_labels: Vec<String> = options
        .iter()
        .filter(|option| selected.iter().any(|item| item == &option.value))
        .map(|option| option.label.clone())
        .collect();
    let show_placeholder = selected_labels.is_empty();
    let display = if show_placeholder {
        if placeholder.is_empty() {
            String::from("Select…")
        } else {
            placeholder.clone()
        }
    } else {
        selected_labels.join(", ")
    };
    let is_open = open();

    rsx! {
        div {
            class: if is_open { "ui-dropdown is-open {class}" } else { "ui-dropdown {class}" },
            "data-ui-dropdown": "{dropdown_id}",
            button {
                r#type: "button",
                class: if is_open { "ui-dropdown-trigger ui-squircle is-open" } else { "ui-dropdown-trigger ui-squircle" },
                "aria-haspopup": "listbox",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    let next = !*open.peek();
                    open.set(next);
                },
                span { class: if show_placeholder { "ui-dropdown-value is-placeholder" } else { "ui-dropdown-value" },
                    "{display}"
                }
                span { class: "ui-dropdown-caret", "aria-hidden": "true" }
            }
            if is_open {
                div {
                    class: "ui-dropdown-menu ui-squircle",
                    role: "listbox",
                    "aria-multiselectable": "true",
                    for option in options.iter() {
                        {
                            let selected_now = selected.iter().any(|item| item == &option.value);
                            let option_value = option.value.clone();
                            rsx! {
                                button {
                                    key: "{option.value}",
                                    r#type: "button",
                                    class: if selected_now { "ui-dropdown-option is-selected" } else { "ui-dropdown-option" },
                                    role: "option",
                                    "aria-selected": if selected_now { "true" } else { "false" },
                                    onclick: move |_| {
                                        value.with_mut(|list| {
                                            if let Some(index) = list.iter().position(|item| item == &option_value) {
                                                list.remove(index);
                                            } else {
                                                list.push(option_value.clone());
                                            }
                                        });
                                    },
                                    span {
                                        class: if selected_now { "ui-dropdown-check is-on" } else { "ui-dropdown-check" },
                                        "aria-hidden": "true",
                                        if selected_now {
                                            "✓"
                                        } else {
                                            ""
                                        }
                                    }
                                    span { class: "ui-dropdown-option-label", "{option.label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
