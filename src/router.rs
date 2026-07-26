use dioxus::prelude::*;

use crate::components::{
    analytics::{AnalyticsOverview, AnalyticsRevenue, AnalyticsTraffic},
    community::{CommunityApplications, CommunityLeaderboards, CommunityPlayers, CommunityVotes},
    content::{ContentAnnouncements, ContentBlog, ContentHelp, ContentPages},
    dashboard::{Dashboard, DashboardActivity},
    forum::{ForumAwards, ForumCategories, ForumRoles, ForumThreads},
    settings::{SettingsGeneral, SettingsHosting, SettingsIntegrations, SettingsSecurity},
    shell::AppShell,
    store::{StoreCoupons, StoreGifts, StoreOrders, StoreProducts},
    support::{SupportAutomation, SupportDepartments, SupportTickets}
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
        #[route("/store/coupons")]
        StoreCoupons {},
        #[route("/store/gifts")]
        StoreGifts {},

        #[route("/forum")]
        ForumCategories {},
        #[route("/forum/threads")]
        ForumThreads {},
        #[route("/forum/roles")]
        ForumRoles {},
        #[route("/forum/awards")]
        ForumAwards {},

        #[route("/support")]
        SupportTickets {},
        #[route("/support/departments")]
        SupportDepartments {},
        #[route("/support/automation")]
        SupportAutomation {},

        #[route("/content")]
        ContentBlog {},
        #[route("/content/help")]
        ContentHelp {},
        #[route("/content/pages")]
        ContentPages {},
        #[route("/content/announcements")]
        ContentAnnouncements {},

        #[route("/community")]
        CommunityPlayers {},
        #[route("/community/leaderboards")]
        CommunityLeaderboards {},
        #[route("/community/votes")]
        CommunityVotes {},
        #[route("/community/applications")]
        CommunityApplications {},

        #[route("/analytics")]
        AnalyticsOverview {},
        #[route("/analytics/revenue")]
        AnalyticsRevenue {},
        #[route("/analytics/traffic")]
        AnalyticsTraffic {},

        #[route("/settings")]
        SettingsGeneral {},
        #[route("/settings/integrations")]
        SettingsIntegrations {},
        #[route("/settings/security")]
        SettingsSecurity {},
        #[route("/settings/hosting")]
        SettingsHosting {},
}
