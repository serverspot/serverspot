use dioxus::prelude::*;
use crate::components::brand::BrandMark;
#[component]
pub fn LoadingScreen() -> Element {
    let mut mounted = use_signal(|| true);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"await new Promise((resolve) => setTimeout(resolve, 1900)); dioxus.send(true);"#,
            );
            let _: bool = eval.recv().await.unwrap_or(true);
            mounted.set(false);
        });
    });
    if !mounted() {
        return rsx! {};
    }
    rsx! {
        div {
            class: "splash-screen fixed inset-0 z-[100] flex flex-col items-center justify-center",
            style: "background: #1c1c24; color: #f4f4f7;",
            aria_busy: "true",
            aria_live: "polite",
            div { class: "flex flex-col items-center gap-8 px-6",
                BrandMark { class: "splash-logo h-16 w-16 sm:h-20 sm:w-20" }
                div { class: "splash-bar-track",
                    div { class: "splash-bar-fill" }
                }
                p { class: "splash-label text-xs font-medium tracking-[0.18em] uppercase text-text-muted",
                    "Loading"
                }
            }
        }
    }
}
