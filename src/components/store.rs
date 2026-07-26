use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill, StubPage};
use crate::components::ui::*;

#[component]
pub fn StoreProducts() -> Element {
    rsx! {
        PageHeader {
            title: "Products",
            subtitle: "Manage products, stock, and checkout packages for your server shop.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Add product"
                }
            },
        }

        section {
            class: "mb-8 grid grid-cols-2 gap-3 md:grid-cols-4",
            StatPill { label: "Revenue", value: "£4,281", accent: "#3ecf8e" }
            StatPill { label: "Orders", value: "96", accent: "#5b9dff" }
            StatPill { label: "Avg. order", value: "£11.40", accent: "#b8b0ff" }
            StatPill { label: "Coupons used", value: "18", accent: "#f0a35e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Top products",
                RowItem { title: "VIP Rank", meta: "Rank · 42 sold", trailing: "£1,260" }
                RowItem { title: "Crate Key Bundle", meta: "Item · 31 sold", trailing: "£620" }
                RowItem { title: "Cosmetics Pack", meta: "Bundle · 19 sold", trailing: "£380" }
                RowItem { title: "Home Teleport", meta: "Perk · 14 sold", trailing: "£210" }
            }
            DataPanel {
                title: "Active promotions",
                RowItem { title: "SUMMER20", meta: "20% off lifetime ranks", trailing: "Active" }
                RowItem { title: "Weekend crates", meta: "Buy 2 get 1 free", trailing: "Ends Sun" }
                RowItem { title: "Creator: NOVA", meta: "10% off · 8% credit share", trailing: "12 uses" }
            }
        }
    }
}

#[component]
pub fn StoreOrders() -> Element {
    rsx! {
        PageHeader {
            title: "Orders",
            subtitle: "Track checkouts, refunds, and delivery status for store purchases.",
        }
        DataPanel {
            title: "Recent orders",
            RowItem { title: "#4821 · VIP Rank", meta: "NovaCraft · Paid", trailing: "£29.99" }
            RowItem { title: "#4818 · Crate Key Bundle", meta: "SkyBuilder · Paid", trailing: "£9.99" }
            RowItem { title: "#4812 · Cosmetics Pack", meta: "AetherFox · Refunded", trailing: "£14.99" }
            RowItem { title: "#4809 · Home Teleport", meta: "RedstoneRex · Paid", trailing: "£4.99" }
        }
    }
}

#[component]
pub fn StoreCoupons() -> Element {
    rsx! {
        StubPage {
            title: "Coupons",
            subtitle: "Create percentage, fixed, and creator coupon codes.",
            hint: "Coupon rules, usage limits, and creator share settings will live here.",
        }
    }
}

#[component]
pub fn StoreGifts() -> Element {
    rsx! {
        StubPage {
            title: "Gift cards",
            subtitle: "Issue store credit and gift packages players can redeem.",
            hint: "Gift card balances and redemption history will appear here.",
        }
    }
}
