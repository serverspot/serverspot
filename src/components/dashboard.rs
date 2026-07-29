use dioxus::prelude::*;
use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
use crate::components::ui::*;
use crate::router::Route;
#[derive(Clone, Copy, PartialEq)]
struct FeatureModule {
    title: &'static str,
    blurb: &'static str,
    accent: &'static str,
    route: Route,
}
const MODULES: &[FeatureModule] = &[
    FeatureModule {
        title: "Store",
        blurb: "Sell ranks, crates, and packages with coupons, gifts, and stock controls.",
        accent: "#3ecf8e",
        route: Route::StoreOverview {},
    },
    FeatureModule {
        title: "Forum",
        blurb: "Categories, posts, reactions, and moderation tools for your community.",
        accent: "#5b9dff",
        route: Route::ForumOverview {},
    },
    FeatureModule {
        title: "Support",
        blurb: "Staff queues, priorities, help center, and ticket history.",
        accent: "#f0a35e",
        route: Route::SupportOverview {},
    },
    FeatureModule {
        title: "Blog",
        blurb: "Publish updates and events with drafts, tags, and scheduling.",
        accent: "#f071a5",
        route: Route::ContentOverview {},
    },
    FeatureModule {
        title: "Players",
        blurb: "Gaming profiles with stats, badges, and linked accounts.",
        accent: "#69bdf2",
        route: Route::PlayersOverview {},
    },
    FeatureModule {
        title: "Leaderboards",
        blurb: "Show player rankings pulled from APIs, plugins, or manual input.",
        accent: "#5eead4",
        route: Route::LeaderboardsOverview {},
    },
    FeatureModule {
        title: "Vote rewards",
        blurb: "Share vote links and automatically reward players with in-game items.",
        accent: "#fbbf24",
        route: Route::VotesOverview {},
    },
    FeatureModule {
        title: "Applications",
        blurb: "Collect and review staff applications with custom fields and workflows.",
        accent: "#fb7185",
        route: Route::ApplicationsOverview {},
    },
    FeatureModule {
        title: "Analytics",
        blurb: "Track revenue, tickets, and engagement across your whole website.",
        accent: "#38bdf8",
        route: Route::AnalyticsOverview {},
    },
];
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        PageHeader {
            title: "Overview",
            subtitle: "Everything you need to run your game server website — store, community, support, and more.",
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-10 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Revenue", value: "£4,281", accent: "#3ecf8e" }
            StatPill { label: "Open tickets", value: "12", accent: "#f0a35e" }
            StatPill { label: "Players", value: "1,842", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
        }
        section { class: "grid grid-cols-1 sm:grid-cols-2 sm:gap-x-12",
            for module in MODULES.iter().copied() {
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
        DataPanel { title: "Latest events",
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
                title: "New forum reply",
                meta: "Season 4 spawn redesign",
                trailing: "18m",
            }
            RowItem {
                title: "Player NovaCraft voted",
                meta: "Community · Vote rewards",
                trailing: "25m",
            }
        }
    }
}
#[component]
fn ModuleCard(module: FeatureModule) -> Element {
    let navigator = use_navigator();
    let dest = module.route;
    let icon = match module.title {
        "Store" => {
            rsx! {
                IconStore {}
            }
        }
        "Forum" => {
            rsx! {
                IconForum {}
            }
        }
        "Support" => {
            rsx! {
                IconTicket {}
            }
        }
        "Blog" => {
            rsx! {
                IconNews {}
            }
        }
        "Players" => {
            rsx! {
                IconUsers {}
            }
        }
        "Leaderboards" | "Analytics" => {
            rsx! {
                IconChart {}
            }
        }
        "Vote rewards" => {
            rsx! {
                IconGift {}
            }
        }
        "Applications" => {
            rsx! {
                IconSupport {}
            }
        }
        _ => {
            rsx! {
                IconGlobe {}
            }
        }
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
            div { class: "min-w-0 flex-1",
                p { class: "module-index-title", "{module.title}" }
                p { class: "module-index-blurb", "{module.blurb}" }
            }
        }
    }
}
