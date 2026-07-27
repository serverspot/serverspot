use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, InfoCard, PageHeader, RowItem, SettingRow, StatPill,
    StatusChip,
};
use crate::components::ui::*;

#[component]
pub fn AccountsAuth() -> Element {
    rsx! {
        PageHeader {
            title: "Authentication",
            subtitle: "Shared account system used across all platform features.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Invite user"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Accounts", value: "3,481", accent: "#5b9dff" }
            StatPill { label: "2FA enabled", value: "62%", accent: "#3ecf8e" }
            StatPill { label: "Sessions", value: "918", accent: "#87d1fe" }
            StatPill { label: "Recovery open", value: "4", accent: "#f0a35e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Auth features",
                FeatureBullets {
                    FeatureBullet { text: "User registration" }
                    FeatureBullet { text: "Email/password authentication" }
                    FeatureBullet { text: "Two-factor authentication" }
                    FeatureBullet { text: "Password recovery" }
                    FeatureBullet { text: "Session management" }
                    FeatureBullet { text: "Account security settings" }
                    FeatureBullet { text: "User privacy settings" }
                }
            }
            DataPanel {
                title: "Security defaults",
                SettingRow {
                    title: "Require email verification",
                    description: "New accounts must verify before purchases or posting.",
                    enabled: true,
                }
                SettingRow {
                    title: "Force 2FA for staff",
                    description: "Staff and admin roles must enable authenticator apps.",
                    enabled: true,
                }
                SettingRow {
                    title: "Remember device",
                    description: "Allow trusted devices to skip 2FA for 30 days.",
                    enabled: false,
                }
            }
        }
    }
}

#[component]
pub fn AccountsLinking() -> Element {
    rsx! {
        PageHeader {
            title: "Account linking",
            subtitle: "Connect website accounts with in-game identities across servers.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "Generate codes"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Linked players", value: "1,204", accent: "#3ecf8e" }
            StatPill { label: "Pending codes", value: "18", accent: "#f5c14a" }
            StatPill { label: "Games", value: "3", accent: "#5b9dff" }
            StatPill { label: "Servers", value: "7", accent: "#87d1fe" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Linking features",
                FeatureBullets {
                    FeatureBullet { text: "Verification through server-generated codes" }
                    FeatureBullet { text: "Link multiple game accounts" }
                    FeatureBullet { text: "View linked accounts" }
                    FeatureBullet { text: "Remove linked accounts" }
                    FeatureBullet { text: "Support multiple games and servers" }
                }
            }
            DataPanel {
                title: "Recent links",
                RowItem {
                    title: "NovaCraft · Survival",
                    meta: "Minecraft · Verified with code N7K2",
                    trailing: "Linked",
                    email: "novacraft@players.serverspot.app",
                }
                RowItem {
                    title: "SkyBuilder · Skyblock",
                    meta: "Minecraft · Verified with code Q1M9",
                    trailing: "Linked",
                    email: "skybuilder@players.serverspot.app",
                }
                RowItem {
                    title: "AetherFox · Creative",
                    meta: "Awaiting in-game confirmation",
                    trailing: "Pending",
                    email: "aetherfox@players.serverspot.app",
                }
            }
        }
    }
}

#[component]
pub fn AccountsConnections() -> Element {
    rsx! {
        PageHeader {
            title: "Connections",
            subtitle: "External service integrations through OAuth for login and sync.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Add provider"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Providers", value: "6", accent: "#7b8cff" }
            StatPill { label: "OAuth logins", value: "842", accent: "#5b9dff" }
            StatPill { label: "Role syncs", value: "128", accent: "#3ecf8e" }
            StatPill { label: "Custom apps", value: "2", accent: "#f0a35e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Supported connections",
                RowItem { title: "Discord", meta: "Login · Role sync · Bot rewards", trailing: "Connected" }
                RowItem { title: "Steam", meta: "Login · Profile information", trailing: "Connected" }
                RowItem { title: "Microsoft", meta: "Login provider", trailing: "Connected" }
                RowItem { title: "Google", meta: "Login provider", trailing: "Connected" }
                RowItem { title: "GitHub", meta: "Developer login", trailing: "Optional" }
                RowItem { title: "Custom OAuth", meta: "Bring your own identity provider", trailing: "2 apps" }
            }
            DataPanel {
                title: "Uses",
                FeatureBullets {
                    FeatureBullet { text: "Login providers" }
                    FeatureBullet { text: "Discord role syncing" }
                    FeatureBullet { text: "Profile information" }
                    FeatureBullet { text: "Bot integrations" }
                    FeatureBullet { text: "Automated rewards" }
                }
            }
        }
    }
}

#[component]
pub fn AccountsProfiles() -> Element {
    rsx! {
        PageHeader {
            title: "User profiles",
            subtitle: "Public profiles shared across the platform with privacy controls.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "Preview profile"
                }
            },
        }

        div {
            class: "mb-6 grid gap-3 sm:grid-cols-3",
            InfoCard {
                title: "Public profile",
                body: "Visible to everyone, including guests browsing the site.",
            }
            InfoCard {
                title: "Members only",
                body: "Visible to signed-in users across your website.",
            }
            InfoCard {
                title: "Private profile",
                body: "Hidden from directories; staff can still review when needed.",
            }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Profile fields",
                FeatureBullets {
                    FeatureBullet { text: "Username" }
                    FeatureBullet { text: "Avatar" }
                    FeatureBullet { text: "Biography" }
                    FeatureBullet { text: "Linked accounts" }
                    FeatureBullet { text: "Achievements" }
                    FeatureBullet { text: "Statistics" }
                    FeatureBullet { text: "Forum activity" }
                    FeatureBullet { text: "Leaderboard rankings" }
                    FeatureBullet { text: "Badges" }
                }
            }
            DataPanel {
                title: "Example profiles",
                RowItem {
                    title: "NovaCraft",
                    meta: "Public · 14 badges · Forum regular",
                    trailing: "View",
                    email: "novacraft@players.serverspot.app",
                }
                RowItem {
                    title: "SkyBuilder",
                    meta: "Members only · Builder badge",
                    trailing: "View",
                    email: "skybuilder@players.serverspot.app",
                }
                RowItem {
                    title: "QuietLeaf",
                    meta: "Private · Staff applicant",
                    trailing: "Hidden",
                    email: "quietleaf@players.serverspot.app",
                }
            }
        }
    }
}

#[component]
pub fn AccountsRoles() -> Element {
    rsx! {
        PageHeader {
            title: "Permissions & roles",
            subtitle: "Fully configurable permission system with hierarchy and groups.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Create role"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Roles", value: "12", accent: "#5b9dff" }
            StatPill { label: "Permissions", value: "86", accent: "#87d1fe" }
            StatPill { label: "Groups", value: "5", accent: "#3ecf8e" }
            StatPill { label: "Overrides", value: "9", accent: "#f0a35e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Example permissions",
                div { class: "mb-3 flex flex-wrap gap-2",
                    StatusChip { label: "forums.create", tone: "#5b9dff" }
                    StatusChip { label: "forums.delete", tone: "#fb7185" }
                    StatusChip { label: "tickets.manage", tone: "#f0a35e" }
                    StatusChip { label: "news.publish", tone: "#f071a5" }
                    StatusChip { label: "applications.review", tone: "#3ecf8e" }
                    StatusChip { label: "analytics.view", tone: "#87d1fe" }
                }
                FeatureBullets {
                    FeatureBullet { text: "Custom roles" }
                    FeatureBullet { text: "Role hierarchy" }
                    FeatureBullet { text: "User permissions" }
                    FeatureBullet { text: "Group permissions" }
                    FeatureBullet { text: "Permission management" }
                }
            }
            DataPanel {
                title: "Role hierarchy",
                RowItem { title: "Owner", meta: "Full access · 1 member", trailing: "Level 100" }
                RowItem { title: "Admin", meta: "Manage staff & settings · 4 members", trailing: "Level 80" }
                RowItem { title: "Moderator", meta: "Forums & tickets · 11 members", trailing: "Level 40" }
                RowItem { title: "Helper", meta: "Limited support tools · 18 members", trailing: "Level 20" }
                RowItem { title: "Member", meta: "Default community access", trailing: "Level 1" }
            }
        }
    }
}
