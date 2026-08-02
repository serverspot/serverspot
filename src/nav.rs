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

    pub fn accent(self) -> &'static str {
        match self {
            Section::Dashboard | Section::Settings | Section::Account => "#87d1fe",
            Section::Store => "#3ecf8e",
            Section::Forum => "#5b9dff",
            Section::Support => "#f0a35e",
            Section::Content => "#f071a5",
            Section::Players => "#69bdf2",
            Section::Leaderboards => "#5eead4",
            Section::Votes => "#fbbf24",
            Section::Applications => "#fb7185",
            Section::Analytics => "#38bdf8",
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
                SubLink {
                    label: "Theme",
                    route: Route::StoreTheme {},
                },
            ],
            Section::Forum => &[
                SubLink {
                    label: "Overview",
                    route: Route::ForumOverview {},
                },
                SubLink {
                    label: "Boards",
                    route: Route::ForumBoards {},
                },
                SubLink {
                    label: "Threads",
                    route: Route::ForumThreads {},
                },
                SubLink {
                    label: "Moderation",
                    route: Route::ForumModeration {},
                },
                SubLink {
                    label: "Auto Moderation",
                    route: Route::ForumAutoModeration {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::ForumSiteSettings {},
                },
                SubLink {
                    label: "Theme",
                    route: Route::ForumTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::SupportTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::ContentTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::PlayersTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::LeaderboardsTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::VotesTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::ApplicationsTheme {},
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
                SubLink {
                    label: "Theme",
                    route: Route::AnalyticsTheme {},
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
        Route::Login {} => Section::Dashboard,
        Route::Dashboard {} | Route::DashboardActivity {} => Section::Dashboard,
        Route::StoreOverview {}
        | Route::StoreProducts {}
        | Route::StoreOrders {}
        | Route::StoreSiteSettings {}
        | Route::StoreTheme {} => Section::Store,
        Route::ForumOverview {}
        | Route::ForumBoards {}
        | Route::ForumThreads {}
        | Route::ForumThread { .. }
        | Route::ForumModeration {}
        | Route::ForumAutoModeration {}
        | Route::ForumSiteSettings {}
        | Route::ForumTheme {} => Section::Forum,
        Route::SupportOverview {}
        | Route::SupportTickets {}
        | Route::SupportHelpCentre {}
        | Route::SupportAutomation {}
        | Route::SupportSiteSettings {}
        | Route::SupportTheme {} => Section::Support,
        Route::ContentOverview {}
        | Route::ContentBlog {}
        | Route::ContentPages {}
        | Route::ContentSiteSettings {}
        | Route::ContentTheme {} => Section::Content,
        Route::PlayersOverview {}
        | Route::CommunityPlayers {}
        | Route::PlayersSiteSettings {}
        | Route::PlayersTheme {} => Section::Players,
        Route::LeaderboardsOverview {}
        | Route::CommunityLeaderboards {}
        | Route::LeaderboardsSiteSettings {}
        | Route::LeaderboardsTheme {} => Section::Leaderboards,
        Route::VotesOverview {}
        | Route::CommunityVotes {}
        | Route::VotesSiteSettings {}
        | Route::VotesTheme {} => Section::Votes,
        Route::ApplicationsOverview {}
        | Route::CommunityApplications {}
        | Route::ApplicationsSiteSettings {}
        | Route::ApplicationsTheme {} => Section::Applications,
        Route::AnalyticsOverview {}
        | Route::AnalyticsWebsite {}
        | Route::AnalyticsCommunity {}
        | Route::AnalyticsGaming {}
        | Route::AnalyticsSiteSettings {}
        | Route::AnalyticsTheme {} => Section::Analytics,
        Route::SettingsGeneral {}
        | Route::AccountsAuth {}
        | Route::AccountsLinking {}
        | Route::AccountsConnections {}
        | Route::AccountsProfiles {}
        | Route::AccountsRoles {}
        | Route::SettingsLocalisation {}
        | Route::SettingsDeveloper {}
        | Route::SettingsIntegrations {}
        | Route::SettingsSecurity {}
        | Route::SettingsHosting {} => Section::Settings,
        Route::Account {} => Section::Account,
    }
}

pub fn subnav_active(current: &Route, target: Route) -> bool {
    match (current, target) {
        (Route::ForumThread { .. }, Route::ForumThreads {}) => true,
        (current, target) => *current == target,
    }
}

pub fn crumb_for(route: &Route) -> &'static str {
    match route {
        Route::Login {} => "Login",
        Route::Dashboard {} => "Overview",
        Route::DashboardActivity {} => "Activity",
        Route::StoreOverview {} => "Overview",
        Route::StoreProducts {} => "Products",
        Route::StoreOrders {} => "Orders",
        Route::StoreSiteSettings {} => "Settings",
        Route::StoreTheme {} => "Theme",
        Route::ForumOverview {} => "Overview",
        Route::ForumBoards {} => "Boards",
        Route::ForumThreads {} => "Threads",
        Route::ForumThread { .. } => "Thread",
        Route::ForumModeration {} => "Moderation",
        Route::ForumAutoModeration {} => "Auto Moderation",
        Route::ForumSiteSettings {} => "Settings",
        Route::ForumTheme {} => "Theme",
        Route::SupportOverview {} => "Overview",
        Route::SupportTickets {} => "Tickets",
        Route::SupportHelpCentre {} => "Help centre",
        Route::SupportAutomation {} => "Automation",
        Route::SupportSiteSettings {} => "Settings",
        Route::SupportTheme {} => "Theme",
        Route::ContentOverview {} => "Overview",
        Route::ContentBlog {} => "Posts",
        Route::ContentPages {} => "Pages",
        Route::ContentSiteSettings {} => "Settings",
        Route::ContentTheme {} => "Theme",
        Route::PlayersOverview {} => "Overview",
        Route::CommunityPlayers {} => "Profiles",
        Route::PlayersSiteSettings {} => "Settings",
        Route::PlayersTheme {} => "Theme",
        Route::LeaderboardsOverview {} => "Overview",
        Route::CommunityLeaderboards {} => "Rankings",
        Route::LeaderboardsSiteSettings {} => "Settings",
        Route::LeaderboardsTheme {} => "Theme",
        Route::VotesOverview {} => "Overview",
        Route::CommunityVotes {} => "Rewards",
        Route::VotesSiteSettings {} => "Settings",
        Route::VotesTheme {} => "Theme",
        Route::ApplicationsOverview {} => "Overview",
        Route::CommunityApplications {} => "Inbox",
        Route::ApplicationsSiteSettings {} => "Settings",
        Route::ApplicationsTheme {} => "Theme",
        Route::AnalyticsOverview {} => "Overview",
        Route::AnalyticsWebsite {} => "Website",
        Route::AnalyticsCommunity {} => "Community",
        Route::AnalyticsGaming {} => "Gaming",
        Route::AnalyticsSiteSettings {} => "Settings",
        Route::AnalyticsTheme {} => "Theme",
        Route::SettingsGeneral {} => "General",
        Route::AccountsAuth {} => "Authentication",
        Route::AccountsLinking {} => "Account linking",
        Route::AccountsConnections {} => "Connections",
        Route::AccountsProfiles {} => "User profiles",
        Route::AccountsRoles {} => "Roles",
        Route::SettingsLocalisation {} => "Localisation",
        Route::SettingsDeveloper {} => "Developer",
        Route::SettingsIntegrations {} => "Integrations",
        Route::SettingsSecurity {} => "Security",
        Route::SettingsHosting {} => "Hosting",
        Route::Account {} => "Profile",
    }
}

pub fn is_theme_editor(route: &Route) -> bool {
    matches!(
        route,
        Route::StoreTheme {}
            | Route::ForumTheme {}
            | Route::SupportTheme {}
            | Route::ContentTheme {}
            | Route::PlayersTheme {}
            | Route::LeaderboardsTheme {}
            | Route::VotesTheme {}
            | Route::ApplicationsTheme {}
            | Route::AnalyticsTheme {}
    )
}
