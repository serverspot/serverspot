use dioxus::prelude::*;

use crate::components::brand::BrandMark;
use crate::router::Route;

#[component]
pub fn AdminNotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    let navigator = use_navigator();

    rsx! {
        div { class: "admin-missing",

            BrandMark { class: "admin-missing-mark h-14 w-14" }
            p { class: "admin-missing-brand", "ServerSpot" }
            p { class: "admin-missing-line", "This page isn't in the admin." }
            button {
                r#type: "button",
                class: "admin-missing-link",
                onclick: move |_| {
                    navigator.push(Route::Dashboard {});
                },
                "Back to dashboard"
            }
        }
    }
}
