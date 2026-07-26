use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Elevated,
    Gradient,
    Soft,
}

#[component]
pub fn Card(
    children: Element,
    #[props(default)] variant: CardVariant,
    #[props(default, into)] class: String,
    #[props(default, into)] padding: String,
) -> Element {
    let variant_class = match variant {
        CardVariant::Default => "bg-surface/40 border border-border-subtle",
        CardVariant::Elevated => "bg-surface border border-border",
        CardVariant::Gradient => "bg-surface border border-border-subtle",
        CardVariant::Soft => "bg-accent-soft border border-border-subtle",
    };

    let pad = if padding.is_empty() {
        "p-5".to_string()
    } else {
        padding
    };

    rsx! {
        div {
            class: "rounded-squircle-lg {variant_class} {pad} {class}",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    title: String,
    #[props(default)] subtitle: String,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: "flex items-start justify-between gap-4 mb-4",
            div {
                class: "min-w-0",
                h3 { class: "text-base font-semibold text-text", "{title}" }
                if !subtitle.is_empty() {
                    p { class: "mt-1 text-sm text-text-secondary", "{subtitle}" }
                }
            }
            if let Some(action) = action {
                {action}
            }
        }
    }
}
