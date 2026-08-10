use dioxus::prelude::*;

use crate::components::page::StatPill;
use crate::components::ui::*;

const GENERAL_ACCENT: &str = "#87d1fe";
const LOCALE_ACCENT: &str = "#34d399";
const DEV_ACCENT: &str = "#4ade80";
const INTEGRATIONS_ACCENT: &str = "#c4b5fd";
const SECURITY_ACCENT: &str = "#fb7185";
const HOSTING_ACCENT: &str = "#38bdf8";

#[component]
pub fn SectionIntro(
    eyebrow: &'static str,
    title: &'static str,
    #[props(default)] description: &'static str,
    #[props(default = "#b0b3c0")] accent: &'static str,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "stg-intro", style: "--stg-accent: {accent};",
            div { class: "stg-intro-head",
                div { class: "min-w-0",
                    p { class: "stg-eyebrow", "{eyebrow}" }
                    h1 { class: "stg-title", "{title}" }
                }
                if let Some(action) = action {
                    div { class: "stg-intro-action", {action} }
                }
            }
            if !description.is_empty() {
                p { class: "stg-desc", "{description}" }
            }
        }
    }
}

#[component]
pub fn ToggleField(
    #[props(default = "")] label: &'static str,
    #[props(default = "")] hint: &'static str,
    #[props(default)] enabled: bool,
) -> Element {
    let mut on = use_signal(|| enabled);
    let switch = rsx! {
        button {
            r#type: "button",
            class: if on() { "stg-toggle is-on" } else { "stg-toggle" },
            role: "switch",
            "aria-checked": if on() { "true" } else { "false" },
            onclick: move |_| {
                let next = !*on.peek();
                on.set(next);
            },
            span { class: "stg-toggle-thumb" }
        }
    };

    if label.is_empty() {
        switch
    } else {
        rsx! {
            div { class: "stg-toggle-row",
                div { class: "min-w-0",
                    p { class: "stg-toggle-label", "{label}" }
                    if !hint.is_empty() {
                        p { class: "stg-toggle-hint", "{hint}" }
                    }
                }
                {switch}
            }
        }
    }
}

#[component]
fn SettingsField(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div { class: "stg-field",
            label { class: "stg-field-label", "{label}" }
            StaticInput { value, class: "max-w-md" }
        }
    }
}

#[component]
pub fn IconLock(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "14",
            height: "14",
            view_box: "0 0 24 24",
            fill: "none",
            rect {
                x: "5",
                y: "11",
                width: "14",
                height: "9",
                rx: "2",
                stroke: "currentColor",
                stroke_width: "1.6",
            }
            path {
                d: "M8 11V7.5a4 4 0 0 1 8 0V11",
                stroke: "currentColor",
                stroke_width: "1.6",
                stroke_linecap: "round",
            }
        }
    }
}

#[component]
pub fn IconShield(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "18",
            height: "18",
            view_box: "0 0 24 24",
            fill: "none",
            path {
                d: "M12 3.5l6.5 2.6v5.4c0 4.7-2.8 7.9-6.5 9.4-3.7-1.5-6.5-4.7-6.5-9.4V6.1L12 3.5z",
                stroke: "currentColor",
                stroke_width: "1.6",
                stroke_linejoin: "round",
            }
            path {
                d: "M9.3 12l1.9 1.9 3.5-3.9",
                stroke: "currentColor",
                stroke_width: "1.6",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn IconKey(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "18",
            height: "18",
            view_box: "0 0 24 24",
            fill: "none",
            circle {
                cx: "8",
                cy: "8",
                r: "3.5",
                stroke: "currentColor",
                stroke_width: "1.6",
            }
            path {
                d: "M10.5 10.5L20 20M15.5 15.5l2.3-2.3M18 18l2-2",
                stroke: "currentColor",
                stroke_width: "1.6",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn IconEye(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        svg {
            class: "shrink-0 {class}",
            width: "14",
            height: "14",
            view_box: "0 0 24 24",
            fill: "none",
            path {
                d: "M2.5 12S6 5.5 12 5.5 21.5 12 21.5 12 18 18.5 12 18.5 2.5 12 2.5 12z",
                stroke: "currentColor",
                stroke_width: "1.6",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "2.6",
                stroke: "currentColor",
                stroke_width: "1.6",
            }
        }
    }
}

#[component]
pub fn SettingsGeneral() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {GENERAL_ACCENT};",
            SectionIntro {
                eyebrow: "Website",
                title: "General",
                description: "How this site presents itself, and who can reach it.",
                accent: GENERAL_ACCENT,
                action: rsx! {
                    Button { "Save changes" }
                },
            }

            section { class: "stg-site-presence",
                div { class: "stg-site-presence-inner",
                    div { class: "stg-site-presence-copy",
                        p { class: "stg-site-presence-kicker", "Your site" }
                        h2 { class: "stg-site-presence-name", "NovaCraft" }
                        p { class: "stg-site-presence-tagline", "Survival, Skyblock & more" }
                        div { class: "stg-site-presence-meta",
                            span { class: "stg-site-presence-domain",
                                IconLock {}
                                "www.novacraft.gg"
                            }
                            span {
                                class: "stg-site-presence-sep",
                                "aria-hidden": "true",
                            }
                            span { class: "stg-site-presence-domain", "novacraft.serverspot.app" }
                            span {
                                class: "stg-site-presence-sep",
                                "aria-hidden": "true",
                            }
                            span { class: "stg-site-presence-live", "Live" }
                        }
                    }
                    div { class: "stg-site-presence-actions",
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            IconGlobe {}
                            "Visit site"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            "Manage domain"
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Identity" }
                        p { class: "stg-site-panel-sub",
                            "Name, tagline, and the addresses players use to find you."
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "stg-site-fields",
                        SettingsField { label: "Site name", value: "NovaCraft" }
                        SettingsField {
                            label: "Tagline",
                            value: "Survival, Skyblock & more",
                        }
                        div { class: "stg-field stg-field-wide",
                            label { class: "stg-field-label", "Custom domain" }
                            StaticInput { value: "www.novacraft.gg" }
                        }
                        SettingsField { label: "Subdomain", value: "novacraft" }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Access" }
                        p { class: "stg-site-panel-sub",
                            "Defaults for how visitors reach and register on the site."
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "stg-site-access",
                        ToggleField {
                            label: "Force HTTPS",
                            hint: "Redirect all traffic on your website to HTTPS.",
                            enabled: true,
                        }
                        ToggleField {
                            label: "Maintenance mode",
                            hint: "Show a maintenance page to non-staff visitors.",
                            enabled: false,
                        }
                        ToggleField {
                            label: "User registration",
                            hint: "Allow new players to create website accounts.",
                            enabled: true,
                        }
                        ToggleField {
                            label: "Email verification",
                            hint: "Require verified email before purchases.",
                            enabled: true,
                        }
                    }
                }
            }
        }
    }
}

struct LocaleRow {
    name: &'static str,
    note: &'static str,
    pct: u8,
    tag: &'static str,
    accent: &'static str,
}

const LOCALES: &[LocaleRow] = &[
    LocaleRow {
        name: "English (UK)",
        note: "Default locale for new visitors",
        pct: 100,
        tag: "Default",
        accent: "#34d399",
    },
    LocaleRow {
        name: "Spanish",
        note: "User preference enabled",
        pct: 96,
        tag: "EUR",
        accent: "#5b9dff",
    },
    LocaleRow {
        name: "Arabic",
        note: "Right-to-left layout",
        pct: 88,
        tag: "RTL",
        accent: "#f0a35e",
    },
    LocaleRow {
        name: "German",
        note: "Currency formatting active",
        pct: 91,
        tag: "EUR",
        accent: "#87d1fe",
    },
    LocaleRow {
        name: "French",
        note: "Community translated",
        pct: 79,
        tag: "Community",
        accent: "#c4b5fd",
    },
];

#[component]
pub fn SettingsLocalisation() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {LOCALE_ACCENT};",
            SectionIntro {
                eyebrow: "Language",
                title: "Localisation",
                description: "Multi-language support, formatting, and translation management.",
                accent: LOCALE_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        "Add language"
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: "Languages", value: "12", accent: LOCALE_ACCENT }
                StatPill {
                    label: "Translated keys",
                    value: "94%",
                    accent: LOCALE_ACCENT,
                }
                StatPill { label: "Currencies", value: "8", accent: LOCALE_ACCENT }
                StatPill { label: "RTL locales", value: "2", accent: LOCALE_ACCENT }
            }

            div { class: "motion-cascade motion-cascade-tight stg-locale-matrix",
                div { class: "stg-locale-head",
                    span { "Language" }
                    span { "Completion" }
                    span { "Notes" }
                    span { "" }
                }
                for row in LOCALES {
                    div {
                        class: "stg-locale-row",
                        style: "--locale-accent: {row.accent};",
                        div { class: "stg-locale-name",
                            span { class: "stg-locale-flagdot" }
                            div {
                                p { class: "stg-locale-title", "{row.name}" }
                                p { class: "stg-locale-sub", "{row.note}" }
                            }
                        }
                        div {
                            div { class: "stg-locale-bar-track",
                                div {
                                    class: "stg-locale-bar-fill",
                                    style: "width: {row.pct}%;",
                                }
                            }
                            p { class: "stg-locale-pct", "{row.pct}% complete" }
                        }
                        div { class: "stg-locale-tags",
                            span { class: "stg-locale-tag", "{row.tag}" }
                        }
                        div { class: "stg-locale-actions",
                            Button {
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Sm,
                                "Edit"
                            }
                        }
                    }
                }
            }
        }
    }
}

struct ApiKeyRow {
    name: &'static str,
    masked: &'static str,
    env: &'static str,
    live: bool,
    scopes: &'static [&'static str],
    last_used: &'static str,
    created: &'static str,
}

const API_KEYS: &[ApiKeyRow] = &[
    ApiKeyRow {
        name: "live_storefront",
        masked: "sk_live_••••92ab",
        env: "Live",
        live: true,
        scopes: &["read:orders", "write:orders"],
        last_used: "Used 4m ago",
        created: "Created 34d ago",
    },
    ApiKeyRow {
        name: "discord_bridge",
        masked: "sk_live_••••1f3c",
        env: "Live",
        live: true,
        scopes: &["read:members"],
        last_used: "Used 1h ago",
        created: "Created 12d ago",
    },
    ApiKeyRow {
        name: "analytics_export",
        masked: "sk_test_••••00e2",
        env: "Test",
        live: false,
        scopes: &["read:analytics"],
        last_used: "Never used",
        created: "Created 2d ago",
    },
];

struct WebhookRow {
    method: &'static str,
    path: &'static str,
    events: &'static str,
    success: u8,
    delivery: &'static str,
    status: &'static str,
    ok: bool,
}

const WEBHOOKS: &[WebhookRow] = &[
    WebhookRow {
        method: "POST",
        path: "/hooks/order.completed",
        events: "order.completed · order.refunded",
        success: 100,
        delivery: "482ms average · 1.4k sent",
        status: "Healthy",
        ok: true,
    },
    WebhookRow {
        method: "POST",
        path: "/hooks/ticket.created",
        events: "ticket.created · ticket.replied",
        success: 99,
        delivery: "210ms average · 860 sent",
        status: "Healthy",
        ok: true,
    },
    WebhookRow {
        method: "POST",
        path: "/hooks/vote.claimed",
        events: "vote.claimed",
        success: 74,
        delivery: "1.2s average · 3 retrying",
        status: "Retrying",
        ok: false,
    },
];

#[component]
pub fn SettingsDeveloper() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {DEV_ACCENT};",
            SectionIntro {
                eyebrow: "Platform",
                title: "Developer",
                description: "API keys, webhooks, and tools for extending ServerSpot.",
                accent: DEV_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        "Create API key"
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: "Active keys", value: "3", accent: DEV_ACCENT }
                StatPill {
                    label: "Webhook endpoints",
                    value: "11",
                    accent: DEV_ACCENT,
                }
                StatPill {
                    label: "API calls / day",
                    value: "18.2k",
                    accent: DEV_ACCENT,
                }
                StatPill {
                    label: "Failed deliveries",
                    value: "3",
                    accent: DEV_ACCENT,
                }
            }

            div { class: "motion-cascade motion-cascade-tight stg-dev-panel",
                div { class: "stg-dev-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-dev-panel-title", "API keys" }
                        p { class: "stg-dev-panel-sub",
                            "Secrets that authenticate requests to the ServerSpot API."
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "View documentation"
                    }
                }
                div { class: "stg-key-head",
                    span { "Key" }
                    span { "Scopes" }
                    span { "Activity" }
                    span { "" }
                }
                for key in API_KEYS {
                    div { class: "stg-key-row",
                        div { class: "min-w-0",
                            div { class: "stg-key-name-row",
                                p { class: "stg-key-name stg-dev-mono", "{key.name}" }
                                span { class: if key.live { "stg-key-env is-live" } else { "stg-key-env" },
                                    "{key.env}"
                                }
                            }
                            div { class: "stg-key-secret",
                                span { class: "stg-key-secret-text stg-dev-mono", "{key.masked}" }
                                button { class: "stg-key-copy", "Copy" }
                            }
                        }
                        div { class: "stg-key-scopes",
                            for scope in key.scopes {
                                span { class: "stg-key-scope stg-dev-mono", "{scope}" }
                            }
                        }
                        div {
                            p { class: "stg-key-activity", "{key.last_used}" }
                            p { class: "stg-key-activity-sub", "{key.created}" }
                        }
                        div { class: "stg-key-actions",
                            Button {
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Sm,
                                "Roll"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                "Revoke"
                            }
                        }
                    }
                }
            }

            div { class: "motion-cascade motion-cascade-tight stg-dev-panel",
                div { class: "stg-dev-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-dev-panel-title", "Webhook endpoints" }
                        p { class: "stg-dev-panel-sub",
                            "Where ServerSpot posts events from your store, forum, and votes."
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        IconPlus {}
                        "Add endpoint"
                    }
                }
                for hook in WEBHOOKS {
                    div { class: "stg-hook-row",
                        div { class: "min-w-0",
                            div { class: "stg-hook-path-row",
                                span { class: "stg-hook-method stg-dev-mono", "{hook.method}" }
                                p { class: "stg-hook-path stg-dev-mono", "{hook.path}" }
                            }
                            p { class: "stg-hook-events", "{hook.events}" }
                        }
                        div {
                            div { class: "stg-hook-bar-track",
                                div {
                                    class: if hook.ok { "stg-hook-bar-fill" } else { "stg-hook-bar-fill is-warn" },
                                    style: "width: {hook.success}%;",
                                }
                            }
                            p { class: "stg-hook-health-meta",
                                "{hook.success}% delivered · {hook.delivery}"
                            }
                        }
                        div { class: "stg-hook-side",
                            span { class: if hook.ok { "stg-hook-status" } else { "stg-hook-status is-warn" },
                                span { class: "stg-hook-status-dot" }
                                "{hook.status}"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                "Send test"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum IntegrationId {
    Discord,
    GoogleAnalytics,
    Zapier,
    SendGrid,
}

struct IntegrationTile {
    id: IntegrationId,
    name: &'static str,
    desc: &'static str,
    accent: &'static str,
    connected: bool,
}

const INTEGRATIONS: &[IntegrationTile] = &[
    IntegrationTile {
        id: IntegrationId::Discord,
        name: "Discord",
        desc: "Login, role rewards, and purchase announcements.",
        accent: "#5865F2",
        connected: true,
    },
    IntegrationTile {
        id: IntegrationId::GoogleAnalytics,
        name: "Google Analytics",
        desc: "Track traffic and conversion funnels on your site.",
        accent: "#F9AB00",
        connected: false,
    },
    IntegrationTile {
        id: IntegrationId::Zapier,
        name: "Zapier",
        desc: "Automate workflows across hundreds of apps.",
        accent: "#FF4A00",
        connected: true,
    },
    IntegrationTile {
        id: IntegrationId::SendGrid,
        name: "SendGrid",
        desc: "Transactional email delivery for receipts and alerts.",
        accent: "#1A82E2",
        connected: false,
    },
];

#[component]
fn IntegrationBrandIcon(id: IntegrationId) -> Element {
    match id {
        IntegrationId::Discord => rsx! {
            svg {
                class: "stg-tile-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.09.09 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.09 16.09 0 0 0-4.8 0c-.14-.34-.37-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.02.06.03.09.02c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02zM8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12zm6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12z" }
            }
        },
        IntegrationId::GoogleAnalytics => rsx! {
            svg {
                class: "stg-tile-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M22.84 2.98a1.62 1.62 0 1 1-3.24 0 1.62 1.62 0 0 1 3.24 0zM1.16 17.76v4.48h4.48a4.48 4.48 0 0 0-4.48-4.48zm0-6.72v4.48c3.71 0 6.72 3.01 6.72 6.72h4.48c0-6.18-5.02-11.2-11.2-11.2zm0-6.72v4.48c7.42 0 13.44 6.02 13.44 13.44h4.48C19.08 10.61 10.95 2.48 1.16 4.32z" }
            }
        },
        IntegrationId::Zapier => rsx! {
            svg {
                class: "stg-tile-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M12 0 9.04 8.04H0l7.07 5.04L4.1 24 12 17.28 19.9 24l-2.97-10.92L24 8.04h-9.04L12 0z" }
            }
        },
        IntegrationId::SendGrid => rsx! {
            svg {
                class: "stg-tile-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M.4 0v9.6h9.6V0H.4zm13.6 0v9.6h9.6V0H14zM.4 14.4V24h9.6v-9.6H.4zm13.6 0 .1 3.2h3.1V24H24v-9.6H14z" }
            }
        },
    }
}

#[component]
pub fn SettingsIntegrations() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {INTEGRATIONS_ACCENT};",
            SectionIntro {
                eyebrow: "Ecosystem",
                title: "Integrations",
                description: "Connect Discord, analytics, email, and automation tools.",
                accent: INTEGRATIONS_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        "Add integration"
                    }
                },
            }

            div { class: "motion-cascade stg-tile-grid",
                for tile in INTEGRATIONS {
                    div {
                        class: "stg-tile",
                        style: "--tile-accent: {tile.accent};",
                        div { class: "stg-tile-top",
                            span {
                                class: "stg-tile-badge",
                                "aria-hidden": "true",
                                IntegrationBrandIcon { id: tile.id }
                            }
                            span { class: if tile.connected { "stg-tile-status is-on" } else { "stg-tile-status" },
                                if tile.connected {
                                    "Connected"
                                } else {
                                    "Off"
                                }
                            }
                        }
                        div {
                            p { class: "stg-tile-name", "{tile.name}" }
                            p { class: "stg-tile-desc", "{tile.desc}" }
                        }
                        div { class: "stg-tile-foot",
                            span { class: "text-xs text-text-muted", "Manage" }
                            ToggleField { enabled: tile.connected }
                        }
                    }
                }
            }
        }
    }
}

struct PolicyCard {
    title: &'static str,
    desc: &'static str,
    enabled: bool,
}

fn posture_band(score: u32) -> (&'static str, &'static str) {
    match score {
        0..=39 => ("var(--color-danger)", "Weak posture"),
        40..=69 => ("var(--color-warning)", "Fair posture"),
        _ => ("var(--color-success)", "Strong posture"),
    }
}

const POLICIES: &[PolicyCard] = &[
    PolicyCard {
        title: "Two-factor authentication",
        desc: "Encourage 2FA for staff and high-value accounts.",
        enabled: true,
    },
    PolicyCard {
        title: "CAPTCHA on forms",
        desc: "Protect login, register, and ticket forms from spam.",
        enabled: true,
    },
    PolicyCard {
        title: "Cookie consent",
        desc: "Ask visitors for permission before storing cookies.",
        enabled: false,
    },
    PolicyCard {
        title: "Session idle timeout",
        desc: "Sign staff out automatically after 30 minutes idle.",
        enabled: true,
    },
];

#[component]
pub fn SettingsSecurity() -> Element {
    let score: u32 = 82;
    let segments: u32 = 10;
    let filled = ((score as f32 / 100.0) * segments as f32).round() as u32;
    let (band_color, band_label) = posture_band(score);

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {SECURITY_ACCENT};",
            SectionIntro {
                eyebrow: "Protection",
                title: "Security",
                description: "Authentication posture, spam protection, and privacy controls.",
                accent: SECURITY_ACCENT,
            }

            div { class: "stg-posture", style: "--posture-color: {band_color};",
                div { class: "stg-posture-top",
                    div { class: "stg-posture-score",
                        span { class: "stg-posture-score-num", "{score}" }
                        span { class: "stg-posture-score-max", "/ 100" }
                    }
                    span { class: "stg-posture-score-label", "{band_label}" }
                }
                div { class: "stg-posture-meter",
                    for i in 0..segments {
                        span {
                            class: if i < filled { "stg-posture-meter-seg is-filled" } else { "stg-posture-meter-seg" },
                            style: "--seg-index: {i};",
                        }
                    }
                }
                div { class: "stg-posture-scale",
                    span { "Weak" }
                    span { "Fair" }
                    span { "Good" }
                    span { "Strong" }
                }
            }

            div { class: "motion-cascade stg-policy-grid",
                for policy in POLICIES {
                    div { class: "stg-policy-card",
                        span { class: "stg-policy-icon", IconShield {} }
                        div { class: "stg-policy-body",
                            div { class: "stg-policy-title-row",
                                p { class: "stg-policy-title", "{policy.title}" }
                                ToggleField { enabled: policy.enabled }
                            }
                            p { class: "stg-policy-desc", "{policy.desc}" }
                        }
                    }
                }
            }
        }
    }
}

struct InfraNode {
    name: &'static str,
    meta: &'static str,
    warn: bool,
}

const INFRA_NODES: &[InfraNode] = &[
    InfraNode {
        name: "Web",
        meta: "99.98% uptime · 42ms",
        warn: false,
    },
    InfraNode {
        name: "Database",
        meta: "99.99% uptime · 8ms",
        warn: false,
    },
    InfraNode {
        name: "CDN",
        meta: "99.95% uptime · 21ms",
        warn: false,
    },
    InfraNode {
        name: "Email relay",
        meta: "Delayed queue · 3 retries",
        warn: true,
    },
];

const BACKUP_TICKS: &[bool] = &[true, true, true, true, false, true, true];
const BACKUP_LABELS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[component]
pub fn SettingsHosting() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {HOSTING_ACCENT};",
            SectionIntro {
                eyebrow: "Infrastructure",
                title: "Hosting",
                description: "Cloud hosting, backups, and deployment for this website.",
                accent: HOSTING_ACCENT,
            }

            div { class: "motion-cascade stg-infra-strip",
                for node in INFRA_NODES {
                    div { class: "stg-infra-node",
                        div { class: "stg-infra-top",
                            span { class: if node.warn { "stg-infra-dot is-warn" } else { "stg-infra-dot" } }
                            span { class: "stg-infra-name", "{node.name}" }
                        }
                        p { class: "stg-infra-meta", "{node.meta}" }
                    }
                }
            }

            div { class: "ui-card p-4",
                div { class: "mb-1 flex items-center justify-between gap-3",
                    h2 { class: "text-sm font-semibold text-text", "Automatic backups" }
                    span { class: "text-xs text-text-muted tabular-nums", "Last snapshot 2h ago" }
                }
                div { class: "stg-infra-timeline",
                    for ok in BACKUP_TICKS {
                        span { class: if *ok { "stg-infra-tick" } else { "stg-infra-tick is-missed" } }
                    }
                }
                div { class: "stg-infra-timeline-labels",
                    for label in BACKUP_LABELS {
                        span { "{label}" }
                    }
                }
                div { class: "mt-3 border-t border-border-subtle pt-1",
                    ToggleField {
                        label: "Nightly snapshots",
                        hint: "Automatic backups of your site and database.",
                        enabled: true,
                    }
                }
                div { class: "mt-4",
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Open hosting panel" }
                }
            }
        }
    }
}
