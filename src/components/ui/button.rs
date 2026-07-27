use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    IconSm,
}

#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default = "")]
    class: &'static str,
    #[props(default = "")] style: &'static str,
    #[props(default)] disabled: bool,
    #[props(default)] full_width: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "ui-btn-primary",
        ButtonVariant::Secondary => "ui-btn-secondary",
        ButtonVariant::Ghost => "ui-btn-ghost",
        ButtonVariant::Outline => "ui-btn-outline",
    };

    let size_class = match size {
        ButtonSize::Sm => "h-8 px-3 text-xs gap-1.5",
        ButtonSize::Md => "h-10 px-4 text-sm gap-2",
        ButtonSize::IconSm => "h-9 w-9 p-0 justify-center",
    };

    let width = if full_width { "w-full" } else { "" };

    rsx! {
        button {
            class: "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer {variant_class} {size_class} {width} {class}",
            style: "{style}",
            disabled,
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}

#[component]
pub fn IconButton(
    children: Element,
    #[props(default = "")] class: &'static str,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50 {class}",
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}
