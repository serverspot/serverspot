
use dioxus::prelude::*;

fn brand_size_style(size: u32) -> &'static str {
    match size {
        14 => "font-size: 14px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
        16 => "font-size: 16px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
        18 => "font-size: 18px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
        20 => "font-size: 20px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
        24 => "font-size: 24px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
        _ => "font-size: 16px; line-height: 1; width: 1em; height: 1em; display: inline-block;",
    }
}

#[component]
pub fn BrandIcon(
    name: &'static str,
    #[props(default = 16)] size: u32,
    #[props(default = "")] class: &'static str,
) -> Element {
    let style = brand_size_style(size);

    rsx! {
        i {
            class: "fa-brands fa-{name} shrink-0 {class}",
            style: "{style}",
            aria_hidden: "true",
        }
    }
}

#[component]
pub fn IconDiscord(#[props(default = "")] class: &'static str) -> Element {
    rsx! { BrandIcon { name: "discord", class } }
}
