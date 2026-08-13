use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::ui::*;
use crate::i18n::t_key;

/// A single inbox row in the header notifications dropdown.
#[derive(Clone, PartialEq, Eq)]
pub struct AppNotification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub when: String,
    pub read: bool,
}

impl AppNotification {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        when: impl Into<String>,
        read: bool,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            when: when.into(),
            read,
        }
    }
}

fn default_notifications() -> Vec<AppNotification> {
    vec![
        AppNotification::new(
            "n1",
            "Store payout cleared",
            "Stripe settled £1,094 for this week’s orders.",
            "2m",
            false,
        ),
        AppNotification::new(
            "n2",
            "High priority ticket",
            "NovaCraft opened a ban appeal that needs review.",
            "18m",
            false,
        ),
        AppNotification::new(
            "n3",
            "Forum report queue",
            "3 reports are waiting in Moderation.",
            "1h",
            false,
        ),
        AppNotification::new(
            "n4",
            "Backup finished",
            "Nightly site backup completed successfully.",
            "Yesterday",
            true,
        ),
    ]
}

/// Copyable handle for the header notifications inbox.
#[derive(Clone, Copy)]
pub struct Notifications {
    open: Signal<bool>,
    items: Signal<Vec<AppNotification>>,
}

impl Notifications {
    pub fn from_signals(open: Signal<bool>, items: Signal<Vec<AppNotification>>) -> Self {
        Self { open, items }
    }

    pub fn is_open(self) -> bool {
        (self.open)()
    }

    pub fn unread_count(self) -> usize {
        self.items
            .read()
            .iter()
            .filter(|item| !item.read)
            .count()
    }

    pub fn toggle(self) {
        let next = !(self.open)();
        self.open.clone().set(next);
    }

    pub fn show(self) {
        self.open.clone().set(true);
    }

    pub fn hide(self) {
        self.open.clone().set(false);
    }

    pub fn set_items(self, items: Vec<AppNotification>) {
        self.items.clone().set(items);
    }

    pub fn push(self, item: AppNotification) {
        self.items.clone().write().insert(0, item);
    }

    pub fn mark_all_read(self) {
        for item in self.items.clone().write().iter_mut() {
            item.read = true;
        }
    }

    pub fn mark_read(self, id: &str) {
        if let Some(item) = self
            .items
            .clone()
            .write()
            .iter_mut()
            .find(|item| item.id == id)
        {
            item.read = true;
        }
    }
}

pub fn use_notifications() -> Notifications {
    use_context::<Notifications>()
}

fn next_dropdown_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

fn use_notifications_dismiss(mut open: Signal<bool>, dropdown_id: u64) {
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
                        const root = document.querySelector(`[data-shell-notifications="${id}"]`);
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
pub fn NotificationsMenu() -> Element {
    let _lang = i18n();
    let notifications = use_notifications();
    let open = notifications.open;
    let items_signal = notifications.items;
    let dropdown_id = use_hook(next_dropdown_id);
    use_notifications_dismiss(open, dropdown_id);

    let unread = notifications.unread_count();
    let open_now = open();
    let items = items_signal();
    let badge = if unread > 9 {
        String::from("9+")
    } else {
        unread.to_string()
    };
    let aria = if unread == 0 {
        t_key("shell-notifications-aria")
    } else {
        t!("shell-notifications-aria-count", count: unread)
    };
    let subtitle = if unread == 0 {
        t_key("shell-notifications-all-caught-up")
    } else {
        t!("shell-notifications-unread", count: unread)
    };
    let panel_aria = t_key("shell-notifications-title");

    rsx! {
        div {
            class: "shell-notifications",
            "data-shell-notifications": "{dropdown_id}",
            button {
                r#type: "button",
                class: if open_now {
                    "ui-btn ui-squircle ui-btn-ghost relative inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text"
                } else {
                    "ui-btn ui-squircle ui-btn-ghost relative inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted"
                },
                aria_label: "{aria}",
                aria_expanded: open_now,
                aria_haspopup: "true",
                onclick: move |_| notifications.toggle(),
                IconBell {}
                if unread > 0 {
                    span { class: "shell-notifications-badge", "{badge}" }
                }
            }

            if open_now {
                div {
                    class: "shell-notifications-panel",
                    role: "menu",
                    aria_label: "{panel_aria}",
                    div { class: "shell-notifications-head",
                        div {
                            p { class: "shell-notifications-title", { t!("shell-notifications-title") } }
                            p { class: "shell-notifications-sub", "{subtitle}" }
                        }
                        if unread > 0 {
                            button {
                                r#type: "button",
                                class: "shell-notifications-mark",
                                onclick: move |_| notifications.mark_all_read(),
                                { t!("shell-notifications-mark-read") }
                            }
                        }
                    }
                    div { class: "shell-notifications-list",
                        if items.is_empty() {
                            p { class: "shell-notifications-empty", { t!("shell-notifications-empty") } }
                        } else {
                            for item in items {
                                {
                                    let id = item.id.clone();
                                    let unread_row = !item.read;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if unread_row {
                                                "shell-notifications-item is-unread"
                                            } else {
                                                "shell-notifications-item"
                                            },
                                            role: "menuitem",
                                            onclick: move |_| {
                                                notifications.mark_read(&id);
                                            },
                                            span { class: "shell-notifications-item-dot", aria_hidden: "true" }
                                            span { class: "min-w-0 flex-1",
                                                span { class: "shell-notifications-item-title", "{item.title}" }
                                                span { class: "shell-notifications-item-body", "{item.body}" }
                                            }
                                            span { class: "shell-notifications-item-when", "{item.when}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn placeholder_notifications() -> Vec<AppNotification> {
    default_notifications()
}
