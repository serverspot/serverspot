pub mod analytics;
pub mod blog;
pub mod community;
pub mod content;
pub mod dashboard;
pub mod forum;
pub mod home;
pub mod loading;
pub mod page;
pub mod settings;
pub mod shell;
pub mod store;
pub mod support;
pub mod ui;

use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "flex gap-4 p-4 text-sm",
            Link {
                to: Route::Dashboard {},
                "Dashboard"
            }
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
