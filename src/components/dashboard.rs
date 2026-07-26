use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
use crate::components::ui::*;

#[derive(Clone, PartialEq)]
struct FeatureModule {
    title: &'static str,
    blurb: &'static str,
    accent: &'static str,
}

#[component]
pub fn Dashboard() -> Element {
    let modules = [
        FeatureModule {
            title: "Web Store",
            blurb: "Sell ranks, crates, and packages with coupons, gifts, and stock controls.",
            accent: "#3ecf8e",
        },
        FeatureModule {
            title: "Forum",
            blurb: "Categories, roles, awards, and tags that keep your community talking.",
            accent: "#5b9dff",
        },
        FeatureModule {
            title: "Support Tickets",
            blurb: "Staff queues, custom fields, and AI-assisted replies in one inbox.",
            accent: "#f0a35e",
        },
        FeatureModule {
            title: "Help Center",
            blurb: "Build FAQs, rules, and guides so players find answers themselves.",
            accent: "#b8b0ff",
        },
        FeatureModule {
            title: "Blog & News",
            blurb: "Publish updates and events, then keep the conversation going in comments.",
            accent: "#f071a5",
        },
        FeatureModule {
            title: "Discord Sync",
            blurb: "Link accounts, grant donor roles, and announce purchases automatically.",
            accent: "#7b8cff",
        },
        FeatureModule {
            title: "Leaderboards",
            blurb: "Show player rankings pulled from your game database in real time.",
            accent: "#5eead4",
        },
        FeatureModule {
            title: "Vote Rewards",
            blurb: "Share vote links and automatically reward players with in-game items.",
            accent: "#fbbf24",
        },
        FeatureModule {
            title: "Player Profiles",
            blurb: "Public profiles with stats, badges, and linked social accounts.",
            accent: "#a78bfa",
        },
        FeatureModule {
            title: "Staff Applications",
            blurb: "Collect and review applications with custom fields and workflows.",
            accent: "#fb7185",
        },
        FeatureModule {
            title: "Analytics",
            blurb: "Track revenue, tickets, and engagement across your whole website.",
            accent: "#38bdf8",
        },
        FeatureModule {
            title: "Localization",
            blurb: "Serve players worldwide with multiple languages and currencies.",
            accent: "#34d399",
        },
    ];

    rsx! {
        PageHeader {
            title: "Overview",
            subtitle: "Everything you need to run your game server website — store, community, support, and more.",
        }

        section {
            class: "mb-10 grid grid-cols-2 gap-3 md:grid-cols-4",
            StatPill { label: "Revenue", value: "£4,281", accent: "#3ecf8e" }
            StatPill { label: "Open tickets", value: "12", accent: "#f0a35e" }
            StatPill { label: "Players", value: "1,842", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
        }

        section {
            class: "grid gap-x-10 gap-y-8 sm:grid-cols-2",
            for module in modules {
                ModuleCard { module }
            }
        }
    }
}

#[component]
pub fn DashboardActivity() -> Element {
    rsx! {
        PageHeader {
            title: "Activity",
            subtitle: "Recent purchases, tickets, and community events across your site.",
        }
        DataPanel {
            title: "Latest events",
            RowItem { title: "Order #4821 completed", meta: "Store · VIP Rank", trailing: "2m" }
            RowItem { title: "Ticket #1842 opened", meta: "Support · Payment", trailing: "11m" }
            RowItem { title: "New forum reply", meta: "Season 4 spawn redesign", trailing: "18m" }
            RowItem { title: "Player NovaCraft voted", meta: "Community · Vote rewards", trailing: "25m" }
        }
    }
}

#[component]
fn ModuleCard(module: FeatureModule) -> Element {
    let icon = match module.title {
        "Web Store" => rsx! { IconStore {} },
        "Forum" => rsx! { IconForum {} },
        "Support Tickets" => rsx! { IconTicket {} },
        "Help Center" => rsx! { IconHelp {} },
        "Blog & News" => rsx! { IconNews {} },
        "Discord Sync" => rsx! { IconDiscord {} },
        "Leaderboards" => rsx! { IconAnalytics {} },
        "Vote Rewards" => rsx! { IconPackage {} },
        "Player Profiles" => rsx! { IconUsers {} },
        "Staff Applications" => rsx! { IconSupport {} },
        "Analytics" => rsx! { IconChart {} },
        _ => rsx! { IconGlobe {} },
    };

    rsx! {
        button {
            class: "ui-btn ui-btn-secondary ui-squircle group flex h-full w-full items-start gap-4 px-4 py-4 text-left font-normal",
            div {
                class: "flex h-10 w-10 shrink-0 items-center justify-center rounded-squircle-sm",
                style: "background: color-mix(in srgb, {module.accent} 16%, transparent); color: {module.accent};",
                {icon}
            }
            div {
                class: "min-w-0 pt-0.5",
                p { class: "text-[15px] font-semibold tracking-tight text-text", "{module.title}" }
                p { class: "mt-1 text-sm font-normal leading-relaxed text-text-muted", "{module.blurb}" }
            }
        }
    }
}
