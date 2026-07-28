use dioxus::prelude::*;

use crate::gravatar::gravatar_url;

fn avatar_px(size: u32) -> &'static str {
    match size {
        24 => "24",
        28 => "28",
        32 => "32",
        40 => "40",
        48 => "48",
        64 => "64",
        72 => "72",
        _ => "32",
    }
}

fn avatar_img_class(extra: &str) -> &'static str {
    match extra {
        "" => "shrink-0 rounded-full object-cover bg-surface-2",
        "ring-2 ring-accent" => "shrink-0 rounded-full object-cover bg-surface-2 ring-2 ring-accent",
        "ring-1 ring-border-subtle" => {
            "shrink-0 rounded-full object-cover bg-surface-2 ring-1 ring-border-subtle"
        }
        "ring-2 ring-border-subtle" => {
            "shrink-0 rounded-full object-cover bg-surface-2 ring-2 ring-border-subtle"
        }
        _ => "shrink-0 rounded-full object-cover bg-surface-2",
    }
}

#[component]
pub fn Avatar(
    #[props(into)] email: String,
    #[props(default = 32)] size: u32,
    #[props(default, into)] class: String,
    #[props(default = "User avatar".to_string(), into)] alt: String,
) -> Element {
    let px = avatar_px(size);
    let img_class = avatar_img_class(&class);

    rsx! {
        MemoAvatar {
            email,
            size,
            px,
            img_class,
            alt,
        }
    }
}

#[component]
fn MemoAvatar(
    email: String,
    size: u32,
    px: &'static str,
    img_class: &'static str,
    alt: String,
) -> Element {
    let src = use_memo(move || gravatar_url(&email, size.saturating_mul(2)));

    rsx! {
        img {
            src: "{src}",
            alt,
            width: px,
            height: px,
            class: img_class,
        }
    }
}
