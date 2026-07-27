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
        }
    }

    pub fn accent(self) -> &'static str {
        match self {
            Section::Dashboard => "#87d1fe",
            Section::Store => "#3ecf8e",
            Section::Forum => "#5b9dff",
            Section::Support => "#f0a35e",
            Section::Content => "#f071a5",
            Section::Players => "#69bdf2",
            Section::Leaderboards => "#5eead4",
            Section::Votes => "#fbbf24",
            Section::Applications => "#fb7185",
            Section::Analytics => "#38bdf8",
            Section::Settings => "#b0b3c0",
        }
    }

    pub fn accent_style(self) -> &'static str {
        match self {
            Section::Dashboard => "--rail-accent: #87d1fe;",
            Section::Store => "--rail-accent: #3ecf8e;",
            Section::Forum => "--rail-accent: #5b9dff;",
            Section::Support => "--rail-accent: #f0a35e;",
            Section::Content => "--rail-accent: #f071a5;",
            Section::Players => "--rail-accent: #69bdf2;",
            Section::Leaderboards => "--rail-accent: #5eead4;",
            Section::Votes => "--rail-accent: #fbbf24;",
            Section::Applications => "--rail-accent: #fb7185;",
            Section::Analytics => "--rail-accent: #38bdf8;",
            Section::Settings => "--rail-accent: #b0b3c0;",
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
        | Route::ForumCategories {}
        | Route::ForumPosts {}
        | Route::ForumModeration {}
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
    }
}

pub fn crumb_for(route: &Route) -> &'static str {
    for section in Section::ALL
        .iter()
        .copied()
        .chain(std::iter::once(Section::Settings))
    {
        for sub in section.subs() {
            if &sub.route == route {
                return sub.label;
            }
        }
    }
    "ServerSpot"
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
