use dioxus::prelude::*;
use crate::components::{
    account::Account,
    accounts::{
        AccountsAuth, AccountsConnections, AccountsLinking, AccountsProfiles,
        AccountsRoles,
    },
    analytics::{AnalyticsCommunity, AnalyticsGaming, AnalyticsWebsite},
    community::{
        CommunityApplications, CommunityLeaderboards, CommunityPlayers, CommunityVotes,
    },
    content::{ContentBlog, ContentPages},
    dashboard::{Dashboard, DashboardActivity},
    feature_overview::{
        AnalyticsOverview, ApplicationsOverview, ContentOverview, ForumOverview,
        LeaderboardsOverview, PlayersOverview, StoreOverview, SupportOverview,
        VotesOverview,
    },
    feature_site::{
        AnalyticsSiteSettings, ApplicationsSiteSettings, ContentSiteSettings,
        ForumSiteSettings, LeaderboardsSiteSettings, PlayersSiteSettings,
        StoreSiteSettings, SupportSiteSettings, VotesSiteSettings,
    },
    forum::{ForumCategories, ForumModeration, ForumPosts},
    login::Login,
    settings::{
        SettingsDeveloper, SettingsGeneral, SettingsHosting, SettingsIntegrations,
        SettingsLocalisation, SettingsSecurity,
    },
    shell::AppShell, store::{StoreOrders, StoreProducts},
    support::{SupportAutomation, SupportHelpCentre, SupportTickets},
    theme::{
        AnalyticsTheme, ApplicationsTheme, ContentTheme, ForumTheme, LeaderboardsTheme,
        PlayersTheme, StoreTheme, SupportTheme, VotesTheme,
    },
};
#[derive(Debug, Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},
    #[layout(AppShell)]
    #[route("/")]
    Dashboard {},
    #[route("/activity")]
    DashboardActivity {},
    #[route("/store")]
    StoreOverview {},
    #[route("/store/products")]
    StoreProducts {},
    #[route("/store/orders")]
    StoreOrders {},
    #[route("/store/settings")]
    StoreSiteSettings {},
    #[route("/store/theme")]
    StoreTheme {},
    #[route("/forum")]
    ForumOverview {},
    #[route("/forum/categories")]
    ForumCategories {},
    #[route("/forum/posts")]
    ForumPosts {},
    #[route("/forum/moderation")]
    ForumModeration {},
    #[route("/forum/settings")]
    ForumSiteSettings {},
    #[route("/forum/theme")]
    ForumTheme {},
    #[route("/support")]
    SupportOverview {},
    #[route("/support/tickets")]
    SupportTickets {},
    #[route("/support/help")]
    SupportHelpCentre {},
    #[route("/support/automation")]
    SupportAutomation {},
    #[route("/support/settings")]
    SupportSiteSettings {},
    #[route("/support/theme")]
    SupportTheme {},
    #[route("/blog")]
    ContentOverview {},
    #[route("/blog/posts")]
    ContentBlog {},
    #[route("/blog/pages")]
    ContentPages {},
    #[route("/blog/settings")]
    ContentSiteSettings {},
    #[route("/blog/theme")]
    ContentTheme {},
    #[route("/players")]
    PlayersOverview {},
    #[route("/players/profiles")]
    CommunityPlayers {},
    #[route("/players/settings")]
    PlayersSiteSettings {},
    #[route("/players/theme")]
    PlayersTheme {},
    #[route("/leaderboards")]
    LeaderboardsOverview {},
    #[route("/leaderboards/rankings")]
    CommunityLeaderboards {},
    #[route("/leaderboards/settings")]
    LeaderboardsSiteSettings {},
    #[route("/leaderboards/theme")]
    LeaderboardsTheme {},
    #[route("/votes")]
    VotesOverview {},
    #[route("/votes/rewards")]
    CommunityVotes {},
    #[route("/votes/settings")]
    VotesSiteSettings {},
    #[route("/votes/theme")]
    VotesTheme {},
    #[route("/applications")]
    ApplicationsOverview {},
    #[route("/applications/inbox")]
    CommunityApplications {},
    #[route("/applications/settings")]
    ApplicationsSiteSettings {},
    #[route("/applications/theme")]
    ApplicationsTheme {},
    #[route("/analytics")]
    AnalyticsOverview {},
    #[route("/analytics/website")]
    AnalyticsWebsite {},
    #[route("/analytics/community")]
    AnalyticsCommunity {},
    #[route("/analytics/gaming")]
    AnalyticsGaming {},
    #[route("/analytics/settings")]
    AnalyticsSiteSettings {},
    #[route("/analytics/theme")]
    AnalyticsTheme {},
    #[route("/settings")]
    SettingsGeneral {},
    #[route("/settings/authentication")]
    AccountsAuth {},
    #[route("/settings/linking")]
    AccountsLinking {},
    #[route("/settings/connections")]
    AccountsConnections {},
    #[route("/settings/profiles")]
    AccountsProfiles {},
    #[route("/settings/roles")]
    AccountsRoles {},
    #[route("/settings/localisation")]
    SettingsLocalisation {},
    #[route("/settings/developer")]
    SettingsDeveloper {},
    #[route("/settings/integrations")]
    SettingsIntegrations {},
    #[route("/settings/security")]
    SettingsSecurity {},
    #[route("/settings/hosting")]
    SettingsHosting {},
    #[route("/account")]
    Account {},
}
