use dioxus::prelude::*;

use crate::router::Route;

use super::ui::*;

#[component]
pub fn PageTransition() -> Element {
    let route = use_route::<Route>();
    let key = format!("{route:?}");

    rsx! {
        div {
            key: "{key}",
            class: "page-enter",
            Outlet::<Route> {}
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
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 border-b border-border-subtle py-3 last:border-0 sm:flex-row sm:items-center sm:justify-between sm:gap-4",
            div {
                class: "min-w-0",
                p { class: "truncate text-sm font-medium text-text", "{title}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
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
