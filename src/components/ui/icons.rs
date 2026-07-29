use dioxus::prelude::*;
use super::hugeicon::{
    ADD_01, AI_CHAT_01, ANALYTICS_UP, CANCEL_01, CHART_LINE_DATA_01, CUSTOMER_SERVICE_01,
    DASHBOARD_SQUARE_01, GIFT, GLOBE_02, HugeIcon, MENU_01, MESSAGE_01, NEWS,
    NOTIFICATION_03, SEARCH_01, SETTINGS_01, STORE_01, TICKET_01, USER_GROUP,
};
#[component]
pub fn IconGrid(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: DASHBOARD_SQUARE_01, class }
    }
}
#[component]
pub fn IconUsers(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: USER_GROUP, class }
    }
}
#[component]
pub fn IconChart(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: CHART_LINE_DATA_01, class }
    }
}
#[component]
pub fn IconBell(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: NOTIFICATION_03, class }
    }
}
#[component]
pub fn IconSettings(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: SETTINGS_01, class }
    }
}
#[component]
pub fn IconPlus(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: ADD_01, class }
    }
}
#[component]
pub fn IconSearch(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: SEARCH_01, class }
    }
}
#[component]
pub fn IconStore(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: STORE_01, class }
    }
}
#[component]
pub fn IconForum(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: MESSAGE_01, class }
    }
}
#[component]
pub fn IconTicket(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: TICKET_01, class }
    }
}
#[component]
pub fn IconNews(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: NEWS, class }
    }
}
#[component]
pub fn IconGlobe(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: GLOBE_02, class }
    }
}
#[component]
pub fn IconSupport(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: CUSTOMER_SERVICE_01, class }
    }
}
#[component]
pub fn IconAnalytics(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: ANALYTICS_UP, class }
    }
}
#[component]
pub fn IconGift(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: GIFT, class }
    }
}
#[component]
pub fn IconAi(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: AI_CHAT_01, class }
    }
}
#[component]
pub fn IconMenu(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: MENU_01, class }
    }
}
#[component]
pub fn IconClose(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        HugeIcon { icon: CANCEL_01, class }
    }
}
