use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Outline,
    Danger,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    IconSm,
}

fn button_base_class(variant: ButtonVariant, size: ButtonSize, full_width: bool) -> String {
    let variant = match variant {
        ButtonVariant::Primary => "ui-btn-primary",
        ButtonVariant::Secondary => "ui-btn-secondary",
        ButtonVariant::Ghost => "ui-btn-ghost",
        ButtonVariant::Outline => "ui-btn-outline",
        ButtonVariant::Danger => "ui-btn-danger",
    };
    let size = match size {
        ButtonSize::Sm => "h-8 px-3 text-xs gap-1.5",
        ButtonSize::Md => "h-10 px-4 text-sm gap-2",
        ButtonSize::IconSm => "h-9 w-9 p-0 justify-center",
    };
    let width = if full_width { " w-full" } else { "" };

    format!(
        "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer {variant} {size}{width}"
    )
}

#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default = "")] class: &'static str,
    #[props(default)] disabled: bool,
    #[props(default)] full_width: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let base = button_base_class(variant, size, full_width);

    if class.is_empty() {
        rsx! {
            button {
                class: "{base}",
                disabled,
                onclick: move |evt| onclick.call(evt),
                {children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{base} {class}",
                disabled,
                onclick: move |evt| onclick.call(evt),
                {children}
            }
        }
    }
}

#[component]
pub fn IconButton(
    children: Element,
    #[props(default = "")] class: &'static str,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let base = "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50";

    if class.is_empty() {
        rsx! {
            button {
                class: base,
                onclick: move |evt| onclick.call(evt),
                {children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{base} {class}",
                onclick: move |evt| onclick.call(evt),
                {children}
            }
        }
    }
}
