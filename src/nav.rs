use crate::router::Route;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Dashboard,
    Store,
    Forum,
    Support,
    Content,
    Community,
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
        Section::Community,
        Section::Analytics,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Dashboard => "Dashboard",
            Section::Store => "Store",
            Section::Forum => "Forum",
            Section::Support => "Support",
            Section::Content => "Content",
            Section::Community => "Community",
            Section::Analytics => "Analytics",
            Section::Settings => "Settings",
        }
    }

    pub fn home(self) -> Route {
        match self {
            Section::Dashboard => Route::Dashboard {},
            Section::Store => Route::StoreProducts {},
            Section::Forum => Route::ForumCategories {},
            Section::Support => Route::SupportTickets {},
            Section::Content => Route::ContentBlog {},
            Section::Community => Route::CommunityPlayers {},
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
                    label: "Products",
                    route: Route::StoreProducts {},
                },
                SubLink {
                    label: "Orders",
                    route: Route::StoreOrders {},
                },
                SubLink {
                    label: "Coupons",
                    route: Route::StoreCoupons {},
                },
                SubLink {
                    label: "Gift cards",
                    route: Route::StoreGifts {},
                },
            ],
            Section::Forum => &[
                SubLink {
                    label: "Categories",
                    route: Route::ForumCategories {},
                },
                SubLink {
                    label: "Threads",
                    route: Route::ForumThreads {},
                },
                SubLink {
                    label: "Roles",
                    route: Route::ForumRoles {},
                },
                SubLink {
                    label: "Awards",
                    route: Route::ForumAwards {},
                },
            ],
            Section::Support => &[
                SubLink {
                    label: "Tickets",
                    route: Route::SupportTickets {},
                },
                SubLink {
                    label: "Departments",
                    route: Route::SupportDepartments {},
                },
                SubLink {
                    label: "Automation",
                    route: Route::SupportAutomation {},
                },
            ],
            Section::Content => &[
                SubLink {
                    label: "Blog",
                    route: Route::ContentBlog {},
                },
                SubLink {
                    label: "Help center",
                    route: Route::ContentHelp {},
                },
                SubLink {
                    label: "Pages",
                    route: Route::ContentPages {},
                },
                SubLink {
                    label: "Announcements",
                    route: Route::ContentAnnouncements {},
                },
            ],
            Section::Community => &[
                SubLink {
                    label: "Players",
                    route: Route::CommunityPlayers {},
                },
                SubLink {
                    label: "Leaderboards",
                    route: Route::CommunityLeaderboards {},
                },
                SubLink {
                    label: "Vote rewards",
                    route: Route::CommunityVotes {},
                },
                SubLink {
                    label: "Applications",
                    route: Route::CommunityApplications {},
                },
            ],
            Section::Analytics => &[
                SubLink {
                    label: "Overview",
                    route: Route::AnalyticsOverview {},
                },
                SubLink {
                    label: "Revenue",
                    route: Route::AnalyticsRevenue {},
                },
                SubLink {
                    label: "Traffic",
                    route: Route::AnalyticsTraffic {},
                },
            ],
            Section::Settings => &[
                SubLink {
                    label: "General",
                    route: Route::SettingsGeneral {},
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
        Route::Dashboard {} | Route::DashboardActivity {} => Section::Dashboard,
        Route::StoreProducts {}
        | Route::StoreOrders {}
        | Route::StoreCoupons {}
        | Route::StoreGifts {} => Section::Store,
        Route::ForumCategories {}
        | Route::ForumThreads {}
        | Route::ForumRoles {}
        | Route::ForumAwards {} => Section::Forum,
        Route::SupportTickets {} | Route::SupportDepartments {} | Route::SupportAutomation {} => {
            Section::Support
        }
        Route::ContentBlog {}
        | Route::ContentHelp {}
        | Route::ContentPages {}
        | Route::ContentAnnouncements {} => Section::Content,
        Route::CommunityPlayers {}
        | Route::CommunityLeaderboards {}
        | Route::CommunityVotes {}
        | Route::CommunityApplications {} => Section::Community,
        Route::AnalyticsOverview {} | Route::AnalyticsRevenue {} | Route::AnalyticsTraffic {} => {
            Section::Analytics
        }
        Route::SettingsGeneral {}
        | Route::SettingsIntegrations {}
        | Route::SettingsSecurity {}
        | Route::SettingsHosting {} => Section::Settings,
        _ => Section::Dashboard,
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
