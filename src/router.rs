use dioxus::prelude::*;

use crate::components::{
    account::Account,
    accounts::{
        AccountsAuth, AccountsConnections, AccountsLinking, AccountsProfiles, AccountsRoles,
    },
    analytics::{AnalyticsCommunity, AnalyticsGaming, AnalyticsWebsite},
    community::{
        CommunityApplications, CommunityLeaderboards, CommunityPlayers, CommunityVotes,
    },
    content::{ContentBlog, ContentPages},
    dashboard::{Dashboard, DashboardActivity},
    feature_overview::{
        AnalyticsOverview, ApplicationsOverview, ContentOverview, ForumOverview,
        LeaderboardsOverview, PlayersOverview, StoreOverview, SupportOverview, VotesOverview,
    },
    feature_site::{
        AnalyticsSiteSettings, ApplicationsSiteSettings, ContentSiteSettings, ForumSiteSettings,
        LeaderboardsSiteSettings, PlayersSiteSettings, StoreSiteSettings, SupportSiteSettings,
        VotesSiteSettings,
    },
    forum::{ForumCategories, ForumModeration, ForumPosts},
    public::{
        PublicAnalytics, PublicApplications, PublicApplicationsForm, PublicBlog, PublicBlogPost,
        PublicForumIndex, PublicForumThread, PublicHome, PublicLeaderboard, PublicLeaderboards,
        PublicLogin as Login, PublicPlayer, PublicPlayers, PublicProfile, PublicShell, PublicStore,
        PublicStoreProduct, PublicSupport, PublicSupportTicket, PublicVotes, PublicVotesClaim,
    },
    settings::{
        SettingsDeveloper, SettingsGeneral, SettingsHosting, SettingsIntegrations,
        SettingsLocalisation, SettingsSecurity,
    },
    shell::AppShell,
    store::{StoreOrders, StoreProducts},
    support::{SupportAutomation, SupportHelpCentre, SupportTickets},
    theme::{SettingsTheme, SettingsThemeEditor},
};

#[derive(Debug, Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(PublicShell)]
        #[route("/")]
        PublicHome {},
        #[route("/login")]
        Login {},
        #[route("/profile")]
        PublicProfile {},
        #[route("/forum")]
        PublicForumIndex {},
        #[route("/forum/thread/:id")]
        PublicForumThread { id: u32 },
        #[route("/store")]
        PublicStore {},
        #[route("/store/product/:id")]
        PublicStoreProduct { id: u32 },
        #[route("/support")]
        PublicSupport {},
        #[route("/support/ticket/:id")]
        PublicSupportTicket { id: u32 },
        #[route("/blog")]
        PublicBlog {},
        #[route("/blog/post/:id")]
        PublicBlogPost { id: u32 },
        #[route("/players")]
        PublicPlayers {},
        #[route("/players/:id")]
        PublicPlayer { id: u32 },
        #[route("/leaderboards")]
        PublicLeaderboards {},
        #[route("/leaderboards/:id")]
        PublicLeaderboard { id: u32 },
        #[route("/votes")]
        PublicVotes {},
        #[route("/votes/claim/:id")]
        PublicVotesClaim { id: u32 },
        #[route("/applications")]
        PublicApplications {},
        #[route("/applications/form/:id")]
        PublicApplicationsForm { id: u32 },
        #[route("/analytics")]
        PublicAnalytics {},
    #[end_layout]

    #[layout(AppShell)]
        #[route("/admin")]
        Dashboard {},
        #[route("/admin/activity")]
        DashboardActivity {},

        #[route("/admin/store")]
        StoreOverview {},
        #[route("/admin/store/products")]
        StoreProducts {},
        #[route("/admin/store/orders")]
        StoreOrders {},
        #[route("/admin/store/settings")]
        StoreSiteSettings {},

        #[route("/admin/forum")]
        ForumOverview {},
        #[route("/admin/forum/categories")]
        ForumCategories {},
        #[route("/admin/forum/posts")]
        ForumPosts {},
        #[route("/admin/forum/moderation")]
        ForumModeration {},
        #[route("/admin/forum/settings")]
        ForumSiteSettings {},

        #[route("/admin/support")]
        SupportOverview {},
        #[route("/admin/support/tickets")]
        SupportTickets {},
        #[route("/admin/support/help")]
        SupportHelpCentre {},
        #[route("/admin/support/automation")]
        SupportAutomation {},
        #[route("/admin/support/settings")]
        SupportSiteSettings {},

        #[route("/admin/blog")]
        ContentOverview {},
        #[route("/admin/blog/posts")]
        ContentBlog {},
        #[route("/admin/blog/pages")]
        ContentPages {},
        #[route("/admin/blog/settings")]
        ContentSiteSettings {},

        #[route("/admin/players")]
        PlayersOverview {},
        #[route("/admin/players/profiles")]
        CommunityPlayers {},
        #[route("/admin/players/settings")]
        PlayersSiteSettings {},

        #[route("/admin/leaderboards")]
        LeaderboardsOverview {},
        #[route("/admin/leaderboards/rankings")]
        CommunityLeaderboards {},
        #[route("/admin/leaderboards/settings")]
        LeaderboardsSiteSettings {},

        #[route("/admin/votes")]
        VotesOverview {},
        #[route("/admin/votes/rewards")]
        CommunityVotes {},
        #[route("/admin/votes/settings")]
        VotesSiteSettings {},

        #[route("/admin/applications")]
        ApplicationsOverview {},
        #[route("/admin/applications/inbox")]
        CommunityApplications {},
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
        #[route("/admin/settings/linking")]
        AccountsLinking {},
        #[route("/admin/settings/connections")]
        AccountsConnections {},
        #[route("/admin/settings/profiles")]
        AccountsProfiles {},
        #[route("/admin/settings/roles")]
        AccountsRoles {},
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
        #[route("/admin/settings/theme/edit")]
        SettingsThemeEditor {},

        #[route("/admin/account")]
        Account {},
}
