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
    Lg,
    Icon,
    IconSm,
}

#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default, into)] class: String,
    #[props(default)] disabled: bool,
    #[props(default)] full_width: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "ui-btn-primary",
        ButtonVariant::Secondary => "ui-btn-secondary",
        ButtonVariant::Ghost => "ui-btn-ghost",
        ButtonVariant::Outline => "ui-btn-outline",
        ButtonVariant::Danger => "ui-btn-danger",
    };

    let size_class = match size {
        ButtonSize::Sm => "h-8 px-3 text-xs gap-1.5",
        ButtonSize::Md => "h-10 px-4 text-sm gap-2",
        ButtonSize::Lg => "h-11 px-5 text-sm gap-2.5",
        ButtonSize::Icon => "h-10 w-10 p-0 justify-center",
        ButtonSize::IconSm => "h-9 w-9 p-0 justify-center",
    };

    let width = if full_width { "w-full" } else { "" };

    rsx! {
        button {
            class: "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer {variant_class} {size_class} {width} {class}",
            disabled,
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}

#[component]
pub fn IconButton(
    children: Element,
    #[props(default, into)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconSm,
            class: "text-text-muted {class}",
            onclick,
            {children}
        }
    }
}
