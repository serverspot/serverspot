use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::brand::BrandMark;

#[component]
pub fn LoadingScreen() -> Element {
    let _lang = i18n();
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
            aria_busy: "true",
            aria_live: "polite",

            div { class: "splash-inner",
                div { class: "splash-mark",
                    BrandMark { class: "splash-mark-base" }
                    BrandMark { class: "splash-mark-fill" }
                }
                p { class: "splash-label", { t!("loading-label") } }
            }
        }
    }
}
