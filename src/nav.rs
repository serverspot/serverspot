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

#[derive(Clone)]
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
            Section::Dashboard | Section::Settings => "#87d1fe",
            Section::Store => "#3ecf8e",
            Section::Forum => "#5b9dff",
            Section::Support => "#f0a35e",
            Section::Content => "#f071a5",
            Section::Players => "#69bdf2",
            Section::Leaderboards => "#5eead4",
            Section::Votes => "#fbbf24",
            Section::Applications => "#fb7185",
            Section::Analytics => "#38bdf8",
            Section::Account => "#b0b3c0",
        }
    }

    pub fn rail_style(self) -> &'static str {
        match self {
            Section::Dashboard | Section::Settings => "--rail-accent: #87d1fe;",
            Section::Store => "--rail-accent: #3ecf8e;",
            Section::Forum => "--rail-accent: #5b9dff;",
            Section::Support => "--rail-accent: #f0a35e;",
            Section::Content => "--rail-accent: #f071a5;",
            Section::Players => "--rail-accent: #69bdf2;",
            Section::Leaderboards => "--rail-accent: #5eead4;",
            Section::Votes => "--rail-accent: #fbbf24;",
            Section::Applications => "--rail-accent: #fb7185;",
            Section::Analytics => "--rail-accent: #38bdf8;",
            Section::Account => "--rail-accent: #b0b3c0;",
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
            Section::Dashboard | Section::Settings => {
                "--color-accent:#87d1fe;--color-accent-strong:color-mix(in srgb,#87d1fe 78%,white);--color-accent-muted:color-mix(in srgb,#87d1fe 70%,black);--color-accent-soft:color-mix(in srgb,#87d1fe 14%,transparent);--rail-accent:#87d1fe;"
            }
            Section::Store => {
                "--color-accent:#3ecf8e;--color-accent-strong:color-mix(in srgb,#3ecf8e 78%,white);--color-accent-muted:color-mix(in srgb,#3ecf8e 70%,black);--color-accent-soft:color-mix(in srgb,#3ecf8e 14%,transparent);--rail-accent:#3ecf8e;"
            }
            Section::Forum => {
                "--color-accent:#5b9dff;--color-accent-strong:color-mix(in srgb,#5b9dff 78%,white);--color-accent-muted:color-mix(in srgb,#5b9dff 70%,black);--color-accent-soft:color-mix(in srgb,#5b9dff 14%,transparent);--rail-accent:#5b9dff;"
            }
            Section::Support => {
                "--color-accent:#f0a35e;--color-accent-strong:color-mix(in srgb,#f0a35e 78%,white);--color-accent-muted:color-mix(in srgb,#f0a35e 70%,black);--color-accent-soft:color-mix(in srgb,#f0a35e 14%,transparent);--rail-accent:#f0a35e;"
            }
            Section::Content => {
                "--color-accent:#f071a5;--color-accent-strong:color-mix(in srgb,#f071a5 78%,white);--color-accent-muted:color-mix(in srgb,#f071a5 70%,black);--color-accent-soft:color-mix(in srgb,#f071a5 14%,transparent);--rail-accent:#f071a5;"
            }
            Section::Players => {
                "--color-accent:#69bdf2;--color-accent-strong:color-mix(in srgb,#69bdf2 78%,white);--color-accent-muted:color-mix(in srgb,#69bdf2 70%,black);--color-accent-soft:color-mix(in srgb,#69bdf2 14%,transparent);--rail-accent:#69bdf2;"
            }
            Section::Leaderboards => {
                "--color-accent:#5eead4;--color-accent-strong:color-mix(in srgb,#5eead4 78%,white);--color-accent-muted:color-mix(in srgb,#5eead4 70%,black);--color-accent-soft:color-mix(in srgb,#5eead4 14%,transparent);--rail-accent:#5eead4;"
            }
            Section::Votes => {
                "--color-accent:#fbbf24;--color-accent-strong:color-mix(in srgb,#fbbf24 78%,white);--color-accent-muted:color-mix(in srgb,#fbbf24 70%,black);--color-accent-soft:color-mix(in srgb,#fbbf24 14%,transparent);--rail-accent:#fbbf24;"
            }
            Section::Applications => {
                "--color-accent:#fb7185;--color-accent-strong:color-mix(in srgb,#fb7185 78%,white);--color-accent-muted:color-mix(in srgb,#fb7185 70%,black);--color-accent-soft:color-mix(in srgb,#fb7185 14%,transparent);--rail-accent:#fb7185;"
            }
            Section::Analytics => {
                "--color-accent:#38bdf8;--color-accent-strong:color-mix(in srgb,#38bdf8 78%,white);--color-accent-muted:color-mix(in srgb,#38bdf8 70%,black);--color-accent-soft:color-mix(in srgb,#38bdf8 14%,transparent);--rail-accent:#38bdf8;"
            }
            Section::Account => {
                "--color-accent:#b0b3c0;--color-accent-strong:color-mix(in srgb,#b0b3c0 78%,white);--color-accent-muted:color-mix(in srgb,#b0b3c0 70%,black);--color-accent-soft:color-mix(in srgb,#b0b3c0 14%,transparent);--rail-accent:#b0b3c0;"
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
                    label: "Categories",
                    route: Route::StoreCategories {},
                },
                SubLink {
                    label: "Coupons",
                    route: Route::StoreCoupons {},
                },
                SubLink {
                    label: "Orders",
                    route: Route::StoreOrders {},
                },
                SubLink {
                    label: "Settings",
                    route: Route::StoreSettings {},
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
                    label: "Boards",
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
                    label: "Staff",
                    route: Route::AccountsStaff {},
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
                SubLink {
                    label: "Theme",
                    route: Route::SettingsTheme {},
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
        | Route::StoreProductNew {}
        | Route::StoreProductEdit { .. }
        | Route::StoreCategories {}
        | Route::StoreCategoryNew {}
        | Route::StoreCategoryEdit { .. }
        | Route::StoreCoupons {}
        | Route::StoreCouponNew {}
        | Route::StoreCouponEdit { .. }
        | Route::StoreOrders {}
        | Route::StoreSettings {} => Section::Store,
        Route::ForumOverview {}
        | Route::ForumBoards {}
        | Route::ForumBoardNew {}
        | Route::ForumBoardEdit { .. }
        | Route::ForumThreads {}
        | Route::ForumThreadNew {}
        | Route::ForumThread { .. }
        | Route::ForumModeration {}
        | Route::ForumAutoModeration {}
        | Route::ForumSiteSettings {} => Section::Forum,
        Route::SupportOverview {}
        | Route::SupportTickets {}
        | Route::SupportTicket { .. }
        | Route::SupportHelpCentre {}
        | Route::SupportHelpNew {}
        | Route::SupportHelpEdit { .. }
        | Route::SupportAutomation {}
        | Route::SupportSiteSettings {} => Section::Support,
        Route::ContentOverview {}
        | Route::ContentBlog {}
        | Route::ContentPostNew {}
        | Route::ContentPostEdit { .. }
        | Route::ContentSiteSettings {} => Section::Content,
        Route::PlayersOverview {}
        | Route::CommunityPlayers {}
        | Route::PlayersProfileDetail { .. }
        | Route::PlayersSiteSettings {} => Section::Players,
        Route::LeaderboardsOverview {}
        | Route::CommunityLeaderboards {}
        | Route::LeaderboardsBoardNew {}
        | Route::LeaderboardsBoardEdit { .. }
        | Route::LeaderboardsSiteSettings {} => Section::Leaderboards,
        Route::VotesOverview {}
        | Route::CommunityVotes {}
        | Route::VoteRewardNew {}
        | Route::VoteRewardEdit { .. }
        | Route::VotesSiteSettings {} => Section::Votes,
        Route::ApplicationsOverview {}
        | Route::CommunityApplications {}
        | Route::ApplicationReview { .. }
        | Route::ApplicationFormNew {}
        | Route::ApplicationsSiteSettings {} => Section::Applications,
        Route::AnalyticsOverview {}
        | Route::AnalyticsWebsite {}
        | Route::AnalyticsCommunity {}
        | Route::AnalyticsGaming {}
        | Route::AnalyticsSiteSettings {} => Section::Analytics,
        Route::SettingsGeneral {}
        | Route::AccountsAuth {}
        | Route::AccountsStaff {}
        | Route::AccountsRoles {}
        | Route::AccountsRoleNew {}
        | Route::SettingsLocalisation {}
        | Route::SettingsDeveloper {}
        | Route::SettingsIntegrations {}
        | Route::SettingsSecurity {}
        | Route::SettingsHosting {}
        | Route::SettingsTheme {} => Section::Settings,
        Route::Account {} => Section::Account,
        Route::AdminNotFound { segments } => match segments.first().map(String::as_str) {
            Some("store") => Section::Store,
            Some("forum") => Section::Forum,
            Some("support") => Section::Support,
            Some("blog") => Section::Content,
            Some("players") => Section::Players,
            Some("leaderboards") => Section::Leaderboards,
            Some("votes") => Section::Votes,
            Some("applications") => Section::Applications,
            Some("analytics") => Section::Analytics,
            Some("settings") => Section::Settings,
            Some("account") => Section::Account,
            Some("activity") | _ => Section::Dashboard,
        },
    }
}

pub fn subnav_active(current: &Route, target: &Route) -> bool {
    match (current, target) {
        (Route::ForumThread { .. } | Route::ForumThreadNew {}, Route::ForumThreads {}) => true,
        (Route::ForumBoardNew {} | Route::ForumBoardEdit { .. }, Route::ForumBoards {}) => true,
        (Route::SupportHelpNew {} | Route::SupportHelpEdit { .. }, Route::SupportHelpCentre {}) => {
            true
        }
        (Route::SupportTicket { .. }, Route::SupportTickets {}) => true,
        (Route::ContentPostNew {} | Route::ContentPostEdit { .. }, Route::ContentBlog {}) => true,
        (Route::StoreProductNew {} | Route::StoreProductEdit { .. }, Route::StoreProducts {}) => {
            true
        }
        (
            Route::StoreCategoryNew {} | Route::StoreCategoryEdit { .. },
            Route::StoreCategories {},
        ) => true,
        (Route::StoreCouponNew {} | Route::StoreCouponEdit { .. }, Route::StoreCoupons {}) => true,
        (Route::PlayersProfileDetail { .. }, Route::CommunityPlayers {}) => true,
        (
            Route::LeaderboardsBoardNew {} | Route::LeaderboardsBoardEdit { .. },
            Route::CommunityLeaderboards {},
        ) => true,
        (Route::VoteRewardNew {} | Route::VoteRewardEdit { .. }, Route::CommunityVotes {}) => true,
        (
            Route::ApplicationReview { .. } | Route::ApplicationFormNew {},
            Route::CommunityApplications {},
        ) => true,
        (Route::AccountsRoleNew {}, Route::AccountsRoles {}) => true,
        (current, target) => current == target,
    }
}

pub fn crumb_for(route: &Route) -> &'static str {
    match route {
        Route::Login {} => "Login",
        Route::Dashboard {} => "Overview",
        Route::DashboardActivity {} => "Activity",
        Route::StoreOverview {} => "Overview",
        Route::StoreProducts {} => "Products",
        Route::StoreProductNew {} => "New product",
        Route::StoreProductEdit { .. } => "Edit product",
        Route::StoreCategories {} => "Categories",
        Route::StoreCategoryNew {} => "New category",
        Route::StoreCategoryEdit { .. } => "Edit category",
        Route::StoreCoupons {} => "Coupons",
        Route::StoreCouponNew {} => "New coupon",
        Route::StoreCouponEdit { .. } => "Edit coupon",
        Route::StoreOrders {} => "Orders",
        Route::StoreSettings {} => "Settings",
        Route::ForumOverview {} => "Overview",
        Route::ForumBoards {} => "Boards",
        Route::ForumBoardNew {} => "New board",
        Route::ForumBoardEdit { .. } => "Edit board",
        Route::ForumThreads {} => "Threads",
        Route::ForumThreadNew {} => "New thread",
        Route::ForumThread { .. } => "Thread",
        Route::ForumModeration {} => "Moderation",
        Route::ForumAutoModeration {} => "Auto Moderation",
        Route::ForumSiteSettings {} => "Settings",
        Route::SupportOverview {} => "Overview",
        Route::SupportTickets {} => "Tickets",
        Route::SupportTicket { .. } => "Ticket",
        Route::SupportHelpCentre {} => "Help centre",
        Route::SupportHelpNew {} => "New article",
        Route::SupportHelpEdit { .. } => "Edit article",
        Route::SupportAutomation {} => "Automation",
        Route::SupportSiteSettings {} => "Settings",
        Route::ContentOverview {} => "Overview",
        Route::ContentBlog {} => "Posts",
        Route::ContentPostNew {} => "New post",
        Route::ContentPostEdit { .. } => "Edit post",
        Route::ContentSiteSettings {} => "Settings",
        Route::PlayersOverview {} => "Overview",
        Route::CommunityPlayers {} => "Profiles",
        Route::PlayersProfileDetail { .. } => "Player file",
        Route::PlayersSiteSettings {} => "Settings",
        Route::LeaderboardsOverview {} => "Overview",
        Route::CommunityLeaderboards {} => "Boards",
        Route::LeaderboardsBoardNew {} => "New board",
        Route::LeaderboardsBoardEdit { .. } => "Edit board",
        Route::LeaderboardsSiteSettings {} => "Settings",
        Route::VotesOverview {} => "Overview",
        Route::CommunityVotes {} => "Rewards",
        Route::VoteRewardNew {} => "New reward",
        Route::VoteRewardEdit { .. } => "Edit reward",
        Route::VotesSiteSettings {} => "Settings",
        Route::ApplicationsOverview {} => "Overview",
        Route::CommunityApplications {} => "Inbox",
        Route::ApplicationReview { .. } => "Review application",
        Route::ApplicationFormNew {} => "New form",
        Route::ApplicationsSiteSettings {} => "Settings",
        Route::AnalyticsOverview {} => "Overview",
        Route::AnalyticsWebsite {} => "Website",
        Route::AnalyticsCommunity {} => "Community",
        Route::AnalyticsGaming {} => "Gaming",
        Route::AnalyticsSiteSettings {} => "Settings",
        Route::SettingsGeneral {} => "General",
        Route::AccountsAuth {} => "Authentication",
        Route::AccountsStaff {} => "Staff",
        Route::AccountsRoles {} => "Roles",
        Route::AccountsRoleNew {} => "New role",
        Route::SettingsLocalisation {} => "Localisation",
        Route::SettingsDeveloper {} => "Developer",
        Route::SettingsIntegrations {} => "Integrations",
        Route::SettingsSecurity {} => "Security",
        Route::SettingsHosting {} => "Hosting",
        Route::SettingsTheme {} => "Theme",
        Route::Account {} => "Profile",
        Route::AdminNotFound { .. } => "Not found",
    }
}

pub fn is_theme_editor(route: &Route) -> bool {
    matches!(route, Route::SettingsTheme {})
}
