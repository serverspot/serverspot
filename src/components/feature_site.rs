use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::community::{PlayersLeaderboardsStyles, VotesApplicationsStyles};
use crate::components::page::{DataPanel, FeatureSettingsChrome, SettingRow, SettingsField};
use crate::i18n::t_key;

#[component]
pub fn PlayersSiteSettings() -> Element {
    let _lang = i18n();

    rsx! {
        PlayersLeaderboardsStyles {}
        FeatureSettingsChrome { subtitle: t_key("feature-players-subtitle"),
            DataPanel { title: t_key("feature-on-website"),
                SettingsField { label: t_key("feature-public-path"), value: "/players" }
                SettingsField { label: t_key("feature-full-url"), value: "www.example.com/players" }
                SettingsField {
                    label: t_key("feature-players-profile-url-pattern"),
                    value: "www.example.com/players/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("feature-domain-hint") }
                }
            }
            DataPanel { title: t_key("feature-players-directory-behaviour"),
                SettingRow {
                    title: t_key("feature-players-roster-search-title"),
                    description: t_key("feature-players-roster-search-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-players-show-ranks-title"),
                    description: t_key("feature-players-show-ranks-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-players-linked-accounts-title"),
                    description: t_key("feature-players-linked-accounts-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-players-public-profiles-title"),
                    description: t_key("feature-players-public-profiles-desc"),
                    enabled: true,
                }
            }
            DataPanel { title: t_key("feature-branding-navigation"),
                SettingsField { label: t_key("feature-page-title"), value: t_key("feature-example-page-title-players") }
                SettingsField {
                    label: t_key("feature-section-navigation"),
                    value: "Profiles, Search, Badges",
                }
                SettingRow {
                    title: t_key("feature-custom-nav-title"),
                    description: t_key("feature-custom-nav-desc"),
                    enabled: false,
                }
            }
            DataPanel { title: t_key("feature-players-roster-defaults"),
                SettingsField { label: t_key("feature-players-default-sort"), value: t_key("feature-example-sort-highest-level") }
                SettingsField { label: t_key("feature-players-cards-per-page"), value: "24" }
                SettingsField { label: t_key("feature-players-offline-grace"), value: "5 minutes" }
            }
        }
    }
}

#[component]
pub fn LeaderboardsSiteSettings() -> Element {
    let _lang = i18n();

    rsx! {
        PlayersLeaderboardsStyles {}
        FeatureSettingsChrome { subtitle: t_key("feature-leaderboards-subtitle"),
            DataPanel { title: t_key("feature-on-website"),
                SettingsField { label: t_key("feature-public-path"), value: "/leaderboards" }
                SettingsField { label: t_key("feature-full-url"), value: "www.example.com/leaderboards" }
                SettingsField {
                    label: t_key("feature-leaderboards-board-url-pattern"),
                    value: "www.example.com/leaderboards/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("feature-domain-hint") }
                }
            }
            DataPanel { title: t_key("feature-leaderboards-board-behaviour"),
                SettingRow {
                    title: t_key("feature-leaderboards-public-boards-title"),
                    description: t_key("feature-leaderboards-public-boards-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-leaderboards-show-avatars-title"),
                    description: t_key("feature-leaderboards-show-avatars-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-leaderboards-auto-resets-title"),
                    description: t_key("feature-leaderboards-auto-resets-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-leaderboards-podium-widget-title"),
                    description: t_key("feature-leaderboards-podium-widget-desc"),
                    enabled: true,
                }
            }
            DataPanel { title: t_key("feature-branding-navigation"),
                SettingsField { label: t_key("feature-page-title"), value: t_key("feature-example-page-title-leaderboards") }
                SettingsField {
                    label: t_key("feature-section-navigation"),
                    value: "Top players, Kills, Playtime",
                }
                SettingRow {
                    title: t_key("feature-custom-nav-title"),
                    description: t_key("feature-custom-nav-desc"),
                    enabled: false,
                }
            }
            DataPanel { title: t_key("feature-leaderboards-data-sources"),
                SettingsField { label: t_key("feature-leaderboards-primary-source"), value: t_key("feature-example-source-plugin-api") }
                SettingsField { label: t_key("feature-leaderboards-sync-interval"), value: "5 minutes" }
                SettingsField { label: t_key("feature-leaderboards-fallback-source"), value: t_key("feature-example-source-csv-import") }
            }
        }
    }
}

#[component]
pub fn VotesSiteSettings() -> Element {
    let _lang = i18n();

    rsx! {
        VotesApplicationsStyles {}
        FeatureSettingsChrome { subtitle: t_key("feature-votes-subtitle"),
            DataPanel { title: t_key("feature-votes-listing-callbacks"),
                SettingsField { label: t_key("feature-votes-connected-sites"), value: "4 listing sites" }
                SettingsField { label: t_key("feature-votes-callback-timeout"), value: "10 seconds" }
                SettingsField { label: t_key("feature-votes-vote-cooldown"), value: "12–24 hours per site" }
                SettingsField {
                    label: t_key("feature-votes-callback-url"),
                    value: "www.example.com/vote/callback/:site",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("feature-votes-callback-hint") }
                }
            }
            DataPanel { title: t_key("feature-votes-claim-delivery"),
                SettingRow {
                    title: t_key("feature-votes-auto-claim-title"),
                    description: t_key("feature-votes-auto-claim-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-votes-queue-offline-title"),
                    description: t_key("feature-votes-queue-offline-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-votes-streak-grace-title"),
                    description: t_key("feature-votes-streak-grace-desc"),
                    enabled: true,
                }
            }
            DataPanel { title: t_key("feature-on-website"),
                SettingsField { label: t_key("feature-public-path"), value: "/vote" }
                SettingsField { label: t_key("feature-full-url"), value: "www.example.com/vote" }
                SettingsField {
                    label: t_key("feature-votes-claim-url-pattern"),
                    value: "www.example.com/vote/claim/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("feature-domain-hint") }
                }
            }
            DataPanel { title: t_key("feature-branding-navigation"),
                SettingsField { label: t_key("feature-page-title"), value: t_key("feature-example-page-title-votes") }
                SettingsField {
                    label: t_key("feature-section-navigation"),
                    value: "Vote links, Rewards, Claim",
                }
                SettingRow {
                    title: t_key("feature-custom-nav-title"),
                    description: t_key("feature-custom-nav-desc"),
                    enabled: false,
                }
            }
        }
    }
}

#[component]
pub fn ApplicationsSiteSettings() -> Element {
    let _lang = i18n();

    rsx! {
        VotesApplicationsStyles {}
        FeatureSettingsChrome { subtitle: t_key("feature-applications-subtitle"),
            DataPanel { title: t_key("feature-on-website"),
                SettingsField { label: t_key("feature-public-path"), value: "/apply" }
                SettingsField { label: t_key("feature-full-url"), value: "www.example.com/apply" }
                SettingsField {
                    label: t_key("feature-applications-form-url-pattern"),
                    value: "www.example.com/apply/:role",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    { t!("feature-domain-hint") }
                }
            }
            DataPanel { title: t_key("feature-applications-desk-behaviour"),
                SettingRow {
                    title: t_key("feature-applications-open-publicly-title"),
                    description: t_key("feature-applications-open-publicly-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-applications-require-account-title"),
                    description: t_key("feature-applications-require-account-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-applications-staff-voting-title"),
                    description: t_key("feature-applications-staff-voting-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("feature-applications-auto-close-title"),
                    description: t_key("feature-applications-auto-close-desc"),
                    enabled: false,
                }
            }
            DataPanel { title: t_key("feature-branding-navigation"),
                SettingsField { label: t_key("feature-page-title"), value: t_key("feature-example-page-title-applications") }
                SettingsField {
                    label: t_key("feature-section-navigation"),
                    value: "Open roles, My applications",
                }
                SettingRow {
                    title: t_key("feature-custom-nav-title"),
                    description: t_key("feature-custom-nav-desc"),
                    enabled: false,
                }
            }
            DataPanel { title: t_key("feature-applications-review-workflow"),
                SettingsField { label: t_key("feature-applications-reviewers-min"), value: "2 minimum" }
                SettingsField { label: t_key("feature-applications-decision-notifications"), value: t_key("feature-example-notifications") }
                SettingsField { label: t_key("feature-applications-reapply-cooldown"), value: "30 days after denial" }
            }
        }
    }
}
