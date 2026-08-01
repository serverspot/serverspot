use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, InfoCard, PageHeader, RowItem, StatPill, StatusChip,
};
use crate::components::ui::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FeatureOverviewKind {
    Store,
    Forum,
    Support,
    Content,
    Players,
    Leaderboards,
    Votes,
    Applications,
    Analytics,
}
#[derive(Clone, Copy)]
struct OverviewCopy {
    title: &'static str,
    subtitle: &'static str,
    about: &'static str,
    accent: &'static str,
    domain: &'static str,
    status: &'static str,
    stats: [(&'static str, &'static str); 4],
    highlights: [&'static str; 5],
    activity: [(&'static str, &'static str, &'static str); 4],
    next_steps: [(&'static str, &'static str); 3],
}
impl FeatureOverviewKind {
    const fn data(self) -> OverviewCopy {
        match self {
            Self::Store => {
                OverviewCopy {
                    title: "Store",
                    subtitle: "Sell ranks, crates, and packages with checkout, coupons, and stock controls.",
                    about: "The web store lets players browse packages, apply coupons, gift purchases, and receive rewards automatically across your connected game servers.",
                    accent: "#3ecf8e",
                    domain: "www.example.com/store",
                    status: "Live",
                    stats: [
                        ("Revenue", "£4,281"),
                        ("Orders", "96"),
                        ("Products", "28"),
                        ("Conversion", "3.8%"),
                    ],
                    highlights: [
                        "Product catalog with ranks, items, and bundles",
                        "Coupons, gifts, and creator codes",
                        "Order tracking and refunds",
                        "Storefront theme on your main website",
                        "Delivery to game servers and Discord roles",
                    ],
                    activity: [
                        ("Order #4821 · VIP Rank", "NovaCraft · Paid", "£29.99"),
                        ("Order #4818 · Crate Keys", "SkyBuilder · Paid", "£9.99"),
                        ("Coupon SUMMER20 used", "20% off lifetime ranks", "8 uses"),
                        ("Stock low · Cosmetics Pack", "4 left in inventory", "Alert"),
                    ],
                    next_steps: [
                        ("Products", "Add or edit packages for the shop"),
                        ("Orders", "Review payments and delivery status"),
                        ("Settings", "Configure store path and branding"),
                    ],
                }
            }
            Self::Forum => {
                OverviewCopy {
                    title: "Forum",
                    subtitle: "Community discussion with categories, moderation, and rich posts.",
                    about: "Forums give your players a place to share builds, suggestions, and announcements with markdown, reactions, mentions, and staff moderation tools.",
                    accent: "#5b9dff",
                    domain: "www.example.com/forum",
                    status: "Live",
                    stats: [
                        ("Threads", "1,204"),
                        ("Posts today", "52"),
                        ("Members", "3,481"),
                        ("Open reports", "3"),
                    ],
                    highlights: [
                        "Categories, posts, and comments",
                        "Markdown, reactions, and mentions",
                        "Attachments, tags, pinning, and locking",
                        "Reports and moderation workflows",
                        "Permission-aware category visibility",
                    ],
                    activity: [
                        ("Season 4 spawn redesign", "Survival · 24 replies", "2m"),
                        ("Rank perks feedback", "Suggestions · 11 replies", "18m"),
                        ("Spam report opened", "Suggestions · Review queue", "New"),
                        ("Patch notes 1.21.4", "Announcements · Pinned", "1h"),
                    ],
                    next_steps: [
                        ("Categories", "Organize boards and visibility"),
                        ("Posts", "Browse recent threads"),
                        ("Moderation", "Clear reports and bans"),
                    ],
                }
            }
            Self::Support => {
                OverviewCopy {
                    title: "Support",
                    subtitle: "Tickets, help centre articles, and staff automation in one portal.",
                    about: "Support combines a ticket inbox with a searchable help centre so players can get answers quickly while staff manage priorities, notes, and departments.",
                    accent: "#f0a35e",
                    domain: "www.example.com/support",
                    status: "Live",
                    stats: [
                        ("Open tickets", "12"),
                        ("Pending", "5"),
                        ("Articles", "32"),
                        ("Avg. reply", "14m"),
                    ],
                    highlights: [
                        "Ticket categories and priority levels",
                        "Staff assignment and internal notes",
                        "Help centre FAQs and guides",
                        "AI-assisted first replies",
                        "Department and SLA tooling",
                    ],
                    activity: [
                        (
                            "#1842 · Payment not received",
                            "Store · High priority",
                            "11m",
                        ),
                        (
                            "#1839 · Can't join lobby",
                            "Gameplay · Assigned Mira",
                            "34m",
                        ),
                        (
                            "Article updated · Vote rewards",
                            "Help centre · Featured",
                            "1h",
                        ),
                        ("AI first reply rule", "Automation · Enabled", "On"),
                    ],
                    next_steps: [
                        ("Tickets", "Work the staff queue"),
                        ("Help centre", "Publish guides and FAQs"),
                        ("Automation", "Tune AI and idle policies"),
                    ],
                }
            }
            Self::Content => {
                OverviewCopy {
                    title: "Blog",
                    subtitle: "Blog posts, news, drafts, and custom pages for your audience.",
                    about: "Blog publishing covers news, articles, and marketing pages with authors, approvals, featured placement, and scheduled publishing.",
                    accent: "#f071a5",
                    domain: "www.example.com/news",
                    status: "Live",
                    stats: [
                        ("Published", "48"),
                        ("Drafts", "6"),
                        ("Scheduled", "3"),
                        ("Authors", "5"),
                    ],
                    highlights: [
                        "Blog posts and news articles",
                        "Categories, tags, and markdown editor",
                        "Drafts and scheduled publishing",
                        "Author management and approvals",
                        "Homepage and featured placement",
                    ],
                    activity: [
                        ("Season 4 launch recap", "News · Published", "Today"),
                        ("Economy changes", "Blog · Draft", "Edit"),
                        ("Weekend crate event", "Scheduled Fri 18:00", "Queue"),
                        ("Homepage slider updated", "3 slides · Live", "2h"),
                    ],
                    next_steps: [
                        ("Posts", "Write or schedule a post"),
                        ("Pages", "Manage custom pages and widgets"),
                        ("Settings", "Configure blog path and branding"),
                    ],
                }
            }
            Self::Players => {
                OverviewCopy {
                    title: "Players",
                    subtitle: "Dedicated gaming profiles with stats, ranks, and linked accounts.",
                    about: "Player profiles surface playtime, achievements, ranks, and linked game identities across every server connected to the platform.",
                    accent: "#69bdf2",
                    domain: "www.example.com/players",
                    status: "Live",
                    stats: [
                        ("Players", "1,842"),
                        ("Linked", "1,204"),
                        ("Games", "3"),
                        ("Servers", "7"),
                    ],
                    highlights: [
                        "Linked game accounts",
                        "Statistics, playtime, and ranks",
                        "Achievements and badges",
                        "Leaderboard positions",
                        "Multi-game and multi-server support",
                    ],
                    activity: [
                        ("NovaCraft profile viewed", "VIP · Level 84", "2m"),
                        ("SkyBuilder linked Survival", "Verification code Q1M9", "14m"),
                        ("Badge awarded · Builder", "ClayMage", "1h"),
                        ("New player registered", "QuietLeaf", "3h"),
                    ],
                    next_steps: [
                        ("Profiles", "Browse and search players"),
                        ("Settings", "Configure players path and branding"),
                        ("Theme", "Customize profile presentation"),
                    ],
                }
            }
            Self::Leaderboards => {
                OverviewCopy {
                    title: "Leaderboards",
                    subtitle: "Rankings and statistics across games, servers, and time ranges.",
                    about: "Leaderboards pull stats from APIs, server plugins, or manual input so you can publish top players, economy boards, and seasonal contests.",
                    accent: "#5eead4",
                    domain: "www.example.com/leaderboards",
                    status: "Live",
                    stats: [
                        ("Boards", "4"),
                        ("Tracked stats", "18"),
                        ("Data sources", "3"),
                        ("Updates / hr", "12"),
                    ],
                    highlights: [
                        "Multiple boards and games",
                        "Custom statistics and history",
                        "Time-based rankings",
                        "API, plugin, and manual sources",
                        "Public ranking pages",
                    ],
                    activity: [
                        ("Top players refreshed", "Survival · Live feed", "1m"),
                        ("Most kills weekly reset", "Skyblock board", "Sun"),
                        ("Economy ranking spike", "SkyBuilder · #2", "22m"),
                        ("Plugin sync healthy", "3 servers reporting", "OK"),
                    ],
                    next_steps: [
                        ("Rankings", "Inspect live boards"),
                        ("Settings", "Configure leaderboards path and branding"),
                        ("Theme", "Style ranking presentation"),
                    ],
                }
            }
            Self::Votes => {
                OverviewCopy {
                    title: "Vote rewards",
                    subtitle: "Track votes, streaks, and automatic reward delivery.",
                    about: "Vote rewards connect listing sites to in-game commands, currency, roles, and badges, with streak tracking and claim flows for players.",
                    accent: "#fbbf24",
                    domain: "www.example.com/vote",
                    status: "Live",
                    stats: [
                        ("Votes today", "214"),
                        ("Streaks", "86"),
                        ("Claims pending", "19"),
                        ("Vote sites", "5"),
                    ],
                    highlights: [
                        "Vote tracking and site integrations",
                        "Reward claiming and streaks",
                        "Vote leaderboards",
                        "In-game commands, items, and roles",
                        "Discord and server delivery",
                    ],
                    activity: [
                        ("NovaCraft claimed day 7", "Streak reward delivered", "4m"),
                        ("MinecraftServers callback", "Vote recorded", "9m"),
                        ("Pending claims", "19 players offline", "Queue"),
                        ("Top voter this month", "AetherFox · 128 votes", "#1"),
                    ],
                    next_steps: [
                        ("Rewards", "Manage sites and claim rules"),
                        ("Settings", "Configure vote path and branding"),
                        ("Theme", "Customize the claim page"),
                    ],
                }
            }
            Self::Applications => {
                OverviewCopy {
                    title: "Applications",
                    subtitle: "Staff recruitment forms, review workflows, and history.",
                    about: "Applications collect custom forms for moderator, builder, and helper roles with reviewer assignment, notes, voting, and status tracking.",
                    accent: "#fb7185",
                    domain: "www.example.com/apply",
                    status: "Live",
                    stats: [
                        ("Submitted", "7"),
                        ("Reviewing", "3"),
                        ("Accepted", "12"),
                        ("Denied", "9"),
                    ],
                    highlights: [
                        "Custom forms and questions",
                        "Applicant profiles",
                        "Review workflow and history",
                        "Assign reviewers and add notes",
                        "Staff voting on applications",
                    ],
                    activity: [
                        ("Moderator · PixelPanda", "Submitted 2h ago", "Review"),
                        ("Builder · ClayMage", "Reviewing · Notes added", "Vote"),
                        ("Helper · QuietLeaf", "Submitted 3 days ago", "Assign"),
                        ("Form published · Helper", "4 questions · Open", "Live"),
                    ],
                    next_steps: [
                        ("Inbox", "Review open applications"),
                        ("Settings", "Configure applications path and branding"),
                        ("Theme", "Brand the application portal"),
                    ],
                }
            }
            Self::Analytics => {
                OverviewCopy {
                    title: "Analytics",
                    subtitle: "Website, community, and gaming insights in one dashboard.",
                    about: "Analytics brings together traffic, engagement, revenue, tickets, votes, and server activity with graphs, reports, and exportable datasets.",
                    accent: "#38bdf8",
                    domain: "www.example.com/analytics",
                    status: "Live",
                    stats: [
                        ("Revenue", "£1,094"),
                        ("Visitors", "8,420"),
                        ("Conversion", "3.8%"),
                        ("Ticket CSAT", "94%"),
                    ],
                    highlights: [
                        "Website users and page views",
                        "Forum and registration engagement",
                        "Player counts, votes, and boards",
                        "Graphs and scheduled reports",
                        "CSV and API data exports",
                    ],
                    activity: [
                        ("Weekly checkout up", "+12% vs last week", "Trend"),
                        ("Discord traffic share", "27% of sessions", "Source"),
                        ("Survival peak players", "412 online", "Peak"),
                        ("Export ready · Revenue", "Last 7 days CSV", "Download"),
                    ],
                    next_steps: [
                        ("Website", "Inspect traffic and pages"),
                        ("Community", "Review engagement metrics"),
                        ("Gaming", "Check server activity"),
                    ],
                }
            }
        }
    }
}
#[component]
pub fn StoreOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Store }
    }
}
#[component]
pub fn ForumOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Forum }
    }
}
#[component]
pub fn SupportOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Support }
    }
}
#[component]
pub fn ContentOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Content }
    }
}
#[component]
pub fn PlayersOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Players }
    }
}
#[component]
pub fn LeaderboardsOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Leaderboards }
    }
}
#[component]
pub fn VotesOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Votes }
    }
}
#[component]
pub fn ApplicationsOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Applications }
    }
}
#[component]
pub fn AnalyticsOverview() -> Element {
    rsx! {
        FeatureOverview { feature: FeatureOverviewKind::Analytics }
    }
}
#[component]
fn FeatureOverview(feature: FeatureOverviewKind) -> Element {
    let copy = feature.data();
    rsx! {
        PageHeader {
            title: copy.title,
            subtitle: copy.subtitle,
            children: rsx! {
                Button { variant: ButtonVariant::Secondary, "View on website" }
            },
        }
        div { class: "mb-6 flex flex-wrap items-center gap-2",
            StatusChip { label: copy.status, tone: copy.accent }
            span { class: "rounded-squircle-sm border border-border-subtle bg-surface/40 px-2.5 py-1 font-mono text-xs text-text-muted",
                "{copy.domain}"
            }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            for (label, value) in copy.stats {
                StatPill { label, value, accent: copy.accent }
            }
        }
        div { class: "mb-6",
            InfoCard { title: "About this feature", body: copy.about }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "What it includes",
                ul { class: "space-y-2.5",
                    for text in copy.highlights {
                        li { class: "flex gap-2.5 text-sm text-text-secondary",
                            span { class: "mt-1.5 h-1.5 w-1.5 shrink-0 rounded-full bg-accent" }
                            span { "{text}" }
                        }
                    }
                }
            }
            DataPanel { title: "Recent activity",
                for (title, meta, trailing) in copy.activity {
                    RowItem { title, meta, trailing }
                }
            }
            DataPanel { title: "Suggested next steps",
                for (title, meta) in copy.next_steps {
                    RowItem { title, meta, trailing: "Open" }
                }
            }
            DataPanel { title: "Quick status",
                RowItem {
                    title: "Public path",
                    meta: copy.domain,
                    trailing: "On website",
                }
                RowItem {
                    title: "Theme",
                    meta: "Can customize independently",
                    trailing: "Ready",
                }
                RowItem {
                    title: "Shared accounts",
                    meta: "Uses website authentication",
                    trailing: "On",
                }
                RowItem {
                    title: "Public surface",
                    meta: "Players can visit this on your website",
                    trailing: copy.status,
                }
            }
        }
    }
}
