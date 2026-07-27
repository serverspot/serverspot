use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill, StatusChip,
};
use crate::components::ui::*;

#[component]
pub fn ContentBlog() -> Element {
    rsx! {
        PageHeader {
            title: "Posts",
            subtitle: "Publish blog posts, news articles, and scheduled updates.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "New post"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Published", value: "48", accent: "#f071a5" }
            StatPill { label: "Drafts", value: "6", accent: "#87d1fe" }
            StatPill { label: "Scheduled", value: "3", accent: "#f5c14a" }
            StatPill { label: "Authors", value: "5", accent: "#5b9dff" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Publishing features",
                FeatureBullets {
                    FeatureBullet { text: "Blog posts and news articles" }
                    FeatureBullet { text: "Categories and tags" }
                    FeatureBullet { text: "Markdown editor" }
                    FeatureBullet { text: "Images" }
                    FeatureBullet { text: "Drafts" }
                    FeatureBullet { text: "Scheduled publishing" }
                }
            }
            DataPanel {
                title: "Recent posts",
                RowItem { title: "Season 4 launch recap", meta: "News · Published · Featured", trailing: "Today" }
                RowItem { title: "Economy changes", meta: "Blog · Draft · Needs approval", trailing: "Edit" }
                RowItem { title: "Weekend crate event", meta: "News · Scheduled Fri 18:00", trailing: "Queue" }
                RowItem { title: "Builder spotlight", meta: "Blog · Homepage placement", trailing: "Live" }
            }
        }
    }
}

#[component]
pub fn ContentPages() -> Element {
    rsx! {
        PageHeader {
            title: "Pages",
            subtitle: "Custom pages, homepage placement, and site widgets.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "New page"
                }
            },
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Admin features",
                FeatureBullets {
                    FeatureBullet { text: "Manage authors" }
                    FeatureBullet { text: "Approvals" }
                    FeatureBullet { text: "Featured articles" }
                    FeatureBullet { text: "Homepage placement" }
                }
                SettingRow {
                    title: "Require editorial approval",
                    description: "Drafts need a second staff approval before publishing.",
                    enabled: true,
                }
            }
            DataPanel {
                title: "Pages & widgets",
                RowItem { title: "Homepage slider", meta: "3 slides · Autoplay on", trailing: "Live" }
                RowItem { title: "Staff page", meta: "12 members listed", trailing: "Live" }
                RowItem { title: "Changelog", meta: "Last entry 2 days ago", trailing: "Live" }
                RowItem { title: "Popup promo", meta: "SUMMER20 banner", trailing: "Scheduled" }
                div { class: "mt-3 flex flex-wrap gap-2",
                    StatusChip { label: "Draft", tone: "#87d1fe" }
                    StatusChip { label: "Scheduled", tone: "#f5c14a" }
                    StatusChip { label: "Featured", tone: "#f071a5" }
                }
            }
        }
    }
}
