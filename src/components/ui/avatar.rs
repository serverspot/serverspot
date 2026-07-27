use dioxus::prelude::*;

use crate::gravatar::gravatar_url;

fn avatar_px(size: u32) -> &'static str {
    match size {
        24 => "24",
        28 => "28",
        32 => "32",
        40 => "40",
        48 => "48",
        _ => "32",
    }
}

#[component]
pub fn Avatar(
    email: &'static str,
    #[props(default = 32)] size: u32,
    #[props(default = "")] class: &'static str,
    #[props(default = "User avatar")] alt: &'static str,
) -> Element {
    let src = use_memo(move || gravatar_url(email, size.saturating_mul(2)));
    let px = avatar_px(size);

    rsx! {
        img {
            src: "{src}",
            alt: "{alt}",
            width: "{px}",
            height: "{px}",
            class: "shrink-0 rounded-full object-cover bg-surface-2 {class}",
            style: "width: {px}px; height: {px}px;",
        }
    }
}
