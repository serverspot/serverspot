//! Brand marks via Font Awesome Brands (CDN loaded in `App`).

use dioxus::prelude::*;

/// Font Awesome brand icon (`fa-brands fa-{name}`).
#[component]
pub fn BrandIcon(
    /// Icon name without the `fa-` prefix, e.g. `"discord"`.
    #[props(into)]
    name: String,
    #[props(default = 16)] size: u32,
    #[props(default, into)] class: String,
) -> Element {
    let classes = format!("fa-brands fa-{name} shrink-0 {class}");
    let style =
        format!("font-size: {size}px; line-height: 1; width: 1em; height: 1em; display: inline-block;");

    rsx! {
        i {
            class: "{classes}",
            style: "{style}",
            aria_hidden: "true",
        }
    }
}

#[component]
pub fn IconDiscord(#[props(default, into)] class: String) -> Element {
    rsx! { BrandIcon { name: "discord", class } }
}
