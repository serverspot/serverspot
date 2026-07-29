use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill,
    StatusChip,
};
use crate::components::ui::*;
#[component]
pub fn SettingsGeneral() -> Element {
    rsx! {
        PageHeader {
            title: "General",
            subtitle: "Website domain, registration, and branding defaults.",
            action: rsx! {
                Button { "Save changes" }
            },
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Website",
                SettingsField { label: "Site name", value: "NovaCraft" }
                SettingsField { label: "Custom domain", value: "www.example.com" }
                SettingsField { label: "Subdomain", value: "novacraft" }
                SettingRow {
                    title: "Force HTTPS",
                    description: "Redirect all traffic on your website to HTTPS.",
                    enabled: true,
                }
            }
            DataPanel { title: "Site options",
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
#[component]
pub fn SettingsLocalisation() -> Element {
    rsx! {
        PageHeader {
            title: "Localisation",
            subtitle: "Multi-language support, formatting, and translation management.",
            action: rsx!
                    { Button { IconPlus {} "Add language" } }
                Button {
                    IconPlus {}
                    "Add language"
                }
            },
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Languages", value: "12", accent: "#34d399" }
            StatPill { label: "Translated keys", value: "94%", accent: "#3ecf8e" }
            StatPill { label: "Currencies", value: "8", accent: "#5b9dff" }
            StatPill { label: "RTL locales", value: "2", accent: "#87d1fe" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Localisation features",
                FeatureBullets {
                    FeatureBullet { text: "Multiple languages" }
                    FeatureBullet { text: "Translation management" }
                    FeatureBullet { text: "User language preferences" }
                    FeatureBullet { text: "Date formatting" }
                    FeatureBullet { text: "Timezone support" }
                    FeatureBullet { text: "Currency formatting" }
                    FeatureBullet { text: "RTL support" }
                }
            }
            DataPanel { title: "Admin tools",
                RowItem {
                    title: "English (UK)",
                    meta: "Default · 100% complete",
                    trailing: "Edit",
                }
                RowItem {
                    title: "Spanish",
                    meta: "User preference enabled · 96%",
                    trailing: "Edit",
                }
                RowItem {
                    title: "Arabic",
                    meta: "RTL · 88% complete",
                    trailing: "Edit",
                }
                RowItem {
                    title: "German",
                    meta: "Currency: EUR · 91%",
                    trailing: "Edit",
                }
                div { class: "mt-3 flex flex-wrap gap-2",
                    StatusChip { label: "Import", tone: "#5b9dff" }
                    StatusChip { label: "Export", tone: "#87d1fe" }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        IconGlobe {}
                        "Manage translations"
                    }
                }
            }
        }
    }
}
#[component]
pub fn SettingsDeveloper() -> Element {
    rsx! {
        PageHeader {
            title: "Developer platform",
            subtitle: "Plugins, APIs, webhooks, and tools for extending ServerSpot.",
            action: rsx! {
                Button {
                    IconPlus {}
                    "Create API key"
                }
            },
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Plugins", value: "7", accent: "#5b9dff" }
            StatPill { label: "Webhooks", value: "11", accent: "#87d1fe" }
            StatPill { label: "API calls / day", value: "18.2k", accent: "#3ecf8e" }
            StatPill { label: "Modules", value: "4", accent: "#f0a35e" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Platform features",
                FeatureBullets {
                    FeatureBullet { text: "Plugin system" }
                    FeatureBullet { text: "Public API" }
                    FeatureBullet { text: "Webhooks" }
                    FeatureBullet { text: "Events" }
                    FeatureBullet { text: "Custom modules" }
                    FeatureBullet { text: "Theme marketplace" }
                }
            }
            DataPanel { title: "Developers can create",
                FeatureBullets {
                    FeatureBullet { text: "New features" }
                    FeatureBullet { text: "Integrations" }
                    FeatureBullet { text: "Themes" }
                    FeatureBullet { text: "Server plugins" }
                }
                div { class: "mt-4 space-y-0",
                    RowItem {
                        title: "order.completed",
                        meta: "Webhook · www.example.com/hooks",
                        trailing: "Active",
                    }
                    RowItem {
                        title: "ticket.created",
                        meta: "Webhook · Discord bridge",
                        trailing: "Active",
                    }
                    RowItem {
                        title: "vote.claimed",
                        meta: "Event listener · reward module",
                        trailing: "Active",
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
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Connected", value: "6", accent: "#7b8cff" }
            StatPill { label: "Webhooks", value: "11", accent: "#5b9dff" }
            StatPill { label: "Linked accounts", value: "892", accent: "#3ecf8e" }
            StatPill { label: "Failed syncs", value: "2", accent: "#f0a35e" }
        }
        DataPanel { title: "Services",
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
        DataPanel { title: "Security & privacy",
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
        DataPanel { title: "Hosting",
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
            div { class: "pt-2",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Open hosting panel" }
            }
        }
    }
}
