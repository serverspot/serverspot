use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill, StubPage};
use crate::components::ui::*;

#[component]
pub fn SupportTickets() -> Element {
    rsx! {
        PageHeader {
            title: "Tickets",
            subtitle: "Staff queues, custom fields, and AI-assisted replies for your players.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "View SLA"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Open", value: "12", accent: "#f0a35e" }
            StatPill { label: "Waiting on staff", value: "3", accent: "#fb7185" }
            StatPill { label: "Resolved today", value: "27", accent: "#3ecf8e" }
            StatPill { label: "Avg. reply", value: "14m", accent: "#5b9dff" }
        }

        DataPanel {
            title: "Open tickets",
            RowItem { title: "#1842 · Payment not received", meta: "Store · High priority", trailing: "11m" }
            RowItem { title: "#1839 · Can't join lobby", meta: "Gameplay · Normal", trailing: "34m" }
            RowItem { title: "#1833 · Rank missing perks", meta: "Store · Normal", trailing: "1h" }
            RowItem { title: "#1828 · Ban appeal", meta: "Moderation · Low", trailing: "3h" }
        }
    }
}

#[component]
pub fn SupportDepartments() -> Element {
    rsx! {
        StubPage {
            title: "Departments",
            subtitle: "Route tickets to the right staff teams with custom fields.",
            hint: "Department routing and form fields will be managed here.",
        }
    }
}

#[component]
pub fn SupportAutomation() -> Element {
    rsx! {
        PageHeader {
            title: "Automation",
            subtitle: "AI replies, notifications, and idle ticket policies.",
        }
        DataPanel {
            title: "Rules",
            RowItem { title: "AI first reply", meta: "Uses help center articles", trailing: "On" }
            RowItem { title: "Discord notifications", meta: "#support-alerts channel", trailing: "On" }
            RowItem { title: "Auto-close idle tickets", meta: "After 7 days without reply", trailing: "Off" }
            div {
                class: "pt-3",
                Button {
                    size: ButtonSize::Sm,
                    IconAi {}
                    "Configure AI"
                }
            }
        }
    }
}
