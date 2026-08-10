use dioxus::prelude::*;

use crate::components::{
    account::Account,
    accounts::{AccountsAuth, AccountsRoleNew, AccountsRoles, AccountsStaff},
    analytics::{
        AnalyticsCommunity, AnalyticsGaming, AnalyticsOverview, AnalyticsSiteSettings,
        AnalyticsWebsite,
    },
    community::{
        ApplicationFormNew, ApplicationReview, CommunityApplications, CommunityLeaderboards,
        CommunityPlayers, CommunityVotes, LeaderboardsBoardEdit, LeaderboardsBoardNew,
        PlayersProfileDetail, VoteRewardEdit, VoteRewardNew,
    },
    content::{ContentBlog, ContentOverview, ContentPostEdit, ContentPostNew, ContentSiteSettings},
    dashboard::{Dashboard, DashboardActivity},
    feature_overview::{
        ApplicationsOverview, LeaderboardsOverview, PlayersOverview, VotesOverview,
    },
    feature_site::{
        ApplicationsSiteSettings, LeaderboardsSiteSettings, PlayersSiteSettings, VotesSiteSettings,
    },
    forum::{
        ForumAutoModeration, ForumBoardEdit, ForumBoardNew, ForumBoards, ForumModeration,
        ForumOverview, ForumSiteSettings, ForumThread, ForumThreadNew, ForumThreads,
    },
    login::Login,
    not_found::AdminNotFound,
    settings::{
        SettingsDeveloper, SettingsGeneral, SettingsHosting, SettingsIntegrations,
        SettingsLocalisation, SettingsSecurity,
    },
    shell::AppShell,
    store::{
        StoreCategories, StoreCategoryEdit, StoreCategoryNew, StoreCouponEdit, StoreCouponNew,
        StoreCoupons, StoreOrders, StoreOverview, StoreProductEdit, StoreProductNew, StoreProducts,
        StoreSettings,
    },
    support::{
        SupportAutomation, SupportHelpCentre, SupportHelpEdit, SupportHelpNew, SupportOverview,
        SupportSiteSettings, SupportTicket, SupportTickets,
    },
    theme::SettingsTheme,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/admin/login")]
    Login {},

    #[redirect("/", || Route::Dashboard {})]
    #[redirect("/login", || Route::Login {})]

    #[layout(AppShell)]
        #[route("/admin")]
        Dashboard {},
        #[route("/admin/activity")]
        DashboardActivity {},

        #[route("/admin/store")]
        StoreOverview {},
        #[route("/admin/store/products")]
        StoreProducts {},
        #[route("/admin/store/products/new")]
        StoreProductNew {},
        #[route("/admin/store/products/:id")]
        StoreProductEdit { id: u64 },
        #[route("/admin/store/categories")]
        StoreCategories {},
        #[route("/admin/store/categories/new")]
        StoreCategoryNew {},
        #[route("/admin/store/categories/:id")]
        StoreCategoryEdit { id: u64 },
        #[route("/admin/store/coupons")]
        StoreCoupons {},
        #[route("/admin/store/coupons/new")]
        StoreCouponNew {},
        #[route("/admin/store/coupons/:id")]
        StoreCouponEdit { id: u64 },
        #[route("/admin/store/orders")]
        StoreOrders {},
        #[route("/admin/store/settings")]
        StoreSettings {},

        #[route("/admin/forum")]
        ForumOverview {},
        #[route("/admin/forum/boards")]
        ForumBoards {},
        #[route("/admin/forum/boards/new")]
        ForumBoardNew {},
        #[route("/admin/forum/boards/:id")]
        ForumBoardEdit { id: u64 },
        #[route("/admin/forum/threads")]
        ForumThreads {},
        #[route("/admin/forum/threads/new")]
        ForumThreadNew {},
        #[route("/admin/forum/threads/:id")]
        ForumThread { id: u64 },
        #[route("/admin/forum/moderation")]
        ForumModeration {},
        #[route("/admin/forum/auto-moderation")]
        ForumAutoModeration {},
        #[route("/admin/forum/settings")]
        ForumSiteSettings {},

        #[route("/admin/support")]
        SupportOverview {},
        #[route("/admin/support/tickets")]
        SupportTickets {},
        #[route("/admin/support/tickets/:id")]
        SupportTicket { id: u64 },
        #[route("/admin/support/help")]
        SupportHelpCentre {},
        #[route("/admin/support/help/new")]
        SupportHelpNew {},
        #[route("/admin/support/help/:id")]
        SupportHelpEdit { id: u64 },
        #[route("/admin/support/automation")]
        SupportAutomation {},
        #[route("/admin/support/settings")]
        SupportSiteSettings {},

        #[route("/admin/blog")]
        ContentOverview {},
        #[route("/admin/blog/posts")]
        ContentBlog {},
        #[route("/admin/blog/posts/new")]
        ContentPostNew {},
        #[route("/admin/blog/posts/:id")]
        ContentPostEdit { id: u64 },
        #[route("/admin/blog/settings")]
        ContentSiteSettings {},

        #[route("/admin/players")]
        PlayersOverview {},
        #[route("/admin/players/profiles")]
        CommunityPlayers {},
        #[route("/admin/players/profiles/:id")]
        PlayersProfileDetail { id: u64 },
        #[route("/admin/players/settings")]
        PlayersSiteSettings {},

        #[route("/admin/leaderboards")]
        LeaderboardsOverview {},
        #[route("/admin/leaderboards/rankings")]
        CommunityLeaderboards {},
        #[route("/admin/leaderboards/rankings/new")]
        LeaderboardsBoardNew {},
        #[route("/admin/leaderboards/rankings/:id")]
        LeaderboardsBoardEdit { id: u64 },
        #[route("/admin/leaderboards/settings")]
        LeaderboardsSiteSettings {},

        #[route("/admin/votes")]
        VotesOverview {},
        #[route("/admin/votes/rewards")]
        CommunityVotes {},
        #[route("/admin/votes/rewards/new")]
        VoteRewardNew {},
        #[route("/admin/votes/rewards/:id")]
        VoteRewardEdit { id: u64 },
        #[route("/admin/votes/settings")]
        VotesSiteSettings {},

        #[route("/admin/applications")]
        ApplicationsOverview {},
        #[route("/admin/applications/inbox")]
        CommunityApplications {},
        #[route("/admin/applications/inbox/:id")]
        ApplicationReview { id: u64 },
        #[route("/admin/applications/forms/new")]
        ApplicationFormNew {},
        #[route("/admin/applications/settings")]
        ApplicationsSiteSettings {},

        #[route("/admin/analytics")]
        AnalyticsOverview {},
        #[route("/admin/analytics/website")]
        AnalyticsWebsite {},
        #[route("/admin/analytics/community")]
        AnalyticsCommunity {},
        #[route("/admin/analytics/gaming")]
        AnalyticsGaming {},
        #[route("/admin/analytics/settings")]
        AnalyticsSiteSettings {},

        #[route("/admin/settings")]
        SettingsGeneral {},
        #[route("/admin/settings/authentication")]
        AccountsAuth {},
        #[redirect("/admin/settings/linking", || Route::AccountsAuth {})]
        #[redirect("/admin/settings/connections", || Route::AccountsAuth {})]
        #[route("/admin/settings/staff")]
        AccountsStaff {},
        #[route("/admin/settings/roles")]
        AccountsRoles {},
        #[route("/admin/settings/roles/new")]
        AccountsRoleNew {},
        #[route("/admin/settings/localisation")]
        SettingsLocalisation {},
        #[route("/admin/settings/developer")]
        SettingsDeveloper {},
        #[route("/admin/settings/integrations")]
        SettingsIntegrations {},
        #[route("/admin/settings/security")]
        SettingsSecurity {},
        #[route("/admin/settings/hosting")]
        SettingsHosting {},
        #[route("/admin/settings/theme")]
        SettingsTheme {},

        #[route("/admin/account")]
        Account {},

        #[route("/admin/:..segments")]
        AdminNotFound { segments: Vec<String> },
}
