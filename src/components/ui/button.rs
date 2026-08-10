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
    IconMd,
}

fn button_base_class(variant: ButtonVariant, size: ButtonSize, full_width: bool) -> &'static str {
    match (variant, size, full_width) {
        (ButtonVariant::Primary, ButtonSize::Sm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-8 px-3 text-xs gap-1.5"
        }
        (ButtonVariant::Primary, ButtonSize::Sm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-8 px-3 text-xs gap-1.5 w-full"
        }
        (ButtonVariant::Primary, ButtonSize::Md, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-10 px-4 text-sm gap-2"
        }
        (ButtonVariant::Primary, ButtonSize::Md, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-10 px-4 text-sm gap-2 w-full"
        }
        (ButtonVariant::Primary, ButtonSize::IconSm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-9 w-9 p-0 justify-center"
        }
        (ButtonVariant::Primary, ButtonSize::IconSm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-9 w-9 p-0 justify-center w-full"
        }
        (ButtonVariant::Primary, ButtonSize::IconMd, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-10 w-10 p-0 justify-center"
        }
        (ButtonVariant::Primary, ButtonSize::IconMd, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-primary h-10 w-10 p-0 justify-center w-full"
        }
        (ButtonVariant::Secondary, ButtonSize::Sm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-8 px-3 text-xs gap-1.5"
        }
        (ButtonVariant::Secondary, ButtonSize::Sm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-8 px-3 text-xs gap-1.5 w-full"
        }
        (ButtonVariant::Secondary, ButtonSize::Md, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-10 px-4 text-sm gap-2"
        }
        (ButtonVariant::Secondary, ButtonSize::Md, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-10 px-4 text-sm gap-2 w-full"
        }
        (ButtonVariant::Secondary, ButtonSize::IconSm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-9 w-9 p-0 justify-center"
        }
        (ButtonVariant::Secondary, ButtonSize::IconSm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-9 w-9 p-0 justify-center w-full"
        }
        (ButtonVariant::Secondary, ButtonSize::IconMd, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-10 w-10 p-0 justify-center"
        }
        (ButtonVariant::Secondary, ButtonSize::IconMd, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-secondary h-10 w-10 p-0 justify-center w-full"
        }
        (ButtonVariant::Ghost, ButtonSize::Sm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-8 px-3 text-xs gap-1.5"
        }
        (ButtonVariant::Ghost, ButtonSize::Sm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-8 px-3 text-xs gap-1.5 w-full"
        }
        (ButtonVariant::Ghost, ButtonSize::Md, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-10 px-4 text-sm gap-2"
        }
        (ButtonVariant::Ghost, ButtonSize::Md, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-10 px-4 text-sm gap-2 w-full"
        }
        (ButtonVariant::Ghost, ButtonSize::IconSm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-9 w-9 p-0 justify-center"
        }
        (ButtonVariant::Ghost, ButtonSize::IconSm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-9 w-9 p-0 justify-center w-full"
        }
        (ButtonVariant::Ghost, ButtonSize::IconMd, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-10 w-10 p-0 justify-center"
        }
        (ButtonVariant::Ghost, ButtonSize::IconMd, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-ghost h-10 w-10 p-0 justify-center w-full"
        }
        (ButtonVariant::Outline, ButtonSize::Sm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-8 px-3 text-xs gap-1.5"
        }
        (ButtonVariant::Outline, ButtonSize::Sm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-8 px-3 text-xs gap-1.5 w-full"
        }
        (ButtonVariant::Outline, ButtonSize::Md, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-10 px-4 text-sm gap-2"
        }
        (ButtonVariant::Outline, ButtonSize::Md, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-10 px-4 text-sm gap-2 w-full"
        }
        (ButtonVariant::Outline, ButtonSize::IconSm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-9 w-9 p-0 justify-center"
        }
        (ButtonVariant::Outline, ButtonSize::IconSm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-9 w-9 p-0 justify-center w-full"
        }
        (ButtonVariant::Outline, ButtonSize::IconMd, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-10 w-10 p-0 justify-center"
        }
        (ButtonVariant::Outline, ButtonSize::IconMd, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-outline h-10 w-10 p-0 justify-center w-full"
        }
        (ButtonVariant::Danger, ButtonSize::Sm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-8 px-3 text-xs gap-1.5"
        }
        (ButtonVariant::Danger, ButtonSize::Sm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-8 px-3 text-xs gap-1.5 w-full"
        }
        (ButtonVariant::Danger, ButtonSize::Md, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-10 px-4 text-sm gap-2"
        }
        (ButtonVariant::Danger, ButtonSize::Md, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-10 px-4 text-sm gap-2 w-full"
        }
        (ButtonVariant::Danger, ButtonSize::IconSm, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-9 w-9 p-0 justify-center"
        }
        (ButtonVariant::Danger, ButtonSize::IconSm, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-9 w-9 p-0 justify-center w-full"
        }
        (ButtonVariant::Danger, ButtonSize::IconMd, false) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-10 w-10 p-0 justify-center"
        }
        (ButtonVariant::Danger, ButtonSize::IconMd, true) => {
            "ui-btn ui-squircle inline-flex items-center justify-center font-semibold disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer ui-btn-danger h-10 w-10 p-0 justify-center w-full"
        }
    }
}

#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default = "")] class: &'static str,
    #[props(default = "")] style: &'static str,
    #[props(default = "")] tip: &'static str,
    #[props(default = "")] aria_label: &'static str,
    #[props(default)] disabled: bool,
    #[props(default)] full_width: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let base = button_base_class(variant, size, full_width);
    let owned;
    let class_name = if class.is_empty() {
        base
    } else {
        owned = format!("{base} {class}");
        owned.as_str()
    };
    let has_tip = !tip.is_empty();
    let resolved_aria = if !aria_label.is_empty() {
        aria_label
    } else if has_tip {
        tip
    } else {
        ""
    };

    rsx! {
        button {
            r#type: "button",
            class: "{class_name}",
            style: if style.is_empty() { "" } else { style },
            disabled,
            title: if has_tip { tip } else { "" },
            "aria-label": if resolved_aria.is_empty() { "" } else { resolved_aria },
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
    const BASE: &str = "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50";
    let class_name = match class {
        "" => BASE,
        "shrink-0" => "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50 shrink-0",
        "shrink-0 md:hidden" => "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50 shrink-0 md:hidden",
        "max-lg:hidden" => "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted disabled:cursor-not-allowed disabled:opacity-50 max-lg:hidden",
        _ => BASE,
    };

    rsx! {
        button {
            r#type: "button",
            class: "{class_name}",
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}
