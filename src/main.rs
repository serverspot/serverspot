mod components;
mod gravatar;
mod nav;
mod router;
mod server_funcs;
mod user;
#[cfg(feature = "server")] mod backend;
use dioxus::prelude::*;
use components::forum::{placeholder_boards, placeholder_threads};
use components::loading::LoadingScreen;
use router::Route;
use user::placeholder_current_user;
pub const FAVICON: Asset = asset!("/assets/favicon.svg");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    #[cfg(feature = "server")] backend::launch(App);
    #[cfg(not(feature = "server"))] dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let current_user = use_signal(placeholder_current_user);
    use_context_provider(|| current_user);
    let boards = use_signal(placeholder_boards);
    use_context_provider(|| boards);
    let threads = use_signal(placeholder_threads);
    use_context_provider(|| threads);
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Title { "ServerSpot" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700;800&display=swap",
        }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
        LoadingScreen {}
    }
}
