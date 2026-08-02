use dioxus::prelude::*;
use crate::components::page::{DataPanel, PageHeader, RowItem, SettingRow};
use crate::components::ui::*;
use crate::router::Route;
use crate::user::CurrentUser;
const BIO: &str = "Owner at ServerSpot. Building tools for game server communities.";
#[component]
pub fn Account() -> Element {
    let navigator = use_navigator();
    let current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user.read();
    let user_email = user.email.clone();
    let user_name = user.name.clone();
    rsx! {
        PageHeader {
            title: "Account",
            subtitle: "Your profile, security, and session preferences.",
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
        }
        section { class: "mb-6 flex flex-col gap-4 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4 sm:mb-8 sm:flex-row sm:items-center sm:gap-5 sm:p-5",
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
                        "Owner"
                    }
                    span { class: "inline-flex items-center rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-xs font-medium text-text-secondary",
                        "2FA on"
                    }
                }
            }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
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
                SettingRow {
                    title: "Product emails",
                    description: "Updates about new features and platform changes.",
                    enabled: true,
                }
                SettingRow {
                    title: "Security alerts",
                    description: "Notify me about new sign-ins and password changes.",
                    enabled: true,
                }
                SettingRow {
                    title: "Marketing",
                    description: "Occasional tips and partner offers.",
                    enabled: false,
                }
            }
        }
        div { class: "mt-4 grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Security",
                SettingRow {
                    title: "Two-factor authentication",
                    description: "Authenticator app required for staff actions.",
                    enabled: true,
                }
                SettingRow {
                    title: "Login notifications",
                    description: "Email me when a new device signs in.",
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
                RowItem {
                    title: "Chrome · Windows",
                    meta: "London, UK · This device",
                    trailing: "Active",
                }
                RowItem {
                    title: "Safari · iPhone",
                    meta: "London, UK · 2 hours ago",
                    trailing: "Revoke",
                }
                RowItem {
                    title: "Firefox · macOS",
                    meta: "Manchester, UK · Yesterday",
                    trailing: "Revoke",
                }
            }
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
