//! App icon wrappers over Hugeicons stroke-rounded glyphs.
//!
//! Raw icons live in [`crate::components::ui::hugeicon`]. Regenerate with:
//! `node scripts/gen-hugeicons.mjs` (requires `@hugeicons/core-free-icons`).

use dioxus::prelude::*;

use super::hugeicon::{
    ADD_01, AI_CHAT_01, ANALYTICS_UP, CANCEL_01, CHART_LINE_DATA_01, CUSTOMER_SERVICE_01,
    DASHBOARD_SQUARE_01, DISCORD, GLOBE_02, HELP_CIRCLE, HugeIcon, MENU_01, MESSAGE_01, NEWS,
    NOTIFICATION_03, PACKAGE_01, SEARCH_01, SETTINGS_01, STORE_01, TICKET_01, USER_GROUP,
};

#[component]
pub fn IconGrid(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: DASHBOARD_SQUARE_01, class } }
}

#[component]
pub fn IconUsers(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: USER_GROUP, class } }
}

#[component]
pub fn IconChart(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: CHART_LINE_DATA_01, class } }
}

#[component]
pub fn IconBell(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: NOTIFICATION_03, class } }
}

#[component]
pub fn IconSettings(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: SETTINGS_01, class } }
}

#[component]
pub fn IconPlus(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: ADD_01, class } }
}

#[component]
pub fn IconSearch(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: SEARCH_01, class } }
}

#[component]
pub fn IconStore(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: STORE_01, class } }
}

#[component]
pub fn IconForum(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: MESSAGE_01, class } }
}

#[component]
pub fn IconTicket(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: TICKET_01, class } }
}

#[component]
pub fn IconNews(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: NEWS, class } }
}

#[component]
pub fn IconDiscord(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: DISCORD, class } }
}

#[component]
pub fn IconGlobe(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: GLOBE_02, class } }
}

#[component]
pub fn IconSupport(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: CUSTOMER_SERVICE_01, class } }
}

#[component]
pub fn IconAnalytics(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: ANALYTICS_UP, class } }
}

#[component]
pub fn IconPackage(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: PACKAGE_01, class } }
}

#[component]
pub fn IconHelp(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: HELP_CIRCLE, class } }
}

#[component]
pub fn IconAi(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: AI_CHAT_01, class } }
}

#[component]
pub fn IconMenu(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: MENU_01, class } }
}

#[component]
pub fn IconClose(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: CANCEL_01, class } }
}
