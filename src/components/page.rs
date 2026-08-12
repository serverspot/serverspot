use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::router::Route;

use super::brand::BrandMark;
use super::ui::*;

#[component]
pub fn PageTransition() -> Element {
    let route = use_route::<Route>();
    let theme_ide = crate::nav::is_theme_editor(&route);
    let route_key = format!("{route:?}");

    rsx! {
        div {
            key: "{route_key}",
            class: if theme_ide { "flex min-h-0 flex-1 flex-col" } else { "page-enter" },
            Outlet::<Route> {}
        }
    }
}

const REPO_URL: &str = "https://github.com/serverspot/serverspot";

#[component]
pub fn PoweredByFooter() -> Element {
    let _lang = i18n();

    rsx! {
        footer { class: "flex items-center justify-center py-6",
            a {
                href: REPO_URL,
                target: "_blank",
                rel: "noopener noreferrer",
                class: "inline-flex items-center gap-2 text-xs text-text-muted transition-colors hover:text-text-secondary",
                span { { t!("footer-powered-by") } }
                BrandMark { class: "h-3.5 w-3.5 opacity-50" }
                span { class: "font-medium", { t!("brand-name") } }
            }
        }
    }
}

#[component]
pub fn PageHeader(
    #[props(into)] title: String,
    #[props(default, into)] subtitle: String,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "page-header-enter mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                h1 { class: "text-2xl font-semibold tracking-tight text-text sm:text-[1.75rem]",
                    "{title}"
                }
                if !subtitle.is_empty() {
                    p { class: "mt-1.5 max-w-xl text-sm text-text-muted", "{subtitle}" }
                }
            }
            if let Some(action) = action {
                div { class: "w-full sm:w-auto", {action} }
            }
        }
    }
}

#[component]
pub fn StatPill(
    #[props(into)] label: String,
    #[props(into)] value: String,
    #[props(default = "")] accent: &'static str,
) -> Element {
    let _ = accent;
    rsx! {
        div {
            p { class: "text-sm text-text-muted", "{label}" }
            p { class: "mt-1 text-2xl font-semibold tabular-nums tracking-tight text-text",
                "{value}"
            }
        }
    }
}

#[component]
pub fn DataPanel(#[props(into)] title: String, children: Element) -> Element {
    rsx! {
        section { class: "overflow-hidden rounded-lg border border-border-subtle bg-surface",
            div { class: "border-b border-border-subtle px-4 py-3",
                h2 { class: "text-sm font-semibold text-text", "{title}" }
            }
            div { class: "motion-cascade motion-cascade-tight px-4 py-1", {children} }
        }
    }
}

#[component]
pub fn RowItem(
    #[props(into)] title: String,
    #[props(into)] meta: String,
    #[props(default, into)] trailing: String,
    #[props(default, into)] email: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 border-b border-border-subtle py-3 last:border-0 sm:flex-row sm:items-center sm:justify-between sm:gap-4",
            div { class: "flex min-w-0 items-center gap-3",
                if !email.is_empty() {
                    Avatar { email, size: 32, alt: title.clone() }
                }
                div { class: "min-w-0",
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
    #[props(into)] title: String,
    #[props(into)] description: String,
    #[props(default)] enabled: bool,
) -> Element {
    let mut on = use_signal(|| enabled);
    let _lang = i18n();

    rsx! {
        div { class: "flex flex-col gap-3 border-b border-border-subtle py-4 last:border-0 sm:flex-row sm:items-start sm:justify-between sm:gap-4",
            div { class: "min-w-0",
                p { class: "text-sm font-medium text-text", "{title}" }
                p { class: "mt-1 text-sm text-text-muted", "{description}" }
            }
            Button {
                class: "self-start",
                variant: if on() { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let next = !*on.peek();
                    on.set(next);
                },
                if on() {
                    { t!("common-on") }
                } else {
                    { t!("common-off") }
                }
            }
        }
    }
}

#[component]
pub fn FeatureBullet(text: &'static str) -> Element {
    rsx! {
        li { class: "flex gap-2.5 text-sm text-text-secondary",
            span { class: "mt-1.5 h-1.5 w-1.5 shrink-0 rounded-full bg-accent" }
            span { "{text}" }
        }
    }
}

#[component]
pub fn FeatureBullets(children: Element) -> Element {
    rsx! {
        ul { class: "motion-cascade motion-cascade-tight space-y-2.5", {children} }
    }
}

#[component]
pub fn StatusChip(
    #[props(into)] label: String,
    #[props(default = "#006d77".to_string(), into)] tone: String,
) -> Element {
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
        div { class: "rounded-lg border border-border-subtle bg-surface p-4",
            p { class: "text-sm font-semibold text-text", "{title}" }
            p { class: "mt-1.5 text-sm leading-relaxed text-text-muted", "{body}" }
        }
    }
}

#[component]
pub fn FeatureSettingsChrome(#[props(into)] subtitle: String, children: Element) -> Element {
    let _lang = i18n();

    rsx! {
        PageHeader {
            title: crate::i18n::t_key("common-settings"),
            subtitle,
            action: rsx! {
                Button { { t!("common-save-changes") } }
            },
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2", {children} }
    }
}

#[component]
pub fn SettingsField(#[props(into)] label: String, #[props(into)] value: String) -> Element {
    rsx! {
        div { class: "border-b border-border-subtle py-3 last:border-0",
            label { class: "mb-1.5 block text-xs font-medium text-text-muted", "{label}" }
            StaticInput { value, class: "max-w-md" }
        }
    }
}

#[component]
pub fn SettingsControl(#[props(into)] label: String, children: Element) -> Element {
    rsx! {
        div { class: "border-b border-border-subtle py-3 last:border-0",
            label { class: "mb-1.5 block text-xs font-medium text-text-muted", "{label}" }
            {children}
        }
    }
}
