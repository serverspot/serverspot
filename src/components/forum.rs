use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill, StubPage};
use crate::components::ui::*;

#[component]
pub fn ForumCategories() -> Element {
    rsx! {
        PageHeader {
            title: "Categories",
            subtitle: "Organize discussion boards, roles, and visibility for your forum.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "New category"
                }
            },
        }

        section {
            class: "mb-8 grid grid-cols-2 gap-3 md:grid-cols-4",
            StatPill { label: "Threads", value: "1,204", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
            StatPill { label: "Members", value: "3,481", accent: "#3ecf8e" }
            StatPill { label: "Awards given", value: "128", accent: "#fbbf24" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Categories",
                RowItem { title: "Announcements", meta: "Pinned · 18 threads", trailing: "Staff" }
                RowItem { title: "Survival", meta: "General talk · 412 threads", trailing: "Public" }
                RowItem { title: "Suggestions", meta: "Player ideas · 96 threads", trailing: "Public" }
                RowItem { title: "Off-topic", meta: "Community lounge · 220 threads", trailing: "Public" }
            }
            DataPanel {
                title: "Recent threads",
                RowItem { title: "Season 4 spawn redesign", meta: "Survival · 24 replies", trailing: "2m" }
                RowItem { title: "Rank perks feedback", meta: "Suggestions · 11 replies", trailing: "18m" }
                RowItem { title: "Patch notes 1.21.4", meta: "Announcements · 40 replies", trailing: "1h" }
                RowItem { title: "Looking for builders", meta: "Off-topic · 7 replies", trailing: "3h" }
            }
        }
    }
}

#[component]
pub fn ForumThreads() -> Element {
    rsx! {
        StubPage {
            title: "Threads",
            subtitle: "Moderate discussions, pin announcements, and manage replies.",
            hint: "Thread moderation tools and filters will land here.",
        }
    }
}

#[component]
pub fn ForumRoles() -> Element {
    rsx! {
        StubPage {
            title: "Roles",
            subtitle: "Control who can post, react, and moderate each category.",
            hint: "Forum role permissions will be configured here.",
        }
    }
}

#[component]
pub fn ForumAwards() -> Element {
    rsx! {
        StubPage {
            title: "Awards",
            subtitle: "Create badges and reputation awards for helpful members.",
            hint: "Award definitions and grant history will appear here.",
        }
    }
}
