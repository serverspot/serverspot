use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
use crate::components::ui::*;
use crate::router::Route;

#[derive(Clone, PartialEq)]
struct FeatureModule {
    title: &'static str,
    blurb: &'static str,
    accent: &'static str,
    route: Route,
}

#[component]
pub fn Dashboard() -> Element {
    let modules = [
        FeatureModule {
            title: "Web Store",
            blurb: "Sell ranks, crates, and packages with coupons, gifts, and stock controls.",
            accent: "#3ecf8e",
            route: Route::StoreProducts {},
        },
        FeatureModule {
            title: "Forum",
            blurb: "Categories, roles, awards, and tags that keep your community talking.",
            accent: "#5b9dff",
            route: Route::ForumCategories {},
        },
        FeatureModule {
            title: "Support Tickets",
            blurb: "Staff queues, custom fields, and AI-assisted replies in one inbox.",
            accent: "#f0a35e",
            route: Route::SupportTickets {},
        },
        FeatureModule {
            title: "Help Center",
            blurb: "Build FAQs, rules, and guides so players find answers themselves.",
            accent: "#87d1fe",
            route: Route::ContentHelp {},
        },
        FeatureModule {
            title: "Blog & News",
            blurb: "Publish updates and events, then keep the conversation going in comments.",
            accent: "#f071a5",
            route: Route::ContentBlog {},
        },
        FeatureModule {
            title: "Discord Sync",
            blurb: "Link accounts, grant donor roles, and announce purchases automatically.",
            accent: "#7b8cff",
            route: Route::SettingsIntegrations {},
        },
        FeatureModule {
            title: "Leaderboards",
            blurb: "Show player rankings pulled from your game database in real time.",
            accent: "#5eead4",
            route: Route::CommunityLeaderboards {},
        },
        FeatureModule {
            title: "Vote Rewards",
            blurb: "Share vote links and automatically reward players with in-game items.",
            accent: "#fbbf24",
            route: Route::CommunityVotes {},
        },
        FeatureModule {
            title: "Player Profiles",
            blurb: "Public profiles with stats, badges, and linked social accounts.",
            accent: "#69bdf2",
            route: Route::CommunityPlayers {},
        },
        FeatureModule {
            title: "Staff Applications",
            blurb: "Collect and review applications with custom fields and workflows.",
            accent: "#fb7185",
            route: Route::CommunityApplications {},
        },
        FeatureModule {
            title: "Analytics",
            blurb: "Track revenue, tickets, and engagement across your whole website.",
            accent: "#38bdf8",
            route: Route::AnalyticsOverview {},
        },
        FeatureModule {
            title: "Localization",
            blurb: "Serve players worldwide with multiple languages and currencies.",
            accent: "#34d399",
            route: Route::SettingsGeneral {},
        },
    ];

    rsx! {
        PageHeader {
            title: "Overview",
            subtitle: "Everything you need to run your game server website — store, community, support, and more.",
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-10 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Revenue", value: "£4,281", accent: "#3ecf8e" }
            StatPill { label: "Open tickets", value: "12", accent: "#f0a35e" }
            StatPill { label: "Players", value: "1,842", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
        }

        section {
            class: "grid grid-cols-1 sm:grid-cols-2 sm:gap-x-12",
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
    let navigator = use_navigator();
    let dest = module.route;

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
            class: "module-index-row group",
            style: "--row-accent: {module.accent};",
            onclick: move |_| {
                navigator.push(dest);
            },
            div {
                class: "module-index-icon rounded-squircle-sm",
                style: "background: {module.accent};",
                {icon}
            }
            div {
                class: "min-w-0 flex-1",
                p { class: "module-index-title", "{module.title}" }
                p { class: "module-index-blurb", "{module.blurb}" }
            }
        }
    }
}
