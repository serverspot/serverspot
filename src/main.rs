#[cfg(feature = "server")]
mod backend;
mod components;
mod gravatar;
mod nav;
mod router;
mod server_funcs;

use dioxus::prelude::*;

use components::loading::LoadingScreen;
use router::Route;

pub const FAVICON: Asset = asset!("/assets/favicon.svg");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "server")]
    backend::launch(App);

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
        LoadingScreen {}
    }
}
