use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill, StubPage};
use crate::components::ui::*;

#[component]
pub fn CommunityPlayers() -> Element {
    rsx! {
        PageHeader {
            title: "Players",
            subtitle: "Profiles, linked accounts, and player search across your community.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "Export players"
                }
            },
        }

        section {
            class: "mb-8 grid grid-cols-2 gap-3 md:grid-cols-4",
            StatPill { label: "Players", value: "1,842", accent: "#5b9dff" }
            StatPill { label: "Votes today", value: "214", accent: "#fbbf24" }
            StatPill { label: "Applications", value: "7", accent: "#fb7185" }
            StatPill { label: "Leaderboards", value: "4", accent: "#5eead4" }
        }

        DataPanel {
            title: "Top players",
            RowItem { title: "NovaCraft", meta: "Playtime · Level 84", trailing: "#1" }
            RowItem { title: "SkyBuilder", meta: "Balance · £2,140", trailing: "#2" }
            RowItem { title: "RedstoneRex", meta: "Kills · 1,902", trailing: "#3" }
            RowItem { title: "AetherFox", meta: "Votes · 128 this month", trailing: "#4" }
        }
    }
}

#[component]
pub fn CommunityLeaderboards() -> Element {
    rsx! {
        StubPage {
            title: "Leaderboards",
            subtitle: "Show rankings pulled from your game database in real time.",
            hint: "Leaderboard sources and display widgets will be configured here.",
        }
    }
}

#[component]
pub fn CommunityVotes() -> Element {
    rsx! {
        StubPage {
            title: "Vote rewards",
            subtitle: "Share vote links and automatically reward players in-game.",
            hint: "Vote sites, intervals, and reward packages will live here.",
        }
    }
}

#[component]
pub fn CommunityApplications() -> Element {
    rsx! {
        PageHeader {
            title: "Applications",
            subtitle: "Review staff and builder applications with custom workflows.",
        }
        DataPanel {
            title: "Open applications",
            RowItem { title: "Moderator · PixelPanda", meta: "Submitted 2h ago", trailing: "Review" }
            RowItem { title: "Builder · ClayMage", meta: "Submitted yesterday", trailing: "Review" }
            RowItem { title: "Helper · QuietLeaf", meta: "Submitted 3 days ago", trailing: "Review" }
        }
    }
}
