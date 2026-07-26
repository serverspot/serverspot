use dioxus::prelude::*;

use crate::components::{
    analytics::AnalyticsOverview,
    community::{CommunityApplications, CommunityPlayers},
    content::{ContentBlog, ContentPages},
    dashboard::{Dashboard, DashboardActivity},
    forum::ForumCategories,
    settings::{SettingsGeneral, SettingsHosting, SettingsIntegrations, SettingsSecurity},
    shell::AppShell,
    store::{StoreOrders, StoreProducts},
    support::{SupportAutomation, SupportTickets},
};

#[derive(Debug, Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/activity")]
        DashboardActivity {},

        #[route("/store")]
        StoreProducts {},
        #[route("/store/orders")]
        StoreOrders {},

        #[route("/forum")]
        ForumCategories {},

        #[route("/support")]
        SupportTickets {},
        #[route("/support/automation")]
        SupportAutomation {},

        #[route("/content")]
        ContentBlog {},
        #[route("/content/pages")]
        ContentPages {},

        #[route("/community")]
        CommunityPlayers {},
        #[route("/community/applications")]
        CommunityApplications {},

        #[route("/analytics")]
        AnalyticsOverview {},

        #[route("/settings")]
        SettingsGeneral {},
        #[route("/settings/integrations")]
        SettingsIntegrations {},
        #[route("/settings/security")]
        SettingsSecurity {},
        #[route("/settings/hosting")]
        SettingsHosting {},
}
