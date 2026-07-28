use dioxus::prelude::*;

use crate::gravatar::gravatar_url;

fn avatar_size_class(size: u32) -> &'static str {
    match size {
        24 => "h-6 w-6",
        28 => "h-7 w-7",
        32 => "h-8 w-8",
        40 => "h-10 w-10",
        44 => "h-11 w-11",
        48 => "h-12 w-12",
        64 => "h-16 w-16",
        72 => "h-[4.5rem] w-[4.5rem]",
        _ => "h-8 w-8",
    }
}

#[component]
pub fn Avatar(
    #[props(into)] email: String,
    #[props(default = 32)] size: u32,
    #[props(default, into)] class: String,
    #[props(default = "User avatar".to_string(), into)] alt: String,
) -> Element {
    let size_class = avatar_size_class(size);
    let img_class = if class.is_empty() {
        format!("shrink-0 self-start rounded-full object-cover bg-surface-2 {size_class}")
    } else {
        format!("shrink-0 self-start rounded-full object-cover bg-surface-2 {size_class} {class}")
    };

    rsx! {
        MemoAvatar {
            email,
            size,
            img_class,
            alt,
        }
    }
}

#[component]
fn MemoAvatar(email: String, size: u32, img_class: String, alt: String) -> Element {
    let src = use_memo(move || gravatar_url(&email, size.saturating_mul(2)));

    rsx! {
        img {
            src: "{src}",
            alt,
            class: "{img_class}",
        }
    }
}
