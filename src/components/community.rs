use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill,
    StatusChip,
};
use crate::components::ui::*;
#[component]
pub fn CommunityPlayers() -> Element {
    rsx! {
        PageHeader {
            title: "Player profiles",
            subtitle: "Dedicated gaming profiles with stats, ranks, and linked accounts.",
            Button { variant: ButtonVariant::Secondary, "Export players" }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Players", value: "1,842", accent: "#69bdf2" }
            StatPill { label: "Linked accounts", value: "1,204", accent: "#3ecf8e" }
            StatPill { label: "Games", value: "3", accent: "#5b9dff" }
            StatPill { label: "Servers", value: "7", accent: "#87d1fe" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Profile features",
                FeatureBullets {
                    FeatureBullet { text: "Linked game accounts" }
                    FeatureBullet { text: "Statistics and playtime" }
                    FeatureBullet { text: "Ranks and achievements" }
                    FeatureBullet { text: "Leaderboard positions" }
                    FeatureBullet { text: "History tracking" }
                    FeatureBullet { text: "Multiple games and servers" }
                }
            }
            DataPanel { title: "Top players",
                RowItem {
                    title: "NovaCraft",
                    meta: "Playtime · Level 84 · VIP",
                    trailing: "#1",
                    email: "novacraft@players.serverspot.app",
                }
                RowItem {
                    title: "SkyBuilder",
                    meta: "Balance · £2,140 · Builder",
                    trailing: "#2",
                    email: "skybuilder@players.serverspot.app",
                }
                RowItem {
                    title: "RedstoneRex",
                    meta: "Kills · 1,902",
                    trailing: "#3",
                    email: "redstonerex@players.serverspot.app",
                }
                RowItem {
                    title: "AetherFox",
                    meta: "Votes · 128 this month",
                    trailing: "#4",
                    email: "aetherfox@players.serverspot.app",
                }
            }
        }
    }
}
#[component]
pub fn CommunityLeaderboards() -> Element {
    rsx! {
        PageHeader {
            title: "Leaderboards",
            subtitle: "Player ranking and statistics across games, servers, and time ranges.",
            Button {
                IconPlus {}
                "New board"
            }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Boards", value: "4", accent: "#5eead4" }
            StatPill { label: "Tracked stats", value: "18", accent: "#5b9dff" }
            StatPill { label: "Data sources", value: "3", accent: "#87d1fe" }
            StatPill { label: "Updates / hr", value: "12", accent: "#3ecf8e" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Leaderboard features",
                FeatureBullets {
                    FeatureBullet { text: "Multiple leaderboards and games" }
                    FeatureBullet { text: "Custom statistics" }
                    FeatureBullet { text: "Player rankings and ranking history" }
                    FeatureBullet { text: "Time-based rankings" }
                }
                div { class: "mt-4 flex flex-wrap gap-2",
                    StatusChip { label: "Top players", tone: "#5eead4" }
                    StatusChip { label: "Most kills", tone: "#fb7185" }
                    StatusChip { label: "Playtime", tone: "#5b9dff" }
                    StatusChip { label: "Economy", tone: "#3ecf8e" }
                    StatusChip { label: "Achievements", tone: "#f5c14a" }
                }
            }
            DataPanel { title: "Data sources",
                RowItem {
                    title: "Game APIs",
                    meta: "Pull live stats from external endpoints",
                    trailing: "Active",
                }
                RowItem {
                    title: "Server plugins",
                    meta: "Push events from Minecraft plugins",
                    trailing: "Active",
                }
                RowItem {
                    title: "Manual input",
                    meta: "Staff-entered seasonal contests",
                    trailing: "Optional",
                }
                SettingRow {
                    title: "Public ranking history",
                    description: "Show weekly and monthly movement on profiles.",
                    enabled: true,
                }
            }
        }
    }
}
#[component]
pub fn CommunityVotes() -> Element {
    rsx! {
        PageHeader {
            title: "Vote rewards",
            subtitle: "Track votes, claim rewards, and sync streaks across voting sites.",
            Button {
                IconPlus {}
                "Add vote site"
            }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Votes today", value: "214", accent: "#fbbf24" }
            StatPill { label: "Streaks", value: "86", accent: "#f0a35e" }
            StatPill { label: "Claims pending", value: "19", accent: "#87d1fe" }
            StatPill { label: "Sites", value: "5", accent: "#5b9dff" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Voting features",
                FeatureBullets {
                    FeatureBullet { text: "Vote tracking" }
                    FeatureBullet { text: "Voting websites integration" }
                    FeatureBullet { text: "Reward claiming" }
                    FeatureBullet { text: "Vote streaks" }
                    FeatureBullet { text: "Vote leaderboards" }
                }
                div { class: "mt-4 border-t border-border-subtle pt-4",
                    p { class: "mb-2 text-xs font-medium uppercase tracking-wide text-text-muted",
                        "Rewards"
                    }
                    FeatureBullets {
                        FeatureBullet { text: "In-game commands" }
                        FeatureBullet { text: "Currency and items" }
                        FeatureBullet { text: "Website roles and badges" }
                    }
                }
            }
            DataPanel { title: "Integrations",
                RowItem {
                    title: "Minecraft voting sites",
                    meta: "Top sites configured with callbacks",
                    trailing: "5 live",
                }
                RowItem {
                    title: "Discord",
                    meta: "Announce streaks and top voters",
                    trailing: "On",
                }
                RowItem {
                    title: "Game servers",
                    meta: "Deliver commands on claim",
                    trailing: "On",
                }
                SettingRow {
                    title: "Auto-claim when online",
                    description: "Run reward commands as soon as the player joins.",
                    enabled: true,
                }
            }
        }
    }
}
#[component]
pub fn CommunityApplications() -> Element {
    rsx! {
        PageHeader {
            title: "Staff applications",
            subtitle: "Recruitment forms, review workflows, and application history.",
            Button { IconPlus {} "New form" }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Submitted", value: "7", accent: "#fb7185" }
            StatPill { label: "Reviewing", value: "3", accent: "#f5c14a" }
            StatPill { label: "Accepted", value: "12", accent: "#3ecf8e" }
            StatPill { label: "Denied", value: "9", accent: "#858899" }
        }
        div { class: "mb-4 flex flex-wrap gap-2",
            StatusChip { label: "Submitted", tone: "#fb7185" }
            StatusChip { label: "Reviewing", tone: "#f5c14a" }
            StatusChip { label: "Accepted", tone: "#3ecf8e" }
            StatusChip { label: "Denied", tone: "#858899" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Application features",
                FeatureBullets {
                    FeatureBullet { text: "Custom application forms" }
                    FeatureBullet { text: "Application questions" }
                    FeatureBullet { text: "Applicant profiles" }
                    FeatureBullet { text: "Review workflow" }
                    FeatureBullet { text: "Application history" }
                }
                div { class: "mt-4 border-t border-border-subtle pt-4",
                    p { class: "mb-2 text-xs font-medium uppercase tracking-wide text-text-muted",
                        "Staff tools"
                    }
                    FeatureBullets {
                        FeatureBullet { text: "Assign reviewers" }
                        FeatureBullet { text: "Add notes" }
                        FeatureBullet { text: "Vote on applications" }
                        FeatureBullet { text: "Manage applications" }
                    }
                }
            }
            DataPanel { title: "Open applications",
                RowItem {
                    title: "Moderator · PixelPanda",
                    meta: "Submitted 2h ago · 4 reviewers",
                    trailing: "Review",
                    email: "pixelpanda@players.serverspot.app",
                }
                RowItem {
                    title: "Builder · ClayMage",
                    meta: "Reviewing · Notes added",
                    trailing: "Vote",
                    email: "claymage@players.serverspot.app",
                }
                RowItem {
                    title: "Helper · QuietLeaf",
                    meta: "Submitted 3 days ago",
                    trailing: "Assign",
                    email: "quietleaf@players.serverspot.app",
                }
            }
        }
    }
}
