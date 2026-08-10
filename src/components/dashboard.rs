use dioxus::prelude::*;

use crate::components::page::{PageHeader, RowItem};
use crate::components::ui::*;
use crate::router::Route;
use crate::user::CurrentUser;

#[derive(Clone, PartialEq)]
struct AttentionItem {
    title: &'static str,
    meta: &'static str,
    count: &'static str,
    route: Route,
}

const ATTENTION: &[AttentionItem] = &[
    AttentionItem {
        title: "Open support tickets",
        meta: "3 waiting over 4 hours",
        count: "12",
        route: Route::SupportTickets {},
    },
    AttentionItem {
        title: "Forum reports",
        meta: "Spam reply chain needs a look",
        count: "3",
        route: Route::ForumModeration {},
    },
    AttentionItem {
        title: "Staff applications",
        meta: "Builder + helper forms",
        count: "2",
        route: Route::CommunityApplications {},
    },
    AttentionItem {
        title: "Pending payouts",
        meta: "Store gateway settlement",
        count: "1",
        route: Route::StoreOrders {},
    },
];

const WEEK_SALES: &[u8] = &[32, 48, 28, 61, 44, 72, 55];
const WEEK_LABELS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[component]
pub fn Dashboard() -> Element {
    let current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user();
    let navigator = use_navigator();
    let greeting = format!("Hey, {}", first_name(&user.name));
    let max_bar = *WEEK_SALES.iter().max().unwrap_or(&1);

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Live overview" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "{greeting}"
                }
            }
            div { class: "flex flex-wrap gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::DashboardActivity {});
                    },
                    "Activity"
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::StoreProducts {});
                    },
                    "New product"
                }
            }
        }

        section { class: "dash-panel mb-6 p-5 sm:p-6",
            div { class: "flex flex-wrap items-end justify-between gap-6",
                div {
                    p { class: "text-sm text-text-muted", "Players online" }
                    p { class: "mt-2 text-5xl font-semibold tabular-nums tracking-tight text-accent sm:text-6xl",
                        "184"
                    }
                    p { class: "mt-2 text-sm text-text-secondary",
                        "Peak today 412 · Survival + Skyblock"
                    }
                }
                div { class: "flex flex-wrap gap-x-8 gap-y-3 text-sm text-text-muted",
                    span { class: "tabular-nums",
                        span { class: "text-text", "£4,281" }
                        " revenue"
                    }
                    span { class: "tabular-nums",
                        span { class: "text-text", "12" }
                        " tickets"
                    }
                    span { class: "tabular-nums",
                        span { class: "text-text", "52" }
                        " posts"
                    }
                }
            }
        }

        div { class: "motion-cascade mb-6 grid gap-6 lg:grid-cols-[minmax(0,1.4fr)_minmax(0,1fr)]",

            section { class: "motion-cascade motion-cascade-tight dash-panel p-5 sm:px-5 sm:py-4",
                div { class: "mb-1 flex items-baseline justify-between gap-3",
                    h2 { class: "text-sm font-semibold text-text", "Needs attention" }
                    button {
                        class: "text-xs text-text-muted transition-colors hover:text-text",
                        onclick: move |_| {
                            navigator.push(Route::SupportTickets {});
                        },
                        "View all"
                    }
                }
                for item in ATTENTION.iter().cloned() {
                    AttentionRow { item }
                }
            }

            section { class: "dash-panel flex flex-col p-5",
                div { class: "mb-4 flex items-baseline justify-between gap-3",
                    h2 { class: "text-sm font-semibold text-text", "Store this week" }
                    p { class: "text-xs text-text-muted tabular-nums", "£812 · 7 days" }
                }
                div { class: "motion-cascade motion-cascade-tight dash-bars",
                    for (i, value) in WEEK_SALES.iter().enumerate() {
                        {
                            let height = ((*value as f32 / max_bar as f32) * 100.0).round() as u32;
                            let today = i + 1 == WEEK_SALES.len();
                            rsx! {
                                div {
                                    class: if today { "dash-bar is-today" } else { "dash-bar" },
                                    style: "height: {height}%;",
                                    title: "{WEEK_LABELS[i]}",
                                }
                            }
                        }
                    }
                }
                div { class: "mt-3 flex justify-between text-[11px] text-text-muted",
                    for label in WEEK_LABELS.iter() {
                        span { class: "w-full text-center", "{label}" }
                    }
                }
            }
        }

        section { class: "dash-panel overflow-hidden",
            div { class: "flex items-center justify-between border-b border-border-subtle px-5 py-3",
                h2 { class: "text-sm font-semibold text-text", "Just now" }
                button {
                    class: "text-xs text-text-muted transition-colors hover:text-text",
                    onclick: move |_| {
                        navigator.push(Route::DashboardActivity {});
                    },
                    "Full feed"
                }
            }
            div { class: "motion-cascade motion-cascade-tight px-5",
                RowItem {
                    title: "Order #4821 completed",
                    meta: "Store · VIP Rank",
                    trailing: "2m",
                }
                RowItem {
                    title: "Ticket #1842 opened",
                    meta: "Support · Payment",
                    trailing: "11m",
                }
                RowItem {
                    title: "NovaCraft voted",
                    meta: "Vote rewards · 7-day streak",
                    trailing: "25m",
                }
            }
        }
    }
}

#[component]
fn AttentionRow(item: AttentionItem) -> Element {
    let navigator = use_navigator();
    let dest = item.route;

    rsx! {
        button {
            class: "attention-row",
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            div { class: "min-w-0",
                p { class: "attention-title text-sm font-medium text-text-secondary",
                    "{item.title}"
                }
                p { class: "mt-0.5 text-xs text-text-muted", "{item.meta}" }
            }
            span { class: "shrink-0 text-sm font-semibold tabular-nums text-accent",
                "{item.count}"
            }
        }
    }
}

fn first_name(name: &str) -> &str {
    name.split_whitespace().next().unwrap_or(name)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActivityCategory {
    Store,
    Support,
    Community,
    Votes,
    Applications,
}

impl ActivityCategory {
    fn label(self) -> &'static str {
        match self {
            ActivityCategory::Store => "Store",
            ActivityCategory::Support => "Support",
            ActivityCategory::Community => "Community",
            ActivityCategory::Votes => "Votes",
            ActivityCategory::Applications => "Applications",
        }
    }

    fn color(self) -> &'static str {
        match self {
            ActivityCategory::Store => "#3ecf8e",
            ActivityCategory::Support => "#f0a35e",
            ActivityCategory::Community => "#5b9dff",
            ActivityCategory::Votes => "#fbbf24",
            ActivityCategory::Applications => "#fb7185",
        }
    }
}

const CATEGORIES: &[ActivityCategory] = &[
    ActivityCategory::Store,
    ActivityCategory::Support,
    ActivityCategory::Community,
    ActivityCategory::Votes,
    ActivityCategory::Applications,
];

struct ActivityEvent {
    day: &'static str,
    time: &'static str,
    category: ActivityCategory,
    title: &'static str,
    meta: &'static str,
}

const ACTIVITY_EVENTS: &[ActivityEvent] = &[
    ActivityEvent {
        day: "Today",
        time: "14:02",
        category: ActivityCategory::Store,
        title: "Order #4821 completed",
        meta: "VIP Rank · £14.99",
    },
    ActivityEvent {
        day: "Today",
        time: "13:51",
        category: ActivityCategory::Support,
        title: "Ticket #1842 opened",
        meta: "Payment not received",
    },
    ActivityEvent {
        day: "Today",
        time: "13:38",
        category: ActivityCategory::Community,
        title: "New forum reply",
        meta: "Season 4 spawn redesign",
    },
    ActivityEvent {
        day: "Today",
        time: "13:15",
        category: ActivityCategory::Votes,
        title: "NovaCraft voted",
        meta: "7-day streak reward claimed",
    },
    ActivityEvent {
        day: "Today",
        time: "12:40",
        category: ActivityCategory::Applications,
        title: "Application submitted",
        meta: "Helper role · QuietLeaf",
    },
    ActivityEvent {
        day: "Today",
        time: "11:58",
        category: ActivityCategory::Store,
        title: "Coupon SUMMER20 used",
        meta: "Crate Key Pack",
    },
    ActivityEvent {
        day: "Yesterday",
        time: "21:12",
        category: ActivityCategory::Support,
        title: "Ticket #1839 resolved",
        meta: "Can't join lobby · Assigned Mira",
    },
    ActivityEvent {
        day: "Yesterday",
        time: "19:47",
        category: ActivityCategory::Community,
        title: "Forum report filed",
        meta: "Spam reply chain flagged",
    },
    ActivityEvent {
        day: "Yesterday",
        time: "18:20",
        category: ActivityCategory::Store,
        title: "Order #4809 completed",
        meta: "Starter Kit · £4.99",
    },
    ActivityEvent {
        day: "Yesterday",
        time: "16:05",
        category: ActivityCategory::Votes,
        title: "AetherFox voted",
        meta: "Top voter this month · #1",
    },
    ActivityEvent {
        day: "Yesterday",
        time: "10:33",
        category: ActivityCategory::Applications,
        title: "Application reviewed",
        meta: "Builder role · ClayMage · Accepted",
    },
];

#[component]
pub fn DashboardActivity() -> Element {
    let mut filter = use_signal(|| Option::<ActivityCategory>::None);
    let active = filter();

    let mut day_order: Vec<&'static str> = Vec::new();
    for event in ACTIVITY_EVENTS {
        if !day_order.contains(&event.day) {
            day_order.push(event.day);
        }
    }

    let day_groups: Vec<(&'static str, Vec<&ActivityEvent>)> = day_order
        .into_iter()
        .filter_map(|day| {
            let events: Vec<&ActivityEvent> = ACTIVITY_EVENTS
                .iter()
                .filter(|event| {
                    event.day == day && active.map(|c| c == event.category).unwrap_or(true)
                })
                .collect();
            if events.is_empty() {
                None
            } else {
                Some((day, events))
            }
        })
        .collect();

    rsx! {
        PageHeader {
            title: "Activity",
            subtitle: "A live, chronological feed of purchases, tickets, and community events.",
        }

        div { class: "activity-toolbar",
            span { class: "signal-live-tag",
                span { class: "pulse-dot" }
                "Streaming"
            }
            div { class: "activity-filters",
                button {
                    class: if active.is_none() { "activity-chip is-active" } else { "activity-chip" },
                    onclick: move |_| filter.set(None),
                    "All"
                }
                for category in CATEGORIES.iter().copied() {
                    button {
                        class: if active == Some(category) { "activity-chip is-active" } else { "activity-chip" },
                        style: "--chip-color: {category.color()};",
                        onclick: move |_| filter.set(Some(category)),
                        span { class: "activity-chip-dot" }
                        "{category.label()}"
                    }
                }
            }
        }

        if day_groups.is_empty() {
            div { class: "activity-empty", "No activity for this filter yet." }
        } else {
            div { class: "activity-stream",
                for (day, events) in day_groups {
                    div {
                        p { class: "activity-day-label", "{day}" }
                        div { class: "motion-cascade motion-cascade-tight activity-day-body",
                            for event in events {
                                div { class: "activity-row",
                                    span {
                                        class: "activity-dot",
                                        style: "--dot-color: {event.category.color()};",
                                    }
                                    span { class: "activity-time", "{event.time}" }
                                    div { class: "activity-content",
                                        p { class: "activity-title", "{event.title}" }
                                        p { class: "activity-meta", "{event.meta}" }
                                    }
                                    span {
                                        class: "activity-tag",
                                        style: "--dot-color: {event.category.color()};",
                                        "{event.category.label()}"
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
