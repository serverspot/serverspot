use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill,
    StatusChip,
};
use crate::components::ui::*;
#[component]
pub fn SupportTickets() -> Element {
    rsx! {
        PageHeader {
            title: "Tickets",
            subtitle: "Customer support management with queues, notes, and history.",
            Button { variant: ButtonVariant::Secondary, "View departments" }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Open", value: "12", accent: "#f0a35e" }
            StatPill { label: "Pending", value: "5", accent: "#f5c14a" }
            StatPill { label: "Resolved", value: "27", accent: "#3ecf8e" }
            StatPill { label: "Closed", value: "140", accent: "#858899" }
        }
        div { class: "mb-4 flex flex-wrap gap-2",
            StatusChip { label: "Open", tone: "#f0a35e" }
            StatusChip { label: "Pending", tone: "#f5c14a" }
            StatusChip { label: "Resolved", tone: "#3ecf8e" }
            StatusChip { label: "Closed", tone: "#858899" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Open tickets",
                RowItem {
                    title: "#1842 · Payment not received",
                    meta: "Store · High priority · Unassigned",
                    trailing: "11m",
                }
                RowItem {
                    title: "#1839 · Can't join lobby",
                    meta: "Gameplay · Normal · Assigned Mira",
                    trailing: "34m",
                }
                RowItem {
                    title: "#1833 · Rank missing perks",
                    meta: "Store · Normal · Internal note added",
                    trailing: "1h",
                }
                RowItem {
                    title: "#1828 · Ban appeal",
                    meta: "Moderation · Low · History: 2 prior",
                    trailing: "3h",
                }
            }
            DataPanel { title: "Ticket features",
                FeatureBullets {
                    FeatureBullet { text: "Create tickets and categories" }
                    FeatureBullet { text: "Priority levels" }
                    FeatureBullet { text: "Staff assignment" }
                    FeatureBullet { text: "Internal notes" }
                    FeatureBullet { text: "Ticket history" }
                }
                div { class: "mt-4 border-t border-border-subtle pt-4",
                    p { class: "mb-2 text-xs font-medium uppercase tracking-wide text-text-muted",
                        "Staff tools"
                    }
                    FeatureBullets {
                        FeatureBullet { text: "Assign tickets and reply to users" }
                        FeatureBullet { text: "Add internal notes" }
                        FeatureBullet { text: "Manage departments" }
                        FeatureBullet { text: "View user history" }
                    }
                }
            }
        }
    }
}
#[component]
pub fn SupportHelpCentre() -> Element {
    rsx! {
        PageHeader {
            title: "Help centre",
            subtitle: "Knowledge base and documentation so players can help themselves.",
            Button {
                IconPlus {}
                "New article"
            }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Articles", value: "32", accent: "#87d1fe" }
            StatPill { label: "Categories", value: "8", accent: "#5b9dff" }
            StatPill { label: "FAQs", value: "14", accent: "#3ecf8e" }
            StatPill { label: "Feedback", value: "91%", accent: "#f5c14a" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Help centre features",
                FeatureBullets {
                    FeatureBullet { text: "Articles and categories" }
                    FeatureBullet { text: "Search" }
                    FeatureBullet { text: "FAQs" }
                    FeatureBullet { text: "Markdown support" }
                    FeatureBullet { text: "Featured articles" }
                    FeatureBullet { text: "Article feedback" }
                }
            }
            DataPanel { title: "Popular content",
                RowItem {
                    title: "How to claim vote rewards",
                    meta: "Guides · Featured",
                    trailing: "2.1k views",
                }
                RowItem {
                    title: "Server rules",
                    meta: "Rules · Pinned",
                    trailing: "1.4k views",
                }
                RowItem {
                    title: "Connecting with Bedrock",
                    meta: "Tutorials",
                    trailing: "980 views",
                }
                RowItem {
                    title: "Store refund policy",
                    meta: "Documentation",
                    trailing: "640 views",
                }
            }
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
        DataPanel { title: "Rules",
            SettingRow {
                title: "AI first reply",
                description: "Draft replies from help centre articles.",
                enabled: true,
            }
            SettingRow {
                title: "Discord notifications",
                description: "Post new tickets to #support-alerts.",
                enabled: true,
            }
            SettingRow {
                title: "Auto-close idle tickets",
                description: "Close tickets after 7 days without a reply.",
                enabled: false,
            }
            div { class: "pt-3",
                Button { size: ButtonSize::Sm,
                    IconAi {}
                    "Configure AI"
                }
            }
        }
    }
}
