use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, SettingRow, StatPill};
use crate::components::ui::*;

#[component]
pub fn SettingsGeneral() -> Element {
    rsx! {
        PageHeader {
            title: "General",
            subtitle: "Site name, registration, localization, and branding defaults.",
            action: rsx! {
                Button { "Save changes" }
            },
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Site",
                SettingRow {
                    title: "Maintenance mode",
                    description: "Show a maintenance page to non-staff visitors.",
                    enabled: false,
                }
                SettingRow {
                    title: "User registration",
                    description: "Allow new players to create website accounts.",
                    enabled: true,
                }
                SettingRow {
                    title: "Email verification",
                    description: "Require verified email before purchases.",
                    enabled: true,
                }
            }
            DataPanel {
                title: "Localization",
                SettingRow {
                    title: "Multi-language site",
                    description: "Serve content in 20+ languages.",
                    enabled: true,
                }
                SettingRow {
                    title: "Multi-currency store",
                    description: "Show prices in the visitor’s preferred currency.",
                    enabled: true,
                }
                div {
                    class: "pt-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        IconGlobe {}
                        "Manage languages"
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingsIntegrations() -> Element {
    rsx! {
        PageHeader {
            title: "Integrations",
            subtitle: "Connect Discord, Tebex, payment providers, and third-party tools.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Add integration"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Connected", value: "6", accent: "#7b8cff" }
            StatPill { label: "Webhooks", value: "11", accent: "#5b9dff" }
            StatPill { label: "Linked accounts", value: "892", accent: "#3ecf8e" }
            StatPill { label: "Failed syncs", value: "2", accent: "#f0a35e" }
        }

        DataPanel {
            title: "Services",
            SettingRow {
                title: "Discord",
                description: "Login, role rewards, and purchase announcements.",
                enabled: true,
            }
            SettingRow {
                title: "Tebex checkout",
                description: "Headless storefront with global payment methods.",
                enabled: true,
            }
            SettingRow {
                title: "Google Analytics",
                description: "Track traffic and conversion funnels on your site.",
                enabled: false,
            }
            SettingRow {
                title: "Tawk.to live chat",
                description: "Chat with visitors directly from your website.",
                enabled: false,
            }
        }
    }
}

#[component]
pub fn SettingsSecurity() -> Element {
    rsx! {
        PageHeader {
            title: "Security",
            subtitle: "Authentication, spam protection, and privacy controls.",
        }
        DataPanel {
            title: "Security & privacy",
            SettingRow {
                title: "Two-factor authentication",
                description: "Encourage 2FA for staff and high-value accounts.",
                enabled: true,
            }
            SettingRow {
                title: "CAPTCHA on forms",
                description: "Protect login, register, and ticket forms from spam.",
                enabled: true,
            }
            SettingRow {
                title: "Cookie consent",
                description: "Ask visitors for permission before storing cookies.",
                enabled: false,
            }
        }
    }
}

#[component]
pub fn SettingsHosting() -> Element {
    rsx! {
        PageHeader {
            title: "Hosting",
            subtitle: "Cloud hosting, backups, and deployment for this website.",
        }
        DataPanel {
            title: "Hosting",
            SettingRow {
                title: "Cloud hosting",
                description: "Managed ServerSpot cloud for this website.",
                enabled: true,
            }
            SettingRow {
                title: "Automatic backups",
                description: "Nightly snapshots of your site and database.",
                enabled: true,
            }
            div {
                class: "pt-2",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    "Open hosting panel"
                }
            }
        }
    }
}
