#[cfg(feature = "server")]
mod backend;
mod components;
mod gravatar;
mod nav;
mod router;
mod server_funcs;
mod theme;

use dioxus::prelude::*;

use gravatar::placeholder_current_user;
use router::Route;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "server")]
    backend::launch(App);

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let current_user = use_signal(placeholder_current_user);
    use_context_provider(|| current_user);

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Title { "ServerSpot" }
        document::Link { rel: "icon", href: "/uploads/site-favicon" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700&display=swap",
        }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
