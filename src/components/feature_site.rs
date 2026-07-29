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
            Self::Store => {
                "Path, branding, and navigation for the storefront on your website."
            }
            Self::Forum => "Path and navigation settings for the community forum.",
            Self::Support => "Path and portal settings for tickets and the help centre.",
            Self::Content => "Path and homepage settings for the blog, news, and pages.",
            Self::Players => "Path and profile hub settings for player pages.",
            Self::Leaderboards => "Path and public board settings for rankings.",
            Self::Votes => "Path and claim-page settings for vote rewards.",
            Self::Applications => "Path and form settings for staff applications.",
            Self::Analytics => "Path and access settings for shared analytics views.",
        }
    }
    fn defaults(self) -> FeatureDefaults {
        match self {
            Self::Store => {
                FeatureDefaults {
                    base_path: "/store",
                    page_title: "Store",
                    primary_nav: "Shop, Ranks, Crates, Gifts",
                }
            }
            Self::Forum => {
                FeatureDefaults {
                    base_path: "/forum",
                    page_title: "Forums",
                    primary_nav: "Categories, Unread, Members",
                }
            }
            Self::Support => {
                FeatureDefaults {
                    base_path: "/support",
                    page_title: "Support",
                    primary_nav: "Tickets, Help centre, Status",
                }
            }
            Self::Content => {
                FeatureDefaults {
                    base_path: "/news",
                    page_title: "Blog",
                    primary_nav: "News, Blog, Rules, Staff",
                }
            }
            Self::Players => {
                FeatureDefaults {
                    base_path: "/players",
                    page_title: "Players",
                    primary_nav: "Profiles, Search, Badges",
                }
            }
            Self::Leaderboards => {
                FeatureDefaults {
                    base_path: "/leaderboards",
                    page_title: "Leaderboards",
                    primary_nav: "Top players, Kills, Playtime",
                }
            }
            Self::Votes => {
                FeatureDefaults {
                    base_path: "/vote",
                    page_title: "Vote rewards",
                    primary_nav: "Vote links, Streaks, Claim",
                }
            }
            Self::Applications => {
                FeatureDefaults {
                    base_path: "/apply",
                    page_title: "Applications",
                    primary_nav: "Open roles, My applications",
                }
            }
            Self::Analytics => {
                FeatureDefaults {
                    base_path: "/analytics",
                    page_title: "Analytics",
                    primary_nav: "Overview, Reports, Exports",
                }
            }
        }
    }
}
#[derive(Clone, Copy)]
struct FeatureDefaults {
    base_path: &'static str,
    page_title: &'static str,
    primary_nav: &'static str,
}
#[component]
pub fn StoreSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Store }
    }
}
#[component]
pub fn ForumSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Forum }
    }
}
#[component]
pub fn SupportSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Support }
    }
}
#[component]
pub fn ContentSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Content }
    }
}
#[component]
pub fn PlayersSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Players }
    }
}
#[component]
pub fn LeaderboardsSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Leaderboards }
    }
}
#[component]
pub fn VotesSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Votes }
    }
}
#[component]
pub fn ApplicationsSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Applications }
    }
}
#[component]
pub fn AnalyticsSiteSettings() -> Element {
    rsx! {
        FeatureSiteSettings { feature: FeatureSite::Analytics }
    }
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
        div { class: "mb-4 flex flex-wrap items-center gap-2",
            span { class: "rounded-squircle-sm border border-border-subtle bg-surface/40 px-2.5 py-1 text-xs text-text-muted",
                "{feature.label()} feature"
            }
            span { class: "text-xs text-text-muted", "Uses your main website domain · www.example.com" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "On your website",
                SettingsField { label: "Public path", value: defaults.base_path }
                SettingsField {
                    label: "Full URL",
                    value: match feature {
                        FeatureSite::Store => "www.example.com/store",
                        FeatureSite::Forum => "www.example.com/forum",
                        FeatureSite::Support => "www.example.com/support",
                        FeatureSite::Content => "www.example.com/news",
                        FeatureSite::Players => "www.example.com/players",
                        FeatureSite::Leaderboards => "www.example.com/leaderboards",
                        FeatureSite::Votes => "www.example.com/vote",
                        FeatureSite::Applications => "www.example.com/apply",
                        FeatureSite::Analytics => "www.example.com/analytics",
                    },
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Branding & navigation",
                SettingsField { label: "Page title", value: defaults
                            .page_title }
                SettingsField { label: "Section navigation", value: defaults
                            .primary_nav }
                SettingRow {
                    title: "Custom branding",
                    description: "Override logo and accent colors for this feature’s pages.",
                    enabled: true,
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Show a feature menu instead of the default website nav.",
                    enabled: false,
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
        div { class: "border-b border-border-subtle py-3 last:border-0",
            label { class: "mb-1.5 block text-xs font-medium text-text-muted", "{label}" }
            StaticInput { value, class: "max-w-md" }
        }
    }
}
