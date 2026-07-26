use dioxus::prelude::*;

#[component]
pub fn Badge(
    children: Element,
    #[props(default, into)] class: String,
    #[props(default, into)] tone: String,
) -> Element {
    let tone_class = match tone.as_str() {
        "success" => "bg-success/15 text-success",
        "danger" => "bg-danger/15 text-danger",
        "accent" => "bg-accent-soft text-accent",
        _ => "bg-surface-3 text-text-secondary",
    };

    rsx! {
        span {
            class: "inline-flex items-center rounded-squircle-sm px-2 py-0.5 text-[10px] font-bold tracking-wide uppercase badge-pop {tone_class} {class}",
            {children}
        }
    }
}
