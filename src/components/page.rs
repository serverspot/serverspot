use dioxus::prelude::*;

use crate::router::Route;

use super::brand::BrandMark;
use super::ui::*;

#[component]
pub fn PageTransition() -> Element {
    let route = use_route::<Route>();
    let section = crate::nav::section_for(&route);
    let theme_ide = crate::nav::is_theme_editor(&route);

    rsx! {
        div {
            key: "{section.label()}",
            class: if theme_ide {
                "flex min-h-0 flex-1 flex-col"
            } else {
                "page-enter"
            },
            Outlet::<Route> {}
        }
    }
}

const REPO_URL: &str = "https://github.com/serverspot/serverspot";

#[component]
pub fn PoweredByFooter() -> Element {
    rsx! {
        footer {
            class: "flex items-center justify-center py-6",
            a {
                href: REPO_URL,
                target: "_blank",
                rel: "noopener noreferrer",
                class: "inline-flex items-center gap-2 text-xs text-text-muted transition-colors hover:text-text-secondary",
                span { "Powered by" }
                BrandMark { class: "h-3.5 w-3.5 opacity-50" }
                span { class: "font-medium", "ServerSpot" }
            }
        }
    }
}

#[component]
pub fn PageHeader(
    title: &'static str,
    #[props(default)] subtitle: &'static str,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: "mb-6 flex flex-wrap items-end justify-between gap-3 sm:mb-8 sm:gap-4",
            div {
                class: "min-w-0",
                h1 { class: "text-2xl font-semibold tracking-tight sm:text-3xl", "{title}" }
                if !subtitle.is_empty() {
                    p { class: "mt-1.5 max-w-2xl text-sm text-text-muted sm:mt-2", "{subtitle}" }
                }
            }
            if let Some(action) = action {
                div { class: "w-full sm:w-auto", {action} }
            }
        }
    }
}

#[component]
pub fn StatPill(label: &'static str, value: &'static str, accent: &'static str) -> Element {
    rsx! {
        div {
            class: "rounded-squircle-lg border border-border-subtle bg-surface/30 px-3 py-2.5 sm:px-4 sm:py-3",
            p { class: "text-xs text-text-muted", "{label}" }
            p {
                class: "mt-1 text-lg font-semibold tabular-nums tracking-tight sm:text-xl",
                style: "color: {accent};",
                "{value}"
            }
        }
    }
}

#[component]
pub fn DataPanel(title: &'static str, children: Element) -> Element {
    rsx! {
        section {
            class: "rounded-squircle-lg border border-border-subtle bg-surface/20 overflow-hidden",
            div {
                class: "border-b border-border-subtle px-4 py-3",
                h2 { class: "text-sm font-medium text-text", "{title}" }
            }
            div { class: "p-4", {children} }
        }
    }
}

#[component]
pub fn RowItem(
    title: &'static str,
    meta: &'static str,
    #[props(default)] trailing: &'static str,
    #[props(default = "")] email: &'static str,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 border-b border-border-subtle py-3 last:border-0 sm:flex-row sm:items-center sm:justify-between sm:gap-4",
            div {
                class: "flex min-w-0 items-center gap-3",
                if !email.is_empty() {
                    Avatar {
                        email,
                        size: 32,
                        alt: title,
                    }
                }
                div {
                    class: "min-w-0",
                    p { class: "truncate text-sm font-medium text-text", "{title}" }
                    p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
                }
            }
            if !trailing.is_empty() {
                span { class: "shrink-0 text-xs text-text-secondary", "{trailing}" }
            }
        }
    }
}

#[component]
pub fn SettingRow(
    title: &'static str,
    description: &'static str,
    #[props(default)] enabled: bool,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-3 border-b border-border-subtle py-4 last:border-0 sm:flex-row sm:items-start sm:justify-between sm:gap-4",
            div {
                class: "min-w-0",
                p { class: "text-sm font-medium text-text", "{title}" }
                p { class: "mt-1 text-sm text-text-muted", "{description}" }
            }
            Button {
                class: "self-start",
                variant: if enabled { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                size: ButtonSize::Sm,
                if enabled { "On" } else { "Off" }
            }
        }
    }
}

#[component]
pub fn FeatureBullet(text: &'static str) -> Element {
    rsx! {
        li {
            class: "flex gap-2.5 text-sm text-text-secondary",
            span {
                class: "mt-1.5 h-1.5 w-1.5 shrink-0 rounded-full bg-accent",
            }
            span { "{text}" }
        }
    }
}

#[component]
pub fn FeatureBullets(children: Element) -> Element {
    rsx! {
        ul { class: "space-y-2.5", {children} }
    }
}

#[component]
pub fn StatusChip(label: &'static str, #[props(default = "#87d1fe")] tone: &'static str) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center rounded-squircle-sm px-2 py-0.5 text-xs font-medium",
            style: "background: color-mix(in srgb, {tone} 16%, transparent); color: {tone};",
            "{label}"
        }
    }
}

#[component]
pub fn InfoCard(title: &'static str, body: &'static str) -> Element {
    rsx! {
        div {
            class: "rounded-squircle-lg border border-border-subtle bg-surface/20 p-4",
            p { class: "text-sm font-medium text-text", "{title}" }
            p { class: "mt-1.5 text-sm text-text-muted", "{body}" }
        }
    }
}
