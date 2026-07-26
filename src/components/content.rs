use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
use crate::components::ui::*;

#[component]
pub fn ContentBlog() -> Element {
    rsx! {
        PageHeader {
            title: "Blog",
            subtitle: "Publish news, patch notes, and event posts for your players.",
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
            StatPill { label: "Help articles", value: "32", accent: "#5b9dff" }
            StatPill { label: "Custom pages", value: "9", accent: "#3ecf8e" }
        }

        DataPanel {
            title: "Recent posts",
            RowItem { title: "Season 4 launch recap", meta: "Blog · Published", trailing: "Today" }
            RowItem { title: "Economy changes", meta: "Blog · Draft", trailing: "Edit" }
            RowItem { title: "How to claim vote rewards", meta: "Help center", trailing: "Pinned" }
            RowItem { title: "Server rules", meta: "Help center", trailing: "Updated" }
        }
    }
}

#[component]
pub fn ContentPages() -> Element {
    rsx! {
        PageHeader {
            title: "Pages",
            subtitle: "Custom pages, sliders, and site widgets.",
        }
        DataPanel {
            title: "Pages & widgets",
            RowItem { title: "Homepage slider", meta: "3 slides · Autoplay on", trailing: "Live" }
            RowItem { title: "Staff page", meta: "12 members listed", trailing: "Live" }
            RowItem { title: "Changelog", meta: "Last entry 2 days ago", trailing: "Live" }
            RowItem { title: "Popup promo", meta: "SUMMER20 banner", trailing: "Scheduled" }
        }
    }
}
