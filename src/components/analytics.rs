use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};

#[component]
pub fn AnalyticsWebsite() -> Element {
    rsx! {
        PageHeader {
            title: "Website analytics",
            subtitle: "Users, page views, traffic sources, and popular pages.",
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Users", value: "6,104", accent: "#5b9dff" }
            StatPill { label: "Page views", value: "28.4k", accent: "#87d1fe" }
            StatPill { label: "Bounce rate", value: "41%", accent: "#f0a35e" }
            StatPill { label: "Avg. session", value: "3m 12s", accent: "#3ecf8e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Weekly traffic",
                div {
                    class: "rounded-squircle-lg px-2 pt-3 pb-1",
                    style: "background: color-mix(in srgb, #5b9dff 8%, transparent);",
                    svg {
                        class: "h-36 w-full",
                        view_box: "0 0 400 140",
                        preserve_aspect_ratio: "none",
                        path {
                            d: "M0 108 C 45 98, 70 86, 110 80 C 160 72, 190 92, 230 64 C 275 36, 310 48, 350 28 C 375 18, 390 16, 400 12",
                            fill: "none",
                            stroke: "#5b9dff",
                            stroke_width: "2",
                            stroke_linecap: "round",
                        }
                    }
                }
            }
            DataPanel {
                title: "Traffic & pages",
                RowItem { title: "Direct", meta: "Returning players", trailing: "38%" }
                RowItem { title: "Discord", meta: "Invite & announcements", trailing: "27%" }
                RowItem { title: "Search", meta: "Organic", trailing: "18%" }
                RowItem { title: "Vote sites", meta: "Reward campaigns", trailing: "17%" }
                RowItem { title: "/store", meta: "Popular page", trailing: "4.2k views" }
                RowItem { title: "/help/vote-rewards", meta: "Popular page", trailing: "2.1k views" }
            }
        }
    }
}

#[component]
pub fn AnalyticsCommunity() -> Element {
    rsx! {
        PageHeader {
            title: "Community analytics",
            subtitle: "Forum activity, registrations, and engagement trends.",
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Registrations", value: "146", accent: "#3ecf8e" }
            StatPill { label: "Forum posts", value: "512", accent: "#f071a5" }
            StatPill { label: "Active members", value: "890", accent: "#5b9dff" }
            StatPill { label: "Engagement", value: "64%", accent: "#87d1fe" }
        }

        DataPanel {
            title: "This week",
            RowItem { title: "New forum threads", meta: "Survival and Suggestions leading", trailing: "+38" }
            RowItem { title: "Verified emails", meta: "Registration funnel", trailing: "91%" }
            RowItem { title: "Returning visitors", meta: "7-day window", trailing: "54%" }
            RowItem { title: "Application starts", meta: "Staff recruitment", trailing: "11" }
        }
    }
}

#[component]
pub fn AnalyticsGaming() -> Element {
    rsx! {
        PageHeader {
            title: "Gaming analytics",
            subtitle: "Player counts, votes, leaderboards, and server activity.",
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Peak players", value: "412", accent: "#5eead4" }
            StatPill { label: "Votes", value: "1,480", accent: "#fbbf24" }
            StatPill { label: "Leaderboard hits", value: "9.2k", accent: "#5b9dff" }
            StatPill { label: "Server uptime", value: "99.8%", accent: "#3ecf8e" }
        }

        DataPanel {
            title: "Server activity",
            RowItem { title: "Survival", meta: "Avg 186 online · Economy climbing", trailing: "Healthy" }
            RowItem { title: "Skyblock", meta: "Avg 94 online · Vote claims up", trailing: "Healthy" }
            RowItem { title: "Creative", meta: "Avg 41 online · Builder spike weekends", trailing: "Steady" }
            RowItem { title: "Lobby", meta: "Queue depth normal", trailing: "OK" }
        }
    }
}
