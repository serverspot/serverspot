use dioxus::prelude::*;

use crate::gravatar::gravatar_url;

/// Circular user avatar loaded from Gravatar for the given email.
#[component]
pub fn Avatar(
    #[props(into)] email: String,
    #[props(default = 32)] size: u32,
    #[props(default, into)] class: String,
    #[props(default, into)] alt: String,
) -> Element {
    let src = gravatar_url(&email, size * 2);
    let px = size.to_string();
    let alt = if alt.is_empty() {
        "User avatar".to_string()
    } else {
        alt
    };

    rsx! {
        img {
            src,
            alt,
            width: "{px}",
            height: "{px}",
            class: "shrink-0 rounded-full object-cover bg-surface-2 {class}",
            style: "width: {px}px; height: {px}px;",
        }
    }
}
