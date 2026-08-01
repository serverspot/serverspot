use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill,
};
use crate::components::ui::*;
#[component]
pub fn ForumCategories() -> Element {
    rsx! {
        PageHeader {
            title: "Categories",
            subtitle: "Organize discussion boards, roles, and visibility for your forum.",
            children: rsx! {
                Button {
                    IconPlus {}
                    "New category"
                }
            },
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Threads", value: "1,204", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
            StatPill { label: "Members", value: "3,481", accent: "#3ecf8e" }
            StatPill { label: "Reports", value: "3", accent: "#f0a35e" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Categories",
                RowItem {
                    title: "Announcements",
                    meta: "Pinned · 18 threads",
                    trailing: "Staff",
                }
                RowItem {
                    title: "Survival",
                    meta: "General talk · 412 threads",
                    trailing: "Public",
                }
                RowItem {
                    title: "Suggestions",
                    meta: "Player ideas · 96 threads",
                    trailing: "Public",
                }
                RowItem {
                    title: "Off-topic",
                    meta: "Community lounge · 220 threads",
                    trailing: "Public",
                }
            }
            DataPanel { title: "Forum features",
                FeatureBullets {
                    FeatureBullet { text: "Categories, posts, and comments" }
                    FeatureBullet { text: "Markdown support" }
                    FeatureBullet { text: "Reactions and user mentions" }
                    FeatureBullet { text: "Attachments and tags" }
                    FeatureBullet { text: "Pinning and locking" }
                    FeatureBullet { text: "Moderation tools and reports" }
                }
            }
        }
    }
}
#[component]
pub fn ForumPosts() -> Element {
    rsx! {
        PageHeader {
            title: "Posts",
            subtitle: "Browse threads, replies, and media across every forum category.",
            Button {
                IconPlus {}
                "New thread"
            }
            Button {
                IconPlus {}
                "New thread"
            }
        }
        DataPanel { title: "Recent threads",
            RowItem {
                title: "Season 4 spawn redesign",
                meta: "Survival · 24 replies · 8 reactions",
                trailing: "2m",
            }
            RowItem {
                title: "Rank perks feedback",
                meta: "Suggestions · 11 replies · tagged: economy",
                trailing: "18m",
            }
            RowItem {
                title: "Patch notes 1.21.4",
                meta: "Announcements · Pinned · Locked comments",
                trailing: "1h",
            }
            RowItem {
                title: "Looking for builders",
                meta: "Off-topic · Mentions @SkyBuilder",
                trailing: "3h",
            }
            RowItem {
                title: "Screenshot dump",
                meta: "Survival · 3 attachments",
                trailing: "5h",
            }
        }
    }
}
#[component]
pub fn ForumModeration() -> Element {
    rsx! {
        PageHeader {
            title: "Moderation",
            subtitle: "Admin tools for reports, bans, and category permissions.",
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Admin features",
                FeatureBullets {
                    FeatureBullet { text: "Delete posts" }
                    FeatureBullet { text: "Delete comments" }
                    FeatureBullet { text: "Ban posting" }
                    FeatureBullet { text: "Manage categories" }
                    FeatureBullet { text: "Control permissions" }
                }
            }
            DataPanel { title: "Open reports",
                RowItem {
                    title: "Spam reply in Suggestions",
                    meta: "Reported by NovaCraft",
                    trailing: "Review",
                }
                RowItem {
                    title: "Toxic thread title",
                    meta: "Reported by QuietLeaf",
                    trailing: "Review",
                }
                RowItem {
                    title: "Duplicate announcement",
                    meta: "Reported by staff",
                    trailing: "Dismiss",
                }
                SettingRow {
                    title: "Auto-hide reported posts",
                    description: "Hide content after three unique member reports.",
                    enabled: false,
                }
            }
        }
    }
}
