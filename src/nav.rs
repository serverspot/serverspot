use crate::router::Route;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Dashboard,
    Store,
    Forum,
    Support,
    Content,
    Players,
    Leaderboards,
    Votes,
    Applications,
    Analytics,
    Settings,
    Account,
}

#[derive(Clone, Copy)]
pub struct SubLink {
    pub label: &'static str,
    pub route: Route,
}

impl Section {
    pub const ALL: &'static [Section] = &[
        Section::Dashboard,
        Section::Store,
        Section::Forum,
        Section::Support,
        Section::Content,
        Section::Players,
        Section::Leaderboards,
        Section::Votes,
        Section::Applications,
        Section::Analytics,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Dashboard => "Dashboard",
            Section::Store => "Store",
            Section::Forum => "Forum",
            Section::Support => "Support",
            Section::Content => "Blog",
            Section::Players => "Players",
            Section::Leaderboards => "Leaderboards",
            Section::Votes => "Vote rewards",
            Section::Applications => "Applications",
            Section::Analytics => "Analytics",
            Section::Settings => "Settings",
            Section::Account => "Account",
        }
    }

    pub fn document_title(self) -> &'static str {
        match self {
            Section::Dashboard => "ServerSpot | Dashboard",
            Section::Store => "ServerSpot | Store",
            Section::Forum => "ServerSpot | Forum",
            Section::Support => "ServerSpot | Support",
            Section::Content => "ServerSpot | Blog",
            Section::Players => "ServerSpot | Players",
            Section::Leaderboards => "ServerSpot | Leaderboards",
            Section::Votes => "ServerSpot | Vote rewards",
            Section::Applications => "ServerSpot | Applications",
            Section::Analytics => "ServerSpot | Analytics",
            Section::Settings => "ServerSpot | Settings",
            Section::Account => "ServerSpot | Account",
        }
    }

    pub fn theme_vars(self) -> &'static str {
        match self {
            Section::Dashboard | Section::Settings | Section::Account => {
                "--color-accent:#87d1fe;--color-accent-strong:color-mix(in srgb,#87d1fe 82%,black);--color-accent-muted:color-mix(in srgb,#87d1fe 68%,black);--color-accent-soft:color-mix(in srgb,#87d1fe 16%,transparent);--rail-accent:#87d1fe;"
            }
            Section::Store => {
                "--color-accent:#3ecf8e;--color-accent-strong:color-mix(in srgb,#3ecf8e 82%,black);--color-accent-muted:color-mix(in srgb,#3ecf8e 68%,black);--color-accent-soft:color-mix(in srgb,#3ecf8e 16%,transparent);--rail-accent:#3ecf8e;"
            }
            Section::Forum => {
                "--color-accent:#5b9dff;--color-accent-strong:color-mix(in srgb,#5b9dff 82%,black);--color-accent-muted:color-mix(in srgb,#5b9dff 68%,black);--color-accent-soft:color-mix(in srgb,#5b9dff 16%,transparent);--rail-accent:#5b9dff;"
            }
            Section::Support => {
                "--color-accent:#f0a35e;--color-accent-strong:color-mix(in srgb,#f0a35e 82%,black);--color-accent-muted:color-mix(in srgb,#f0a35e 68%,black);--color-accent-soft:color-mix(in srgb,#f0a35e 16%,transparent);--rail-accent:#f0a35e;"
            }
            Section::Content => {
                "--color-accent:#f071a5;--color-accent-strong:color-mix(in srgb,#f071a5 82%,black);--color-accent-muted:color-mix(in srgb,#f071a5 68%,black);--color-accent-soft:color-mix(in srgb,#f071a5 16%,transparent);--rail-accent:#f071a5;"
            }
            Section::Players => {
                "--color-accent:#69bdf2;--color-accent-strong:color-mix(in srgb,#69bdf2 82%,black);--color-accent-muted:color-mix(in srgb,#69bdf2 68%,black);--color-accent-soft:color-mix(in srgb,#69bdf2 16%,transparent);--rail-accent:#69bdf2;"
            }
            Section::Leaderboards => {
                "--color-accent:#5eead4;--color-accent-strong:color-mix(in srgb,#5eead4 82%,black);--color-accent-muted:color-mix(in srgb,#5eead4 68%,black);--color-accent-soft:color-mix(in srgb,#5eead4 16%,transparent);--rail-accent:#5eead4;"
            }
            Section::Votes => {
                "--color-accent:#fbbf24;--color-accent-strong:color-mix(in srgb,#fbbf24 82%,black);--color-accent-muted:color-mix(in srgb,#fbbf24 68%,black);--color-accent-soft:color-mix(in srgb,#fbbf24 16%,transparent);--rail-accent:#fbbf24;"
            }
            Section::Applications => {
                "--color-accent:#fb7185;--color-accent-strong:color-mix(in srgb,#fb7185 82%,black);--color-accent-muted:color-mix(in srgb,#fb7185 68%,black);--color-accent-soft:color-mix(in srgb,#fb7185 16%,transparent);--rail-accent:#fb7185;"
            }
            Section::Analytics => {
                "--color-accent:#38bdf8;--color-accent-strong:color-mix(in srgb,#38bdf8 82%,black);--color-accent-muted:color-mix(in srgb,#38bdf8 68%,black);--color-accent-soft:color-mix(in srgb,#38bdf8 16%,transparent);--rail-accent:#38bdf8;"
            }
        }
    }

    pub fn home(self) -> Route {
        match self {
            Section::Dashboard => Route::Dashboard {},
            Section::Store => Route::StoreOverview {},
            Section::Forum => Route::ForumOverview {},
            Section::Support => Route::SupportOverview {},
            Section::Content => Route::ContentOverview {},
            Section::Players => Route::PlayersOverview {},
            Section::Leaderboards => Route::LeaderboardsOverview {},
            Section::Votes => Route::VotesOverview {},
            Section::Applications => Route::ApplicationsOverview {},
            Section::Analytics => Route::AnalyticsOverview {},
            Section::Settings => Route::SettingsGeneral {},
            Section::Account => Route::Account {},
        }
    }

    pub fn subs(self) -> &'static [SubLink] {
        match self {
            Section::Dashboard => &[
                SubLink {
                    label: "Overview",
                    route: Route::Dashboard {},
                },
                SubLink {
                    label: "Activity",
                    route: Route::DashboardActivity {},
                },
            ],
            Section::Store => &[
                SubLink {
                    label: "Overview",
                    route: Route::StoreOverview {},
                },
                SubLink {
                    label: "Products",
                    route: Route::StoreProducts {},
                },
                SubLink {
                    label: "Orders",
                    route: Route::StoreOrders {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::StoreSiteSettings {},
                },
            ],
            Section::Forum => &[
                SubLink {
                    label: "Overview",
                    route: Route::ForumOverview {},
                },
                SubLink {
                    label: "Categories",
                    route: Route::ForumCategories {},
                },
                SubLink {
                    label: "Posts",
                    route: Route::ForumPosts {},
                },
                SubLink {
                    label: "Moderation",
                    route: Route::ForumModeration {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::ForumSiteSettings {},
                },
            ],
            Section::Support => &[
                SubLink {
                    label: "Overview",
                    route: Route::SupportOverview {},
                },
                SubLink {
                    label: "Tickets",
                    route: Route::SupportTickets {},
                },
                SubLink {
                    label: "Help centre",
                    route: Route::SupportHelpCentre {},
                },
                SubLink {
                    label: "Automation",
                    route: Route::SupportAutomation {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::SupportSiteSettings {},
                },
            ],
            Section::Content => &[
                SubLink {
                    label: "Overview",
                    route: Route::ContentOverview {},
                },
                SubLink {
                    label: "Posts",
                    route: Route::ContentBlog {},
                },
                SubLink {
                    label: "Pages",
                    route: Route::ContentPages {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::ContentSiteSettings {},
                },
            ],
            Section::Players => &[
                SubLink {
                    label: "Overview",
                    route: Route::PlayersOverview {},
                },
                SubLink {
                    label: "Profiles",
                    route: Route::CommunityPlayers {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::PlayersSiteSettings {},
                },
            ],
            Section::Leaderboards => &[
                SubLink {
                    label: "Overview",
                    route: Route::LeaderboardsOverview {},
                },
                SubLink {
                    label: "Rankings",
                    route: Route::CommunityLeaderboards {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::LeaderboardsSiteSettings {},
                },
            ],
            Section::Votes => &[
                SubLink {
                    label: "Overview",
                    route: Route::VotesOverview {},
                },
                SubLink {
                    label: "Rewards",
                    route: Route::CommunityVotes {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::VotesSiteSettings {},
                },
            ],
            Section::Applications => &[
                SubLink {
                    label: "Overview",
                    route: Route::ApplicationsOverview {},
                },
                SubLink {
                    label: "Inbox",
                    route: Route::CommunityApplications {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::ApplicationsSiteSettings {},
                },
            ],
            Section::Analytics => &[
                SubLink {
                    label: "Overview",
                    route: Route::AnalyticsOverview {},
                },
                SubLink {
                    label: "Website",
                    route: Route::AnalyticsWebsite {},
                },
                SubLink {
                    label: "Community",
                    route: Route::AnalyticsCommunity {},
                },
                SubLink {
                    label: "Gaming",
                    route: Route::AnalyticsGaming {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::AnalyticsSiteSettings {},
                },
            ],
            Section::Settings => &[
                SubLink {
                    label: "General",
                    route: Route::SettingsGeneral {},
                },
                SubLink {
                    label: "Authentication",
                    route: Route::AccountsAuth {},
                },
                SubLink {
                    label: "Account linking",
                    route: Route::AccountsLinking {},
                },
                SubLink {
                    label: "Connections",
                    route: Route::AccountsConnections {},
                },
                SubLink {
                    label: "User profiles",
                    route: Route::AccountsProfiles {},
                },
                SubLink {
                    label: "Roles",
                    route: Route::AccountsRoles {},
                },
                SubLink {
                    label: "Localisation",
                    route: Route::SettingsLocalisation {},
                },
                SubLink {
                    label: "Theme",
                    route: Route::SettingsTheme {},
                },
                SubLink {
                    label: "Developer",
                    route: Route::SettingsDeveloper {},
                },
                SubLink {
                    label: "Integrations",
                    route: Route::SettingsIntegrations {},
                },
                SubLink {
                    label: "Security",
                    route: Route::SettingsSecurity {},
                },
                SubLink {
                    label: "Hosting",
                    route: Route::SettingsHosting {},
                },
            ],
            Section::Account => &[SubLink {
                label: "Profile",
                route: Route::Account {},
            }],
        }
    }
}

pub fn section_for(route: &Route) -> Section {
    match route {
        Route::Login {}
        | Route::PublicProfile {}
        | Route::PublicHome {}
        | Route::PublicForumIndex {}
        | Route::PublicForumThread { .. }
        | Route::PublicStore {}
        | Route::PublicStoreProduct { .. }
        | Route::PublicSupport {}
        | Route::PublicSupportTicket { .. }
        | Route::PublicBlog {}
        | Route::PublicBlogPost { .. }
        | Route::PublicPlayers {}
        | Route::PublicPlayer { .. }
        | Route::PublicLeaderboards {}
        | Route::PublicLeaderboard { .. }
        | Route::PublicVotes {}
        | Route::PublicVotesClaim { .. }
        | Route::PublicApplications {}
        | Route::PublicApplicationsForm { .. }
        | Route::PublicAnalytics {} => Section::Dashboard,
        Route::Dashboard {} | Route::DashboardActivity {} => Section::Dashboard,
        Route::StoreOverview {}
        | Route::StoreProducts {}
        | Route::StoreOrders {}
        | Route::StoreSiteSettings {} => Section::Store,
        Route::ForumOverview {}
        | Route::ForumCategories {}
        | Route::ForumPosts {}
        | Route::ForumModeration {}
        | Route::ForumSiteSettings {} => Section::Forum,
        Route::SupportOverview {}
        | Route::SupportTickets {}
        | Route::SupportHelpCentre {}
        | Route::SupportAutomation {}
        | Route::SupportSiteSettings {} => Section::Support,
        Route::ContentOverview {}
        | Route::ContentBlog {}
        | Route::ContentPages {}
        | Route::ContentSiteSettings {} => Section::Content,
        Route::PlayersOverview {}
        | Route::CommunityPlayers {}
        | Route::PlayersSiteSettings {} => Section::Players,
        Route::LeaderboardsOverview {}
        | Route::CommunityLeaderboards {}
        | Route::LeaderboardsSiteSettings {} => Section::Leaderboards,
        Route::VotesOverview {}
        | Route::CommunityVotes {}
        | Route::VotesSiteSettings {} => Section::Votes,
        Route::ApplicationsOverview {}
        | Route::CommunityApplications {}
        | Route::ApplicationsSiteSettings {} => Section::Applications,
        Route::AnalyticsOverview {}
        | Route::AnalyticsWebsite {}
        | Route::AnalyticsCommunity {}
        | Route::AnalyticsGaming {}
        | Route::AnalyticsSiteSettings {} => Section::Analytics,
        Route::SettingsGeneral {}
        | Route::AccountsAuth {}
        | Route::AccountsLinking {}
        | Route::AccountsConnections {}
        | Route::AccountsProfiles {}
        | Route::AccountsRoles {}
        | Route::SettingsLocalisation {}
        | Route::SettingsTheme {}
        | Route::SettingsThemeEditor {}
        | Route::SettingsDeveloper {}
        | Route::SettingsIntegrations {}
        | Route::SettingsSecurity {}
        | Route::SettingsHosting {} => Section::Settings,
        Route::Account {} => Section::Account,
    }
}

pub fn crumb_for(route: &Route) -> &'static str {
    match route {
        Route::Login {} => "Login",
        Route::PublicProfile {} => "Profile",
        Route::PublicHome {} => "Home",
        Route::PublicForumIndex {} => "Forum",
        Route::PublicForumThread { .. } => "Thread",
        Route::PublicStore {} => "Store",
        Route::PublicStoreProduct { .. } => "Product",
        Route::PublicSupport {} => "Support",
        Route::PublicSupportTicket { .. } => "Ticket",
        Route::PublicBlog {} => "Blog",
        Route::PublicBlogPost { .. } => "Post",
        Route::PublicPlayers {} => "Players",
        Route::PublicPlayer { .. } => "Player",
        Route::PublicLeaderboards {} => "Leaderboards",
        Route::PublicLeaderboard { .. } => "Board",
        Route::PublicVotes {} => "Votes",
        Route::PublicVotesClaim { .. } => "Claim",
        Route::PublicApplications {} => "Applications",
        Route::PublicApplicationsForm { .. } => "Form",
        Route::PublicAnalytics {} => "Analytics",
        Route::Dashboard {} => "Overview",
        Route::DashboardActivity {} => "Activity",
        Route::StoreOverview {} => "Overview",
        Route::StoreProducts {} => "Products",
        Route::StoreOrders {} => "Orders",
        Route::StoreSiteSettings {} => "Settings",
        Route::ForumOverview {} => "Overview",
        Route::ForumCategories {} => "Categories",
        Route::ForumPosts {} => "Posts",
        Route::ForumModeration {} => "Moderation",
        Route::ForumSiteSettings {} => "Settings",
        Route::SupportOverview {} => "Overview",
        Route::SupportTickets {} => "Tickets",
        Route::SupportHelpCentre {} => "Help centre",
        Route::SupportAutomation {} => "Automation",
        Route::SupportSiteSettings {} => "Settings",
        Route::ContentOverview {} => "Overview",
        Route::ContentBlog {} => "Posts",
        Route::ContentPages {} => "Pages",
        Route::ContentSiteSettings {} => "Settings",
        Route::PlayersOverview {} => "Overview",
        Route::CommunityPlayers {} => "Profiles",
        Route::PlayersSiteSettings {} => "Settings",
        Route::LeaderboardsOverview {} => "Overview",
        Route::CommunityLeaderboards {} => "Rankings",
        Route::LeaderboardsSiteSettings {} => "Settings",
        Route::VotesOverview {} => "Overview",
        Route::CommunityVotes {} => "Rewards",
        Route::VotesSiteSettings {} => "Settings",
        Route::ApplicationsOverview {} => "Overview",
        Route::CommunityApplications {} => "Inbox",
        Route::ApplicationsSiteSettings {} => "Settings",
        Route::AnalyticsOverview {} => "Overview",
        Route::AnalyticsWebsite {} => "Website",
        Route::AnalyticsCommunity {} => "Community",
        Route::AnalyticsGaming {} => "Gaming",
        Route::AnalyticsSiteSettings {} => "Settings",
        Route::SettingsGeneral {} => "General",
        Route::AccountsAuth {} => "Authentication",
        Route::AccountsLinking {} => "Account linking",
        Route::AccountsConnections {} => "Connections",
        Route::AccountsProfiles {} => "User profiles",
        Route::AccountsRoles {} => "Roles",
        Route::SettingsLocalisation {} => "Localisation",
        Route::SettingsTheme {} => "Theme",
        Route::SettingsThemeEditor {} => "Edit theme",
        Route::SettingsDeveloper {} => "Developer",
        Route::SettingsIntegrations {} => "Integrations",
        Route::SettingsSecurity {} => "Security",
        Route::SettingsHosting {} => "Hosting",
        Route::Account {} => "Profile",
    }
}

pub fn is_theme_editor(route: &Route) -> bool {
    matches!(route, Route::SettingsThemeEditor {})
}