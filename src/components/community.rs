use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
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
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Players", value: "1,842", accent: "#5b9dff" }
            StatPill { label: "Votes today", value: "214", accent: "#fbbf24" }
            StatPill { label: "Applications", value: "7", accent: "#fb7185" }
            StatPill { label: "Leaderboards", value: "4", accent: "#5eead4" }
        }

        DataPanel {
            title: "Top players",
            RowItem { title: "NovaCraft", meta: "Playtime · Level 84", trailing: "#1", email: "novacraft@players.serverspot.app" }
            RowItem { title: "SkyBuilder", meta: "Balance · £2,140", trailing: "#2", email: "skybuilder@players.serverspot.app" }
            RowItem { title: "RedstoneRex", meta: "Kills · 1,902", trailing: "#3", email: "redstonerex@players.serverspot.app" }
            RowItem { title: "AetherFox", meta: "Votes · 128 this month", trailing: "#4", email: "aetherfox@players.serverspot.app" }
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
            RowItem { title: "Moderator · PixelPanda", meta: "Submitted 2h ago", trailing: "Review", email: "pixelpanda@players.serverspot.app" }
            RowItem { title: "Builder · ClayMage", meta: "Submitted yesterday", trailing: "Review", email: "claymage@players.serverspot.app" }
            RowItem { title: "Helper · QuietLeaf", meta: "Submitted 3 days ago", trailing: "Review", email: "quietleaf@players.serverspot.app" }
        }
    }
}
