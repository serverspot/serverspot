use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::DataPanel;
use crate::components::settings::{SectionIntro, ToggleField};
use crate::components::ui::*;
use crate::i18n::{apply_user_locale, t_key};
use crate::router::Route;
use crate::user::CurrentUser;

const BIO: &str = "Owner at ServerSpot. Building tools for game server communities.";
const ACCOUNT_ACCENT: &str = "#b0b3c0";

const LOCALES: &[(&str, &str)] = &[
    ("en-US", "lang-en"),
    ("fr-FR", "lang-fr"),
    ("de-DE", "lang-de"),
    ("es-ES", "lang-es"),
];

#[component]
pub fn Account() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user.read();
    let user_email = user.email.clone();
    let user_username = user.username.clone();
    let user_name = user.name.clone();
    let user_role = user.role.clone();
    let user_locale = user.locale.clone();

    rsx! {
        SectionIntro {
            eyebrow: t!("account-eyebrow"),
            title: t!("account-title"),
            description: t!("account-description"),
            accent: ACCOUNT_ACCENT,
            action: rsx! {
                div { class: "flex flex-wrap gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            navigator.push(Route::Login {});
                        },
                        { t!("account-action-sign-out") }
                    }
                    Button { { t!("account-action-save") } }
                }
            },
        }

        section { class: "acct-profile-strip",
            Avatar {
                email: user_email.clone(),
                size: 72,
                alt: user_name.clone(),
                class: "ring-2 ring-border-subtle",
            }
            div { class: "min-w-0 flex-1",
                p { class: "text-lg font-semibold tracking-tight", "{user_name}" }
                p { class: "mt-0.5 text-sm text-text-muted", "@{user_username}" }
                div { class: "mt-3 flex flex-wrap gap-2",
                    span { class: "inline-flex items-center rounded-squircle-sm bg-accent-soft px-2 py-0.5 text-xs font-medium text-accent",
                        "{user_role}"
                    }
                    span { class: "inline-flex items-center rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-xs font-medium text-text-secondary",
                        { t!("account-badge-2fa-on") }
                    }
                }
                div { class: "acct-provider-row mt-3",
                    span { class: "acct-provider-chip", { t!("account-provider-discord") } }
                    span { class: "acct-provider-chip", { t!("account-provider-steam") } }
                }
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: t!("account-panel-profile"),
                div { class: "flex flex-col gap-4",
                    Field { label: t!("account-field-display-name"),
                        StaticInput { value: user_name.clone() }
                    }
                    Field { label: t!("account-field-username"),
                        StaticInput { value: user_username.clone() }
                    }
                    Field { label: t!("account-field-bio"),
                        StaticInput { value: BIO }
                    }
                }
            }

            DataPanel { title: t!("account-panel-preferences"),
                div { class: "flex flex-col gap-1.5 border-b border-border-subtle pb-4",
                    label { class: "text-xs font-medium text-text-muted", { t!("account-language-label") } }
                    p { class: "mb-2 text-xs text-text-muted", { t!("account-language-hint") } }
                    select {
                        class: "ui-input w-full",
                        value: "{user_locale}",
                        onchange: move |evt| {
                            let locale = evt.value();
                            current_user.write().locale = locale.clone();
                            apply_user_locale(&locale);
                        },
                        for (code, label_key) in LOCALES {
                            option {
                                value: "{code}",
                                selected: user_locale == *code,
                                { t_key(label_key) }
                            }
                        }
                    }
                }
                ToggleField {
                    label: t!("account-toggle-product-updates-label"),
                    hint: t!("account-toggle-product-updates-hint"),
                    enabled: true,
                }
                ToggleField {
                    label: t!("account-toggle-security-alerts-label"),
                    hint: t!("account-toggle-security-alerts-hint"),
                    enabled: true,
                }
                ToggleField {
                    label: t!("account-toggle-marketing-label"),
                    hint: t!("account-toggle-marketing-hint"),
                    enabled: false,
                }
            }
        }

        div { class: "mt-4 grid gap-4 lg:grid-cols-2",
            DataPanel { title: t!("account-panel-security"),
                ToggleField {
                    label: t!("account-toggle-2fa-label"),
                    hint: t!("account-toggle-2fa-hint"),
                    enabled: true,
                }
                ToggleField {
                    label: t!("account-toggle-login-notifications-label"),
                    hint: t!("account-toggle-login-notifications-hint"),
                    enabled: true,
                }
                div { class: "flex flex-wrap gap-2 border-t border-border-subtle pt-4",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        { t!("account-action-change-password") }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        { t!("account-action-recovery-codes") }
                    }
                }
            }

            DataPanel { title: t!("account-panel-sessions"),
                SessionRow {
                    badge: "CH",
                    title: "Chrome · Windows".to_string(),
                    meta: format!("London, UK · {}", t!("account-session-this-device")),
                    trailing: t!("account-session-active"),
                }
                SessionRow {
                    badge: "SF",
                    title: "Safari · iPhone".to_string(),
                    meta: "London, UK · 2 hours ago".to_string(),
                    trailing: t!("account-session-revoke"),
                }
                SessionRow {
                    badge: "FF",
                    title: "Firefox · macOS".to_string(),
                    meta: "Manchester, UK · Yesterday".to_string(),
                    trailing: t!("account-session-revoke"),
                }
            }
        }
    }
}

#[component]
fn SessionRow(
    badge: &'static str,
    #[props(into)] title: String,
    #[props(into)] meta: String,
    #[props(into)] trailing: String,
) -> Element {
    rsx! {
        div { class: "acct-session-row",
            span { class: "acct-session-icon", "{badge}" }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-sm font-medium text-text", "{title}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
            }
            span { class: "shrink-0 text-xs text-text-secondary", "{trailing}" }
        }
    }
}

#[component]
fn Field(#[props(into)] label: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1.5",
            label { class: "text-xs font-medium text-text-muted", "{label}" }
            {children}
        }
    }
}
