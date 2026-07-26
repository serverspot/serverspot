use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn LoadingScreen() -> Element {
    let mut visible = use_signal(|| true);

    if !visible() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "splash-screen fixed inset-0 z-[100] flex flex-col items-center justify-center",
            style: "background: #1c1c24; color: #f4f4f7;",
            aria_busy: "true",
            aria_live: "polite",
            onanimationend: move |evt| {
                if evt.data().animation_name().contains("splash-screen-lifecycle") {
                    visible.set(false);
                }
            },

            div {
                class: "flex flex-col items-center gap-8 px-6",
                img {
                    src: LOGO,
                    alt: "ServerSpot",
                    class: "splash-logo h-16 w-auto sm:h-20",
                }
                div {
                    class: "splash-bar-track",
                    div { class: "splash-bar-fill" }
                }
                p {
                    class: "splash-label text-xs font-medium tracking-[0.18em] uppercase text-text-muted",
                    "Loading"
                }
            }
        }
    }
}
