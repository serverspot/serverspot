use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::brand::BrandMark;
use crate::router::Route;

#[component]
pub fn AdminNotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    let navigator = use_navigator();
    let _lang = i18n();

    rsx! {
        div { class: "admin-missing",

            BrandMark { class: "admin-missing-mark h-14 w-14" }
            p { class: "admin-missing-brand", { t!("brand-name") } }
            p { class: "admin-missing-line", { t!("not-found-message") } }
            button {
                r#type: "button",
                class: "admin-missing-link",
                onclick: move |_| {
                    navigator.push(Route::Dashboard {});
                },
                { t!("not-found-back-dashboard") }
            }
        }
    }
}
