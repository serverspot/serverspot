use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill, StatusChip,
};
use crate::components::ui::*;
use crate::server_funcs::{upload_site_favicon, upload_site_logo};

#[component]
pub fn SettingsGeneral() -> Element {
    let mut logo_revision = use_signal(|| 0u64);
    let mut logo_status = use_signal(String::new);
    let mut uploading_logo = use_signal(|| false);
    let mut favicon_revision = use_signal(|| 0u64);
    let mut favicon_status = use_signal(String::new);
    let mut uploading_favicon = use_signal(|| false);
    let logo_url = format!("/uploads/site-logo?v={}", logo_revision());
    let favicon_url = format!("/uploads/site-favicon?v={}", favicon_revision());
    let logo_template_hint = ["{", "{", " site.logo ", "}", "}", " → /uploads/site-logo"].concat();

    rsx! {
        PageHeader {
            title: "General",
            subtitle: "Website domain, registration, and branding defaults.",
            action: rsx! {
                Button { "Save changes" }
            },
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Website",
                SettingsField { label: "Site name", value: "NovaCraft" }
                SettingsField { label: "Custom domain", value: "www.example.com" }
                SettingsField { label: "Subdomain", value: "novacraft" }
                div {
                    class: "border-b border-border-subtle py-4 last:border-0",
                    div {
                        class: "mb-3",
                        label { class: "block text-xs font-medium text-text-muted", "Site logo" }
                        p {
                            class: "mt-1 text-xs text-text-dim",
                            "PNG, JPEG, WebP, or GIF. Maximum 4 MB."
                        }
                    }
                    div {
                        class: "flex flex-col gap-4 sm:flex-row sm:items-center",
                        div {
                            class: "ui-squircle grid h-24 w-24 shrink-0 place-items-center overflow-hidden border border-border-subtle bg-surface-elevated",
                            img {
                                class: "h-full w-full object-contain p-3",
                                src: "{logo_url}",
                                alt: "Current site logo",
                            }
                        }
                        div {
                            class: "min-w-0 flex-1",
                            label {
                                class: "ui-btn ui-squircle ui-btn-secondary inline-flex h-9 cursor-pointer items-center justify-center gap-2 px-3 text-xs font-semibold",
                                input {
                                    r#type: "file",
                                    accept: "image/png,image/jpeg,image/webp,image/gif",
                                    class: "sr-only",
                                    disabled: uploading_logo(),
                                    onchange: move |event| {
                                        let files = event.files();
                                        let Some(file) = files.first().cloned() else {
                                            return;
                                        };
                                        let file_name = file.name();
                                        uploading_logo.set(true);
                                        logo_status.set("Uploading…".to_string());
                                        spawn(async move {
                                            let result = match file.read_bytes().await {
                                                Ok(bytes) => {
                                                    upload_site_logo(file_name, bytes.to_vec()).await
                                                }
                                                Err(error) => Err(ServerFnError::new(error.to_string())),
                                            };
                                            match result {
                                                Ok(()) => {
                                                    logo_revision += 1;
                                                    logo_status.set("Logo uploaded successfully.".to_string());
                                                }
                                                Err(error) => {
                                                    logo_status.set(format!("Upload failed: {error}"));
                                                }
                                            }
                                            uploading_logo.set(false);
                                        });
                                    },
                                }
                                if uploading_logo() { "Uploading…" } else { "Choose image" }
                            }
                            p {
                                class: "mt-2 break-all font-mono text-xs text-text-muted",
                                "{logo_template_hint}"
                            }
                            if !logo_status().is_empty() {
                                p { class: "mt-2 text-xs text-text-muted", "{logo_status}" }
                            }
                        }
                    }
                }
                div {
                    class: "border-b border-border-subtle py-4 last:border-0",
                    div {
                        class: "mb-3",
                        label { class: "block text-xs font-medium text-text-muted", "Site favicon" }
                        p {
                            class: "mt-1 text-xs text-text-dim",
                            "PNG, JPEG, WebP, GIF, or ICO. Maximum 2 MB."
                        }
                    }
                    div {
                        class: "flex flex-col gap-4 sm:flex-row sm:items-center",
                        div {
                            class: "ui-squircle grid h-24 w-24 shrink-0 place-items-center overflow-hidden border border-border-subtle bg-surface-elevated",
                            img {
                                class: "h-12 w-12 object-contain",
                                src: "{favicon_url}",
                                alt: "Current site favicon",
                            }
                        }
                        div {
                            class: "min-w-0 flex-1",
                            label {
                                class: "ui-btn ui-squircle ui-btn-secondary inline-flex h-9 cursor-pointer items-center justify-center gap-2 px-3 text-xs font-semibold",
                                input {
                                    r#type: "file",
                                    accept: "image/png,image/jpeg,image/webp,image/gif,image/x-icon,image/vnd.microsoft.icon,.ico",
                                    class: "sr-only",
                                    disabled: uploading_favicon(),
                                    onchange: move |event| {
                                        let files = event.files();
                                        let Some(file) = files.first().cloned() else {
                                            return;
                                        };
                                        let file_name = file.name();
                                        uploading_favicon.set(true);
                                        favicon_status.set("Uploading…".to_string());
                                        spawn(async move {
                                            let result = match file.read_bytes().await {
                                                Ok(bytes) => {
                                                    upload_site_favicon(file_name, bytes.to_vec()).await
                                                }
                                                Err(error) => Err(ServerFnError::new(error.to_string())),
                                            };
                                            match result {
                                                Ok(()) => {
                                                    favicon_revision += 1;
                                                    let _ = document::eval(
                                                        r#"
                                                        const href = `/uploads/site-favicon?v=${Date.now()}`;
                                                        for (const link of document.querySelectorAll("link[rel~='icon']")) {
                                                          link.href = href;
                                                        }
                                                        "#,
                                                    );
                                                    favicon_status.set("Favicon uploaded successfully.".to_string());
                                                }
                                                Err(error) => {
                                                    favicon_status.set(format!("Upload failed: {error}"));
                                                }
                                            }
                                            uploading_favicon.set(false);
                                        });
                                    },
                                }
                                if uploading_favicon() { "Uploading…" } else { "Choose favicon" }
                            }
                            p {
                                class: "mt-2 break-all font-mono text-xs text-text-muted",
                                "/uploads/site-favicon"
                            }
                            if !favicon_status().is_empty() {
                                p { class: "mt-2 text-xs text-text-muted", "{favicon_status}" }
                            }
                        }
                    }
                }
                SettingRow {
                    title: "Force HTTPS",
                    description: "Redirect all traffic on your website to HTTPS.",
                    enabled: true,
                }
            }
            DataPanel {
                title: "Site options",
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
        div {
            class: "border-b border-border-subtle py-3 last:border-0",
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
            action: rsx! {
                Button {
                    IconPlus {}
                    "Add language"
                }
            },
        }

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Languages", value: "12", accent: "#34d399" }
            StatPill { label: "Translated keys", value: "94%", accent: "#3ecf8e" }
            StatPill { label: "Currencies", value: "8", accent: "#5b9dff" }
            StatPill { label: "RTL locales", value: "2", accent: "#87d1fe" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Localisation features",
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
            DataPanel {
                title: "Admin tools",
                RowItem { title: "English (UK)", meta: "Default · 100% complete", trailing: "Edit" }
                RowItem { title: "Spanish", meta: "User preference enabled · 96%", trailing: "Edit" }
                RowItem { title: "Arabic", meta: "RTL · 88% complete", trailing: "Edit" }
                RowItem { title: "German", meta: "Currency: EUR · 91%", trailing: "Edit" }
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

        section {
            class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Plugins", value: "7", accent: "#5b9dff" }
            StatPill { label: "Webhooks", value: "11", accent: "#87d1fe" }
            StatPill { label: "API calls / day", value: "18.2k", accent: "#3ecf8e" }
            StatPill { label: "Modules", value: "4", accent: "#f0a35e" }
        }

        div {
            class: "grid gap-4 lg:grid-cols-2",
            DataPanel {
                title: "Platform features",
                FeatureBullets {
                    FeatureBullet { text: "Plugin system" }
                    FeatureBullet { text: "Public API" }
                    FeatureBullet { text: "Webhooks" }
                    FeatureBullet { text: "Events" }
                    FeatureBullet { text: "Custom modules" }
                    FeatureBullet { text: "Theme marketplace" }
                }
            }
            DataPanel {
                title: "Developers can create",
                FeatureBullets {
                    FeatureBullet { text: "New features" }
                    FeatureBullet { text: "Integrations" }
                    FeatureBullet { text: "Themes" }
                    FeatureBullet { text: "Server plugins" }
                }
                div { class: "mt-4 space-y-0",
                    RowItem { title: "order.completed", meta: "Webhook · www.example.com/hooks", trailing: "Active" }
                    RowItem { title: "ticket.created", meta: "Webhook · Discord bridge", trailing: "Active" }
                    RowItem { title: "vote.claimed", meta: "Event listener · reward module", trailing: "Active" }
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
