use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::StatPill;
use crate::components::ui::*;
use crate::i18n::{
    builtin_translation_catalogs, catalog_from_uploaded_ftl, settings_locale_keys_note,
    settings_locale_pct_complete, settings_locale_upload_success, source_key_count, t_key,
    TranslationCatalog,
};

const GENERAL_ACCENT: &str = "#87d1fe";
const LOCALE_ACCENT: &str = "#34d399";
const DEV_ACCENT: &str = "#4ade80";
const INTEGRATIONS_ACCENT: &str = "#c4b5fd";
const SECURITY_ACCENT: &str = "#fb7185";
const HOSTING_ACCENT: &str = "#38bdf8";

#[component]
pub fn SectionIntro(
    #[props(into)] eyebrow: String,
    #[props(into)] title: String,
    #[props(default)] description: String,
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
    #[props(default = String::new())] label: String,
    #[props(default = String::new())] hint: String,
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
fn SettingsField(#[props(into)] label: String, value: &'static str) -> Element {
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
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {GENERAL_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-general-eyebrow"),
                title: t_key("settings-general-title"),
                description: t_key("settings-general-description"),
                accent: GENERAL_ACCENT,
                action: rsx! {
                    Button { { t!("common-save-changes") } }
                },
            }

            section { class: "stg-site-presence",
                div { class: "stg-site-presence-inner",
                    div { class: "stg-site-presence-copy",
                        p { class: "stg-site-presence-kicker", { t!("settings-general-your-site") } }
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
                            span { class: "stg-site-presence-live", { t!("settings-general-live") } }
                        }
                    }
                    div { class: "stg-site-presence-actions",
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            IconGlobe {}
                            { t!("settings-general-visit-site") }
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            { t!("settings-general-manage-domain") }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("settings-general-identity-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("settings-general-identity-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "stg-site-fields",
                        SettingsField { label: t_key("settings-general-field-site-name"), value: "NovaCraft" }
                        SettingsField {
                            label: t_key("settings-general-field-tagline"),
                            value: "Survival, Skyblock & more",
                        }
                        div { class: "stg-field stg-field-wide",
                            label { class: "stg-field-label", { t!("settings-general-field-custom-domain") } }
                            StaticInput { value: "www.novacraft.gg" }
                        }
                        SettingsField { label: t_key("settings-general-field-subdomain"), value: "novacraft" }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("settings-general-access-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("settings-general-access-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "stg-site-access",
                        ToggleField {
                            label: t_key("settings-general-toggle-https-label"),
                            hint: t_key("settings-general-toggle-https-hint"),
                            enabled: true,
                        }
                        ToggleField {
                            label: t_key("settings-general-toggle-maintenance-label"),
                            hint: t_key("settings-general-toggle-maintenance-hint"),
                            enabled: false,
                        }
                        ToggleField {
                            label: t_key("settings-general-toggle-registration-label"),
                            hint: t_key("settings-general-toggle-registration-hint"),
                            enabled: true,
                        }
                        ToggleField {
                            label: t_key("settings-general-toggle-verification-label"),
                            hint: t_key("settings-general-toggle-verification-hint"),
                            enabled: true,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingsLocalisation() -> Element {
    let _lang = i18n();
    let source_total = source_key_count();
    let mut catalogs = use_signal(builtin_translation_catalogs);
    let mut upload_error = use_signal(|| Option::<String>::None);
    let mut upload_notice = use_signal(|| Option::<String>::None);

    let language_count = catalogs.read().len();
    let uploaded_count = catalogs.read().iter().filter(|c| c.is_uploaded).count();
    let avg_coverage = if language_count == 0 {
        0
    } else {
        let sum: usize = catalogs
            .read()
            .iter()
            .map(|c| c.completion_pct(source_total) as usize)
            .sum();
        sum / language_count
    };
    let rows: Vec<TranslationCatalog> = catalogs.read().clone();
    let language_count_label = language_count.to_string();
    let source_total_label = source_total.to_string();
    let avg_coverage_label = format!("{avg_coverage}%");
    let uploaded_count_label = uploaded_count.to_string();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {LOCALE_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-locale-eyebrow"),
                title: t_key("settings-locale-title"),
                description: t_key("settings-locale-description"),
                accent: LOCALE_ACCENT,
                action: rsx! {
                    label {
                        class: "ui-btn ui-squircle ui-btn-primary inline-flex h-10 cursor-pointer items-center justify-center gap-2 px-4 text-sm font-semibold",
                        IconPlus {}
                        { t!("settings-locale-add-language") }
                        input {
                            r#type: "file",
                            accept: ".ftl,text/plain",
                            class: "sr-only",
                            onchange: move |evt| {
                                async move {
                                    upload_error.set(None);
                                    upload_notice.set(None);
                                    let Some(file) = evt.files().into_iter().next() else {
                                        return;
                                    };
                                    let name = file.name();
                                    let Ok(bytes) = file.read_bytes().await else {
                                        upload_error
                                            .set(Some(t_key("settings-locale-upload-read-failed")));
                                        return;
                                    };
                                    let Ok(text) = String::from_utf8(bytes.to_vec()) else {
                                        upload_error
                                            .set(Some(t_key("settings-locale-upload-invalid-utf8")));
                                        return;
                                    };
                                    match catalog_from_uploaded_ftl(&name, &text) {
                                        Ok(catalog) => {
                                            let code = catalog.code.clone();
                                            catalogs.with_mut(|list| {
                                                if let Some(existing) =
                                                    list.iter_mut().find(|row| row.code == code)
                                                {
                                                    *existing = catalog;
                                                } else {
                                                    list.push(catalog);
                                                    list.sort_by(|a, b| a.code.cmp(&b.code));
                                                }
                                            });
                                            upload_notice
                                                .set(Some(settings_locale_upload_success(code)));
                                        }
                                        Err(key) => {
                                            upload_error.set(Some(t_key(key)));
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
            }

            if let Some(error) = upload_error() {
                p { class: "mb-4 text-sm text-danger", "{error}" }
            }
            if let Some(notice) = upload_notice() {
                p { class: "mb-4 text-sm text-success", "{notice}" }
            }
            p { class: "mb-4 text-xs text-text-muted", { t!("settings-locale-upload-hint") } }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill {
                    label: t_key("settings-locale-stat-languages"),
                    value: language_count_label,
                    accent: LOCALE_ACCENT,
                }
                StatPill {
                    label: t_key("settings-locale-stat-source-keys"),
                    value: source_total_label,
                    accent: LOCALE_ACCENT,
                }
                StatPill {
                    label: t_key("settings-locale-stat-avg-coverage"),
                    value: avg_coverage_label,
                    accent: LOCALE_ACCENT,
                }
                StatPill {
                    label: t_key("settings-locale-stat-uploaded"),
                    value: uploaded_count_label,
                    accent: LOCALE_ACCENT,
                }
            }

            div { class: "motion-cascade motion-cascade-tight stg-locale-matrix",
                div { class: "stg-locale-head",
                    span { { t!("settings-locale-col-language") } }
                    span { { t!("settings-locale-col-completion") } }
                    span { { t!("settings-locale-col-notes") } }
                    span { "" }
                }
                for row in rows {
                    {
                        let pct = row.completion_pct(source_total);
                        let title = row.display_name();
                        let note = settings_locale_keys_note(
                            row.matching_keys,
                            source_total,
                            row.filename.clone(),
                        );
                        let pct_label = settings_locale_pct_complete(pct);
                        let tag = if row.is_source {
                            t_key("settings-locale-tag-source")
                        } else if row.is_uploaded {
                            t_key("settings-locale-tag-uploaded")
                        } else if pct >= 100 {
                            t_key("settings-locale-tag-complete")
                        } else {
                            t_key("settings-locale-tag-partial")
                        };
                        let accent = row.accent;
                        let code = row.code.clone();
                        rsx! {
                            div {
                                class: "stg-locale-row",
                                style: "--locale-accent: {accent};",
                                div { class: "stg-locale-name",
                                    span { class: "stg-locale-flagdot" }
                                    div {
                                        p { class: "stg-locale-title", "{title}" }
                                        p { class: "stg-locale-sub", "{code}" }
                                    }
                                }
                                div {
                                    div { class: "stg-locale-bar-track",
                                        div {
                                            class: "stg-locale-bar-fill",
                                            style: "width: {pct}%;",
                                        }
                                    }
                                    p { class: "stg-locale-pct", "{pct_label}" }
                                }
                                div { class: "stg-locale-tags",
                                    span { class: "stg-locale-tag", "{tag}" }
                                    p { class: "stg-locale-sub mt-1", "{note}" }
                                }
                                div { class: "stg-locale-actions",
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        size: ButtonSize::Sm,
                                        { t!("common-edit") }
                                    }
                                }
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
    env_key: &'static str,
    live: bool,
    scopes: &'static [&'static str],
    last_used: &'static str,
    created: &'static str,
}

const API_KEYS: &[ApiKeyRow] = &[
    ApiKeyRow {
        name: "live_storefront",
        masked: "sk_live_••••92ab",
        env_key: "settings-dev-env-live",
        live: true,
        scopes: &["read:orders", "write:orders"],
        last_used: "Used 4m ago",
        created: "Created 34d ago",
    },
    ApiKeyRow {
        name: "discord_bridge",
        masked: "sk_live_••••1f3c",
        env_key: "settings-dev-env-live",
        live: true,
        scopes: &["read:members"],
        last_used: "Used 1h ago",
        created: "Created 12d ago",
    },
    ApiKeyRow {
        name: "analytics_export",
        masked: "sk_test_••••00e2",
        env_key: "settings-dev-env-test",
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
    status_key: &'static str,
    ok: bool,
}

const WEBHOOKS: &[WebhookRow] = &[
    WebhookRow {
        method: "POST",
        path: "/hooks/order.completed",
        events: "order.completed · order.refunded",
        success: 100,
        delivery: "482ms average · 1.4k sent",
        status_key: "settings-dev-hook-status-healthy",
        ok: true,
    },
    WebhookRow {
        method: "POST",
        path: "/hooks/ticket.created",
        events: "ticket.created · ticket.replied",
        success: 99,
        delivery: "210ms average · 860 sent",
        status_key: "settings-dev-hook-status-healthy",
        ok: true,
    },
    WebhookRow {
        method: "POST",
        path: "/hooks/vote.claimed",
        events: "vote.claimed",
        success: 74,
        delivery: "1.2s average · 3 retrying",
        status_key: "settings-dev-hook-status-retrying",
        ok: false,
    },
];

#[component]
pub fn SettingsDeveloper() -> Element {
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {DEV_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-dev-eyebrow"),
                title: t_key("settings-dev-title"),
                description: t_key("settings-dev-description"),
                accent: DEV_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        { t!("settings-dev-create-api-key") }
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: t_key("settings-dev-stat-active-keys"), value: "3", accent: DEV_ACCENT }
                StatPill {
                    label: t_key("settings-dev-stat-webhook-endpoints"),
                    value: "11",
                    accent: DEV_ACCENT,
                }
                StatPill {
                    label: t_key("settings-dev-stat-api-calls-day"),
                    value: "18.2k",
                    accent: DEV_ACCENT,
                }
                StatPill {
                    label: t_key("settings-dev-stat-failed-deliveries"),
                    value: "3",
                    accent: DEV_ACCENT,
                }
            }

            div { class: "motion-cascade motion-cascade-tight stg-dev-panel",
                div { class: "stg-dev-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-dev-panel-title", { t!("settings-dev-api-keys-title") } }
                        p { class: "stg-dev-panel-sub",
                            { t!("settings-dev-api-keys-sub") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        { t!("settings-dev-view-docs") }
                    }
                }
                div { class: "stg-key-head",
                    span { { t!("settings-dev-col-key") } }
                    span { { t!("settings-dev-col-scopes") } }
                    span { { t!("settings-dev-col-activity") } }
                    span { "" }
                }
                for key in API_KEYS {
                    div { class: "stg-key-row",
                        div { class: "min-w-0",
                            div { class: "stg-key-name-row",
                                p { class: "stg-key-name stg-dev-mono", "{key.name}" }
                                span { class: if key.live { "stg-key-env is-live" } else { "stg-key-env" },
                                    { t_key(key.env_key) }
                                }
                            }
                            div { class: "stg-key-secret",
                                span { class: "stg-key-secret-text stg-dev-mono", "{key.masked}" }
                                button { class: "stg-key-copy", { t!("common-copy") } }
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
                                { t!("settings-dev-roll") }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                { t!("settings-dev-revoke") }
                            }
                        }
                    }
                }
            }

            div { class: "motion-cascade motion-cascade-tight stg-dev-panel",
                div { class: "stg-dev-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-dev-panel-title", { t!("settings-dev-webhooks-title") } }
                        p { class: "stg-dev-panel-sub",
                            { t!("settings-dev-webhooks-sub") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        IconPlus {}
                        { t!("settings-dev-add-endpoint") }
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
                                { t!("settings-dev-hook-delivered", pct: hook.success, delivery: hook.delivery) }
                            }
                        }
                        div { class: "stg-hook-side",
                            span { class: if hook.ok { "stg-hook-status" } else { "stg-hook-status is-warn" },
                                span { class: "stg-hook-status-dot" }
                                { t_key(hook.status_key) }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                { t!("settings-dev-send-test") }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct IntegrationTile {
    name: &'static str,
    desc_key: &'static str,
    accent: &'static str,
    connected: bool,
    icon: &'static str,
}

const INTEGRATIONS: &[IntegrationTile] = &[
    IntegrationTile {
        name: "Minecraft",
        desc_key: "settings-integrations-desc-minecraft",
        accent: "#62B64A",
        connected: true,
        icon: "M3 8l9-5 9 5v8l-9 5-9-5V8zm9 1.7L6.2 6.6 12 3.4l5.8 3.2L12 9.7zm1 .85v6.9l6-3.3V7.25L13 10.55z",
    },
    IntegrationTile {
        name: "FiveM",
        desc_key: "settings-integrations-desc-fivem",
        accent: "#F40552",
        connected: true,
        icon: "M12 2 20 6.5v11L12 22 4 17.5v-11L12 2zm-2.2 5.2v9.6h1.8v-3.4h2.1c1.8 0 2.9-1 2.9-2.6 0-1.6-1.1-2.6-2.9-2.6H9.8zm1.8 1.6h1.8c.8 0 1.3.4 1.3 1.1s-.5 1.1-1.3 1.1H11.6V8.8z",
    },
    IntegrationTile {
        name: "Rust",
        desc_key: "settings-integrations-desc-rust",
        accent: "#CE422B",
        connected: true,
        icon: "M12 9a3 3 0 100 6 3 3 0 000-6zm0-7 1.2 3.8L17 4.2l.9 3.8L22 9.2 19.2 12 22 14.8l-4.1 1.2-.9 3.8-3.8-1.6L12 22l-1.2-3.8-3.8 1.6-.9-3.8L2 14.8 4.8 12 2 9.2l4.1-1.2.9-3.8 3.8 1.6L12 2z",
    },
    IntegrationTile {
        name: "Hytale",
        desc_key: "settings-integrations-desc-hytale",
        accent: "#3DDC97",
        connected: false,
        icon: "M4 3h6v7h4V3h6v18h-6v-7H10v7H4V3z",
    },
    IntegrationTile {
        name: "Discord",
        desc_key: "settings-integrations-desc-discord",
        accent: "#5865F2",
        connected: true,
        icon: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.09.09 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.09 16.09 0 0 0-4.8 0c-.14-.34-.37-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.02.06.03.09.02c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02zM8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12zm6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12z",
    },
    IntegrationTile {
        name: "Google Analytics",
        desc_key: "settings-integrations-desc-google-analytics",
        accent: "#F9AB00",
        connected: false,
        icon: "M22.84 2.98a1.62 1.62 0 1 1-3.24 0 1.62 1.62 0 0 1 3.24 0zM1.16 17.76v4.48h4.48a4.48 4.48 0 0 0-4.48-4.48zm0-6.72v4.48c3.71 0 6.72 3.01 6.72 6.72h4.48c0-6.18-5.02-11.2-11.2-11.2zm0-6.72v4.48c7.42 0 13.44 6.02 13.44 13.44h4.48C19.08 10.61 10.95 2.48 1.16 4.32z",
    },
    IntegrationTile {
        name: "Zapier",
        desc_key: "settings-integrations-desc-zapier",
        accent: "#FF4A00",
        connected: true,
        icon: "M12 0 9.04 8.04H0l7.07 5.04L4.1 24 12 17.28 19.9 24l-2.97-10.92L24 8.04h-9.04L12 0z",
    },
    IntegrationTile {
        name: "SendGrid",
        desc_key: "settings-integrations-desc-sendgrid",
        accent: "#1A82E2",
        connected: false,
        icon: "M.4 0v9.6h9.6V0H.4zm13.6 0v9.6h9.6V0H14zM.4 14.4V24h9.6v-9.6H.4zm13.6 0 .1 3.2h3.1V24H24v-9.6H14z",
    },
];

#[component]
fn IntegrationBrandIcon(name: &'static str, icon: &'static str) -> Element {
    match name {
        "Minecraft" => rsx! {
            img {
                class: "stg-tile-brand is-mark",
                src: asset!("/assets/integrations/minecraft.svg"),
                alt: "",
            }
        },
        "FiveM" => rsx! {
            img {
                class: "stg-tile-brand is-mark",
                src: asset!("/assets/integrations/fivem.svg"),
                alt: "",
            }
        },
        "Rust" => rsx! {
            img {
                class: "stg-tile-brand is-mark is-rust",
                src: asset!("/assets/integrations/rust.png"),
                alt: "",
            }
        },
        "Hytale" => rsx! {
            img {
                class: "stg-tile-brand is-mark",
                src: asset!("/assets/integrations/hytale.webp"),
                alt: "",
            }
        },
        _ => rsx! {
            svg {
                class: "stg-tile-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: icon }
            }
        },
    }
}

#[component]
pub fn SettingsIntegrations() -> Element {
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {INTEGRATIONS_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-integrations-eyebrow"),
                title: t_key("settings-integrations-title"),
                description: t_key("settings-integrations-description"),
                accent: INTEGRATIONS_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        { t!("settings-integrations-add") }
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
                                IntegrationBrandIcon { name: tile.name, icon: tile.icon }
                            }
                            span { class: if tile.connected { "stg-tile-status is-on" } else { "stg-tile-status" },
                                if tile.connected {
                                    { t!("common-connected") }
                                } else {
                                    { t!("common-off") }
                                }
                            }
                        }
                        div {
                            p { class: "stg-tile-name", "{tile.name}" }
                            p { class: "stg-tile-desc", { t_key(tile.desc_key) } }
                        }
                        div { class: "stg-tile-foot",
                            span { class: "text-xs text-text-muted", { t!("common-manage") } }
                            ToggleField { enabled: tile.connected }
                        }
                    }
                }
            }
        }
    }
}

struct PolicyCard {
    title_key: &'static str,
    desc_key: &'static str,
    enabled: bool,
}

fn posture_band_key(score: u32) -> &'static str {
    match score {
        0..=39 => "settings-security-posture-weak",
        40..=69 => "settings-security-posture-fair",
        _ => "settings-security-posture-strong",
    }
}

const POLICIES: &[PolicyCard] = &[
    PolicyCard {
        title_key: "settings-security-policy-2fa-title",
        desc_key: "settings-security-policy-2fa-desc",
        enabled: true,
    },
    PolicyCard {
        title_key: "settings-security-policy-captcha-title",
        desc_key: "settings-security-policy-captcha-desc",
        enabled: true,
    },
    PolicyCard {
        title_key: "settings-security-policy-cookie-title",
        desc_key: "settings-security-policy-cookie-desc",
        enabled: false,
    },
    PolicyCard {
        title_key: "settings-security-policy-session-title",
        desc_key: "settings-security-policy-session-desc",
        enabled: true,
    },
];

#[component]
pub fn SettingsSecurity() -> Element {
    let _lang = i18n();
    let score: u32 = 82;
    let segments: u32 = 10;
    let filled = ((score as f32 / 100.0) * segments as f32).round() as u32;
    let band_color = match score {
        0..=39 => "var(--color-danger)",
        40..=69 => "var(--color-warning)",
        _ => "var(--color-success)",
    };
    let band_key = posture_band_key(score);

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {SECURITY_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-security-eyebrow"),
                title: t_key("settings-security-title"),
                description: t_key("settings-security-description"),
                accent: SECURITY_ACCENT,
            }

            div { class: "stg-posture", style: "--posture-color: {band_color};",
                div { class: "stg-posture-top",
                    div { class: "stg-posture-score",
                        span { class: "stg-posture-score-num", "{score}" }
                        span { class: "stg-posture-score-max", "/ 100" }
                    }
                    span { class: "stg-posture-score-label", { t_key(band_key) } }
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
                    span { { t!("settings-security-scale-weak") } }
                    span { { t!("settings-security-scale-fair") } }
                    span { { t!("settings-security-scale-good") } }
                    span { { t!("settings-security-scale-strong") } }
                }
            }

            div { class: "motion-cascade stg-policy-grid",
                for policy in POLICIES {
                    div { class: "stg-policy-card",
                        span { class: "stg-policy-icon", IconShield {} }
                        div { class: "stg-policy-body",
                            div { class: "stg-policy-title-row",
                                p { class: "stg-policy-title", { t_key(policy.title_key) } }
                                ToggleField { enabled: policy.enabled }
                            }
                            p { class: "stg-policy-desc", { t_key(policy.desc_key) } }
                        }
                    }
                }
            }
        }
    }
}

struct InfraNode {
    name_key: &'static str,
    meta: &'static str,
    warn: bool,
}

const INFRA_NODES: &[InfraNode] = &[
    InfraNode {
        name_key: "settings-hosting-node-web",
        meta: "99.98% uptime · 42ms",
        warn: false,
    },
    InfraNode {
        name_key: "settings-hosting-node-database",
        meta: "99.99% uptime · 8ms",
        warn: false,
    },
    InfraNode {
        name_key: "settings-hosting-node-cdn",
        meta: "99.95% uptime · 21ms",
        warn: false,
    },
    InfraNode {
        name_key: "settings-hosting-node-email-relay",
        meta: "Delayed queue · 3 retries",
        warn: true,
    },
];

const BACKUP_TICKS: &[bool] = &[true, true, true, true, false, true, true];
const BACKUP_LABEL_KEYS: &[&str] = &[
    "settings-hosting-day-mon",
    "settings-hosting-day-tue",
    "settings-hosting-day-wed",
    "settings-hosting-day-thu",
    "settings-hosting-day-fri",
    "settings-hosting-day-sat",
    "settings-hosting-day-sun",
];

#[component]
pub fn SettingsHosting() -> Element {
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {HOSTING_ACCENT};",
            SectionIntro {
                eyebrow: t_key("settings-hosting-eyebrow"),
                title: t_key("settings-hosting-title"),
                description: t_key("settings-hosting-description"),
                accent: HOSTING_ACCENT,
            }

            div { class: "motion-cascade stg-infra-strip",
                for node in INFRA_NODES {
                    div { class: "stg-infra-node",
                        div { class: "stg-infra-top",
                            span { class: if node.warn { "stg-infra-dot is-warn" } else { "stg-infra-dot" } }
                            span { class: "stg-infra-name", { t_key(node.name_key) } }
                        }
                        p { class: "stg-infra-meta", "{node.meta}" }
                    }
                }
            }

            div { class: "ui-card p-4",
                div { class: "mb-1 flex items-center justify-between gap-3",
                    h2 { class: "text-sm font-semibold text-text", { t!("settings-hosting-backups-title") } }
                    span { class: "text-xs text-text-muted tabular-nums", "Last snapshot 2h ago" }
                }
                div { class: "stg-infra-timeline",
                    for ok in BACKUP_TICKS {
                        span { class: if *ok { "stg-infra-tick" } else { "stg-infra-tick is-missed" } }
                    }
                }
                div { class: "stg-infra-timeline-labels",
                    for label_key in BACKUP_LABEL_KEYS {
                        span { { t_key(label_key) } }
                    }
                }
                div { class: "mt-3 border-t border-border-subtle pt-1",
                    ToggleField {
                        label: t_key("settings-hosting-toggle-nightly-label"),
                        hint: t_key("settings-hosting-toggle-nightly-hint"),
                        enabled: true,
                    }
                }
                div { class: "mt-4",
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, { t!("settings-hosting-open-panel") } }
                }
            }
        }
    }
}
