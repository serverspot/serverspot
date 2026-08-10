use dioxus::prelude::*;

use crate::components::page::DataPanel;
use crate::components::settings::{SectionIntro, ToggleField};
use crate::components::ui::*;
use crate::router::Route;
use crate::user::CurrentUser;

const BIO: &str = "Owner at ServerSpot. Building tools for game server communities.";
const ACCOUNT_ACCENT: &str = "#b0b3c0";

#[component]
pub fn Account() -> Element {
    let navigator = use_navigator();
    let current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user.read();
    let user_email = user.email.clone();
    let user_name = user.name.clone();
    let user_role = user.role.clone();

    rsx! {
        SectionIntro {
            eyebrow: "You",
            title: "Account",
            description: "Your profile, security, and session preferences.",
            accent: ACCOUNT_ACCENT,
            action: rsx! {
                div { class: "flex flex-wrap gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            navigator.push(Route::Login {});
                        },
                        "Sign out"
                    }
                    Button { "Save changes" }
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
                p { class: "mt-0.5 text-sm text-text-muted", "{user_email}" }
                div { class: "mt-3 flex flex-wrap gap-2",
                    span { class: "inline-flex items-center rounded-squircle-sm bg-accent-soft px-2 py-0.5 text-xs font-medium text-accent",
                        "{user_role}"
                    }
                    span { class: "inline-flex items-center rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-xs font-medium text-text-secondary",
                        "2FA on"
                    }
                }
                div { class: "acct-provider-row mt-3",
                    span { class: "acct-provider-chip", "Discord · connected" }
                    span { class: "acct-provider-chip", "Steam · connected" }
                }
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Profile",
                div { class: "flex flex-col gap-4",
                    Field { label: "Display name",
                        StaticInput { value: user_name.clone() }
                    }
                    Field { label: "Email",
                        StaticInput { value: user_email.clone() }
                    }
                    Field { label: "Bio",
                        StaticInput { value: BIO }
                    }
                }
            }

            DataPanel { title: "Preferences",
                ToggleField {
                    label: "Product emails",
                    hint: "Updates about new features and platform changes.",
                    enabled: true,
                }
                ToggleField {
                    label: "Security alerts",
                    hint: "Notify me about new sign-ins and password changes.",
                    enabled: true,
                }
                ToggleField {
                    label: "Marketing",
                    hint: "Occasional tips and partner offers.",
                    enabled: false,
                }
            }
        }

        div { class: "mt-4 grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Security",
                ToggleField {
                    label: "Two-factor authentication",
                    hint: "Authenticator app required for staff actions.",
                    enabled: true,
                }
                ToggleField {
                    label: "Login notifications",
                    hint: "Email me when a new device signs in.",
                    enabled: true,
                }
                div { class: "flex flex-wrap gap-2 border-t border-border-subtle pt-4",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "Change password"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "View recovery codes"
                    }
                }
            }

            DataPanel { title: "Active sessions",
                SessionRow {
                    badge: "CH",
                    title: "Chrome · Windows",
                    meta: "London, UK · This device",
                    trailing: "Active",
                }
                SessionRow {
                    badge: "SF",
                    title: "Safari · iPhone",
                    meta: "London, UK · 2 hours ago",
                    trailing: "Revoke",
                }
                SessionRow {
                    badge: "FF",
                    title: "Firefox · macOS",
                    meta: "Manchester, UK · Yesterday",
                    trailing: "Revoke",
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
    trailing: &'static str,
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
fn Field(label: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1.5",
            label { class: "text-xs font-medium text-text-muted", "{label}" }
            {children}
        }
    }
}
