mod components;
mod gravatar;
mod nav;
mod router;
mod server_funcs;
mod user;

#[cfg(feature = "server")]
mod backend;

use dioxus::prelude::*;

use components::community::{
    placeholder_applications, placeholder_leaderboard_boards, placeholder_vote_rewards,
};
use components::content::placeholder_posts;
use components::forum::{placeholder_boards, placeholder_threads};
use components::loading::LoadingScreen;
use components::store::{placeholder_categories, placeholder_coupons, placeholder_products};
use components::support::placeholder_articles;
use router::Route;
use user::placeholder_current_user;

pub const FAVICON: Asset = asset!("/assets/favicon.svg");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const SUPPORT_FORUM_CSS: Asset = asset!("/css-partials/support-forum.css");

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
    let boards = use_signal(placeholder_boards);
    use_context_provider(|| boards);
    let threads = use_signal(placeholder_threads);
    use_context_provider(|| threads);
    let products = use_signal(placeholder_products);
    use_context_provider(|| products);
    let categories = use_signal(placeholder_categories);
    use_context_provider(|| categories);
    let coupons = use_signal(placeholder_coupons);
    use_context_provider(|| coupons);
    let posts = use_signal(placeholder_posts);
    use_context_provider(|| posts);
    let articles = use_signal(placeholder_articles);
    use_context_provider(|| articles);
    let leaderboard_boards = use_signal(placeholder_leaderboard_boards);
    use_context_provider(|| leaderboard_boards);
    let vote_rewards = use_signal(placeholder_vote_rewards);
    use_context_provider(|| vote_rewards);
    let applications = use_signal(placeholder_applications);
    use_context_provider(|| applications);

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, viewport-fit=cover",
        }
        document::Title { "ServerSpot" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700&display=swap",
        }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: SUPPORT_FORUM_CSS }
        Router::<Route> {}

        LoadingScreen {}
    }
}
