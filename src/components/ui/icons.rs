use dioxus::prelude::*;

use super::hugeicon::{
    HugeIcon, ADD_01, AI_CHAT_01, ANALYTICS_UP, ARROW_DOWN_01, BOOK_OPEN_01, CANCEL_01,
    CHART_LINE_DATA_01, CUSTOMER_SERVICE_01, DASHBOARD_SQUARE_01, DISCORD, GLOBE_02, HELP_CIRCLE,
    LAYERS_01, MENU_01, MESSAGE_01, NEWS, NOTIFICATION_03, PACKAGE_01, SEARCH_01, SETTINGS_01,
    SHOPPING_CART_01, STORE_01, TICKET_01, USER_GROUP, WALLET_01,
};

#[component]
pub fn IconGrid(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: DASHBOARD_SQUARE_01, class } }
}

#[component]
pub fn IconLayers(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: LAYERS_01, class } }
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
pub fn IconWallet(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: WALLET_01, class } }
}

#[component]
pub fn IconSearch(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: SEARCH_01, class } }
}

#[component]
pub fn IconChevronDown(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: ARROW_DOWN_01, size: 12, class } }
}

#[component]
pub fn IconStore(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: STORE_01, class } }
}

#[component]
pub fn IconCart(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: SHOPPING_CART_01, class } }
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
pub fn IconWiki(#[props(default, into)] class: String) -> Element {
    rsx! { HugeIcon { icon: BOOK_OPEN_01, class } }
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

#[component]
pub fn Sparkline(
    #[props(default)] positive: bool,
    #[props(default, into)] class: String,
) -> Element {
    let stroke = if positive { "#34d399" } else { "#f87171" };
    let fill = if positive {
        "rgba(52, 211, 153, 0.12)"
    } else {
        "rgba(248, 113, 113, 0.12)"
    };
    let d = if positive {
        "M0 28 C 18 26, 28 20, 42 18 C 58 16, 70 22, 86 12 C 98 6, 110 10, 120 4 L 120 36 L 0 36 Z"
    } else {
        "M0 8 C 18 10, 28 16, 42 18 C 58 20, 70 14, 86 24 C 98 30, 110 26, 120 32 L 120 36 L 0 36 Z"
    };
    let line = if positive {
        "M0 28 C 18 26, 28 20, 42 18 C 58 16, 70 22, 86 12 C 98 6, 110 10, 120 4"
    } else {
        "M0 8 C 18 10, 28 16, 42 18 C 58 20, 70 14, 86 24 C 98 30, 110 26, 120 32"
    };

    rsx! {
        svg {
            class: "sparkline h-10 w-full {class}",
            view_box: "0 0 120 36",
            preserve_aspect_ratio: "none",
            path { d: "{d}", fill: "{fill}", stroke: "none" }
            path {
                d: "{line}",
                class: "sparkline-path",
                fill: "none",
                stroke: "{stroke}",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
    }
}
