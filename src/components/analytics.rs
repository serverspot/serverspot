use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill, StubPage};
use crate::components::ui::*;

#[component]
pub fn AnalyticsOverview() -> Element {
    rsx! {
        PageHeader {
            title: "Overview",
            subtitle: "Revenue, engagement, and support performance across your website.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "Last 7 days"
                }
            },
        }

        section {
            class: "mb-8 grid grid-cols-2 gap-3 md:grid-cols-4",
            StatPill { label: "Revenue", value: "£1,094", accent: "#3ecf8e" }
            StatPill { label: "Visitors", value: "8,420", accent: "#5b9dff" }
            StatPill { label: "Conversion", value: "3.8%", accent: "#b8b0ff" }
            StatPill { label: "Ticket CSAT", value: "94%", accent: "#f071a5" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Weekly checkout",
                div {
                    class: "rounded-squircle-lg px-2 pt-3 pb-1",
                    style: "background: color-mix(in srgb, #3ecf8e 8%, transparent);",
                    svg {
                        class: "h-36 w-full",
                        view_box: "0 0 400 140",
                        preserve_aspect_ratio: "none",
                        path {
                            d: "M0 108 C 45 98, 70 86, 110 80 C 160 72, 190 92, 230 64 C 275 36, 310 48, 350 28 C 375 18, 390 16, 400 12",
                            fill: "none",
                            stroke: "#3ecf8e",
                            stroke_width: "2",
                            stroke_linecap: "round",
                        }
                    }
                }
            }
            DataPanel {
                title: "Top traffic sources",
                RowItem { title: "Direct", meta: "Returning players", trailing: "38%" }
                RowItem { title: "Discord", meta: "Invite & announcements", trailing: "27%" }
                RowItem { title: "Search", meta: "Organic", trailing: "18%" }
                RowItem { title: "Vote sites", meta: "Reward campaigns", trailing: "17%" }
            }
        }
    }
}

#[component]
pub fn AnalyticsRevenue() -> Element {
    rsx! {
        StubPage {
            title: "Revenue",
            subtitle: "Breakdown of store sales by product, coupon, and currency.",
            hint: "Detailed revenue charts and export tools will appear here.",
        }
    }
}

#[component]
pub fn AnalyticsTraffic() -> Element {
    rsx! {
        StubPage {
            title: "Traffic",
            subtitle: "Visitors, referral sources, and conversion funnels.",
            hint: "Traffic reports and funnel views will land here.",
        }
    }
}
