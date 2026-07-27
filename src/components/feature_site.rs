use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, SettingRow};
use crate::components::ui::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FeatureSite {
    Store,
    Forum,
    Support,
    Content,
    Players,
    Leaderboards,
    Votes,
    Applications,
    Analytics,
}

impl FeatureSite {
    fn label(self) -> &'static str {
        match self {
            Self::Store => "Store",
            Self::Forum => "Forum",
            Self::Support => "Support",
            Self::Content => "Blog",
            Self::Players => "Players",
            Self::Leaderboards => "Leaderboards",
            Self::Votes => "Vote rewards",
            Self::Applications => "Applications",
            Self::Analytics => "Analytics",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Store => "Domain, branding, and public URL settings for the storefront.",
            Self::Forum => "Domain and navigation settings for the community forum.",
            Self::Support => "Domain and portal settings for tickets and the help centre.",
            Self::Content => "Domain and homepage settings for the blog, news, and pages.",
            Self::Players => "Domain and profile hub settings for player pages.",
            Self::Leaderboards => "Domain and public board settings for rankings.",
            Self::Votes => "Domain and claim-page settings for vote rewards.",
            Self::Applications => "Domain and form settings for staff applications.",
            Self::Analytics => "Domain and access settings for shared analytics views.",
        }
    }

    fn defaults(self) -> SiteDefaults {
        match self {
            Self::Store => SiteDefaults {
                domain: "store.example.com",
                subdomain: "store",
                base_path: "/",
                site_title: "NovaCraft Store",
                primary_nav: "Shop, Ranks, Crates, Gifts",
            },
            Self::Forum => SiteDefaults {
                domain: "forum.example.com",
                subdomain: "forum",
                base_path: "/",
                site_title: "NovaCraft Forums",
                primary_nav: "Categories, Unread, Members",
            },
            Self::Support => SiteDefaults {
                domain: "support.example.com",
                subdomain: "support",
                base_path: "/",
                site_title: "NovaCraft Support",
                primary_nav: "Tickets, Help centre, Status",
            },
            Self::Content => SiteDefaults {
                domain: "www.example.com",
                subdomain: "www",
                base_path: "/news",
                site_title: "NovaCraft Blog",
                primary_nav: "News, Blog, Rules, Staff",
            },
            Self::Players => SiteDefaults {
                domain: "players.example.com",
                subdomain: "players",
                base_path: "/",
                site_title: "NovaCraft Players",
                primary_nav: "Profiles, Search, Badges",
            },
            Self::Leaderboards => SiteDefaults {
                domain: "boards.example.com",
                subdomain: "boards",
                base_path: "/",
                site_title: "NovaCraft Leaderboards",
                primary_nav: "Top players, Kills, Playtime",
            },
            Self::Votes => SiteDefaults {
                domain: "vote.example.com",
                subdomain: "vote",
                base_path: "/",
                site_title: "NovaCraft Vote Rewards",
                primary_nav: "Vote links, Streaks, Claim",
            },
            Self::Applications => SiteDefaults {
                domain: "apply.example.com",
                subdomain: "apply",
                base_path: "/",
                site_title: "NovaCraft Applications",
                primary_nav: "Open roles, My applications",
            },
            Self::Analytics => SiteDefaults {
                domain: "insights.example.com",
                subdomain: "insights",
                base_path: "/",
                site_title: "NovaCraft Analytics",
                primary_nav: "Overview, Reports, Exports",
            },
        }
    }
}

#[derive(Clone, Copy)]
struct SiteDefaults {
    domain: &'static str,
    subdomain: &'static str,
    base_path: &'static str,
    site_title: &'static str,
    primary_nav: &'static str,
}

#[component]
pub fn StoreSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Store } }
}

#[component]
pub fn ForumSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Forum } }
}

#[component]
pub fn SupportSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Support } }
}

#[component]
pub fn ContentSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Content } }
}

#[component]
pub fn PlayersSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Players } }
}

#[component]
pub fn LeaderboardsSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Leaderboards } }
}

#[component]
pub fn VotesSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Votes } }
}

#[component]
pub fn ApplicationsSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Applications } }
}

#[component]
pub fn AnalyticsSiteSettings() -> Element {
    rsx! { FeatureSiteSettings { feature: FeatureSite::Analytics } }
}

#[component]
fn FeatureSiteSettings(feature: FeatureSite) -> Element {
    let defaults = feature.defaults();

    rsx! {
        PageHeader {
            title: "Settings",
            subtitle: feature.subtitle(),
            action: rsx! {
                Button { "Save changes" }
            },
        }

        div {
            class: "mb-4 flex flex-wrap items-center gap-2",
            span {
                class: "rounded-squircle-sm border border-border-subtle bg-surface/40 px-2.5 py-1 text-xs text-text-muted",
                "{feature.label()} site"
            }
            span {
                class: "text-xs text-text-muted",
                "Domain and branding for this feature only"
            }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Domain",
                SettingsField { label: "Custom domain", value: defaults.domain }
                SettingsField { label: "Subdomain", value: defaults.subdomain }
                SettingsField { label: "Base path", value: defaults.base_path }
                SettingRow {
                    title: "Force HTTPS",
                    description: "Redirect all traffic on this feature domain to HTTPS.",
                    enabled: true,
                }
            }
            DataPanel {
                title: "Branding & navigation",
                SettingsField { label: "Site title", value: defaults.site_title }
                SettingsField { label: "Primary navigation", value: defaults.primary_nav }
                SettingRow {
                    title: "Custom branding",
                    description: "Override logo, favicon, and accent colors for this feature.",
                    enabled: true,
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Use a dedicated menu instead of the shared site nav.",
                    enabled: true,
                }
                SettingRow {
                    title: "Custom layout",
                    description: "Enable feature-specific page layouts and templates.",
                    enabled: false,
                }
            }
        }
    }
}

#[component]
fn SettingsField(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            class: "border-b border-border-subtle py-3 last:border-0",
            label { class: "mb-1.5 block text-xs font-medium text-text-muted", "{label}" }
            StaticInput { value, class: "max-w-md" }
        }
    }
}
