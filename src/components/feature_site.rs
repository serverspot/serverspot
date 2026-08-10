use dioxus::prelude::*;

use crate::components::community::{PlayersLeaderboardsStyles, VotesApplicationsStyles};
use crate::components::page::{DataPanel, FeatureSettingsChrome, SettingRow, SettingsField};

#[component]
pub fn PlayersSiteSettings() -> Element {
    rsx! {
        PlayersLeaderboardsStyles {}
        FeatureSettingsChrome { subtitle: "Path and profile hub settings for the player directory.",
            DataPanel { title: "On your website",
                SettingsField { label: "Public path", value: "/players" }
                SettingsField { label: "Full URL", value: "www.example.com/players" }
                SettingsField {
                    label: "Profile URL pattern",
                    value: "www.example.com/players/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Directory behaviour",
                SettingRow {
                    title: "Public roster search",
                    description: "Let visitors search and filter the player directory.",
                    enabled: true,
                }
                SettingRow {
                    title: "Show ranks",
                    description: "Display rank badges on profiles and the roster.",
                    enabled: true,
                }
                SettingRow {
                    title: "Show linked accounts",
                    description: "Reveal linked Minecraft and Discord identities publicly.",
                    enabled: true,
                }
                SettingRow {
                    title: "Public profile pages",
                    description: "Allow anyone to view a player's case file page.",
                    enabled: true,
                }
            }
            DataPanel { title: "Branding & navigation",
                SettingsField { label: "Page title", value: "Players" }
                SettingsField {
                    label: "Section navigation",
                    value: "Profiles, Search, Badges",
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Show a feature menu instead of the default website nav.",
                    enabled: false,
                }
            }
            DataPanel { title: "Roster defaults",
                SettingsField { label: "Default sort", value: "Highest level" }
                SettingsField { label: "Cards per page", value: "24" }
                SettingsField { label: "Offline grace period", value: "5 minutes" }
            }
        }
    }
}

#[component]
pub fn LeaderboardsSiteSettings() -> Element {
    rsx! {
        PlayersLeaderboardsStyles {}
        FeatureSettingsChrome { subtitle: "Path and public board settings for rankings and podiums.",
            DataPanel { title: "On your website",
                SettingsField { label: "Public path", value: "/leaderboards" }
                SettingsField { label: "Full URL", value: "www.example.com/leaderboards" }
                SettingsField {
                    label: "Board URL pattern",
                    value: "www.example.com/leaderboards/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Board behaviour",
                SettingRow {
                    title: "Public boards",
                    description: "Show leaderboards on the public website.",
                    enabled: true,
                }
                SettingRow {
                    title: "Show avatars",
                    description: "Display player avatars next to ranked entries.",
                    enabled: true,
                }
                SettingRow {
                    title: "Automatic resets",
                    description: "Reset boards automatically based on their schedule.",
                    enabled: true,
                }
                SettingRow {
                    title: "Podium spotlight widget",
                    description: "Show the top-3 podium widget on the overview page.",
                    enabled: true,
                }
            }
            DataPanel { title: "Branding & navigation",
                SettingsField { label: "Page title", value: "Leaderboards" }
                SettingsField {
                    label: "Section navigation",
                    value: "Top players, Kills, Playtime",
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Show a feature menu instead of the default website nav.",
                    enabled: false,
                }
            }
            DataPanel { title: "Data sources",
                SettingsField { label: "Primary source", value: "In-game plugin API" }
                SettingsField { label: "Sync interval", value: "5 minutes" }
                SettingsField { label: "Fallback source", value: "Manual CSV import" }
            }
        }
    }
}

#[component]
pub fn VotesSiteSettings() -> Element {
    rsx! {
        VotesApplicationsStyles {}
        FeatureSettingsChrome { subtitle: "Listing sites, callbacks, and claim delivery for vote rewards.",
            DataPanel { title: "Listing sites & callbacks",
                SettingsField { label: "Connected sites", value: "4 listing sites" }
                SettingsField { label: "Callback timeout", value: "10 seconds" }
                SettingsField { label: "Vote cooldown", value: "12–24 hours per site" }
                SettingsField {
                    label: "Callback URL",
                    value: "www.example.com/vote/callback/:site",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Each listing site posts to this callback when a vote is verified."
                }
            }
            DataPanel { title: "Claim delivery",
                SettingRow {
                    title: "Auto-claim online players",
                    description: "Run reward commands immediately if the player is online.",
                    enabled: true,
                }
                SettingRow {
                    title: "Queue offline claims",
                    description: "Hold claims until the player next joins the server.",
                    enabled: true,
                }
                SettingRow {
                    title: "Streak grace period",
                    description: "Allow a 24-hour grace period before a streak resets.",
                    enabled: true,
                }
            }
            DataPanel { title: "On your website",
                SettingsField { label: "Public path", value: "/vote" }
                SettingsField { label: "Full URL", value: "www.example.com/vote" }
                SettingsField {
                    label: "Claim URL pattern",
                    value: "www.example.com/vote/claim/:id",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Branding & navigation",
                SettingsField { label: "Page title", value: "Vote rewards" }
                SettingsField {
                    label: "Section navigation",
                    value: "Vote links, Rewards, Claim",
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Show a feature menu instead of the default website nav.",
                    enabled: false,
                }
            }
        }
    }
}

#[component]
pub fn ApplicationsSiteSettings() -> Element {
    rsx! {
        VotesApplicationsStyles {}
        FeatureSettingsChrome { subtitle: "Path, form defaults, and review workflow for applications.",
            DataPanel { title: "On your website",
                SettingsField { label: "Public path", value: "/apply" }
                SettingsField { label: "Full URL", value: "www.example.com/apply" }
                SettingsField {
                    label: "Form URL pattern",
                    value: "www.example.com/apply/:role",
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Desk behaviour",
                SettingRow {
                    title: "Open applications publicly",
                    description: "Let visitors browse open roles and apply.",
                    enabled: true,
                }
                SettingRow {
                    title: "Require account link",
                    description: "Applicants must link a Minecraft account to apply.",
                    enabled: true,
                }
                SettingRow {
                    title: "Staff voting",
                    description: "Allow reviewers to cast yes/no votes on each application.",
                    enabled: true,
                }
                SettingRow {
                    title: "Auto-close duplicate applications",
                    description: "Archive earlier applications when a newer one is submitted.",
                    enabled: false,
                }
            }
            DataPanel { title: "Branding & navigation",
                SettingsField { label: "Page title", value: "Applications" }
                SettingsField {
                    label: "Section navigation",
                    value: "Open roles, My applications",
                }
                SettingRow {
                    title: "Custom navigation",
                    description: "Show a feature menu instead of the default website nav.",
                    enabled: false,
                }
            }
            DataPanel { title: "Review workflow",
                SettingsField { label: "Reviewers per application", value: "2 minimum" }
                SettingsField { label: "Decision notifications", value: "Email + Discord" }
                SettingsField { label: "Reapply cooldown", value: "30 days after denial" }
            }
        }
    }
}
