use dioxus::prelude::*;

use crate::theme::{
    css_custom_properties, get_theme_config, load_merged_config, public_content_security_policy,
    read_theme_source_shared, render_template, sanitize_css_bundle, sanitize_css_image_url,
    theme_css_bundle, TemplateContext, TemplateValue, ThemeConfigState, ACTIVE_THEME,
};

const HEADER_BG: Asset = asset!("/assets/theme/header-bg.png");

#[component]
pub fn PublicShell() -> Element {
    let config = use_server_future(get_theme_config)?;
    let state = match config() {
        Some(Ok(state)) => state,
        _ => load_merged_config(ACTIVE_THEME).unwrap_or_else(|_| ThemeConfigState {
            schema: crate::theme::ThemeSchema {
                name: "Default".into(),
                description: String::new(),
                options: Vec::new(),
            },
            values: Default::default(),
        }),
    };
    let config_css =
        sanitize_css_bundle(&css_custom_properties(&state.schema, &state.values));
    let header_image = match state.values.get("header_image").map(String::as_str) {
        Some("") | None => format!("{HEADER_BG}"),
        Some("/assets/theme/header-bg.png") | Some("/assets/theme/header-bg.jpg") => {
            format!("{HEADER_BG}")
        }
        Some(path) => sanitize_css_image_url(path).unwrap_or_else(|| format!("{HEADER_BG}")),
    };
    use_context_provider(|| state);

    rsx! {
        document::Meta {
            "http-equiv": "Content-Security-Policy",
            content: "{public_content_security_policy()}",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css",
        }
        style {
            dangerous_inner_html: ":root {{ --spot-header-image: url(\"{header_image}\"); }} {config_css} html, body {{ height: auto !important; min-height: 100% !important; overflow-x: hidden !important; overflow-y: auto !important; }}",
        }
        div { class: "spot-public min-h-dvh bg-[#f2f4f9] text-[#1f2533]",
            Outlet::<crate::router::Route> {}
        }
    }
}

#[component]
pub fn PublicHome() -> Element {
    render_page(
        "home",
        "Home · ServerSpot",
        &format!("{ACTIVE_THEME}/index.html"),
        home_context(),
    )
}

#[component]
pub fn PublicLogin() -> Element {
    render_page(
        "home",
        "Log in · ServerSpot",
        &format!("{ACTIVE_THEME}/login.html"),
        basic_context(),
    )
}

#[component]
pub fn PublicProfile() -> Element {
    render_page(
        "home",
        "Profile · ServerSpot",
        &format!("{ACTIVE_THEME}/profile.html"),
        profile_context(),
    )
}

#[component]
pub fn PublicForumIndex() -> Element {
    render_page(
        "forum",
        "Forum · ServerSpot",
        &format!("{ACTIVE_THEME}/forum/index.html"),
        forum_index_context(),
    )
}

#[component]
pub fn PublicForumThread(id: u32) -> Element {
    render_page(
        "forum",
        "Thread · ServerSpot",
        &format!("{ACTIVE_THEME}/forum/thread.html"),
        forum_thread_context(id),
    )
}

#[component]
pub fn PublicStore() -> Element {
    render_page(
        "store",
        "Store · ServerSpot",
        &format!("{ACTIVE_THEME}/store/index.html"),
        list_context("store", store_items()),
    )
}

#[component]
pub fn PublicStoreProduct(id: u32) -> Element {
    render_page(
        "store",
        "Product · ServerSpot",
        &format!("{ACTIVE_THEME}/store/product.html"),
        detail_context(store_items(), id),
    )
}

#[component]
pub fn PublicSupport() -> Element {
    render_page(
        "support",
        "Support · ServerSpot",
        &format!("{ACTIVE_THEME}/support/index.html"),
        list_context("support", support_items()),
    )
}

#[component]
pub fn PublicSupportTicket(id: u32) -> Element {
    render_page(
        "support",
        "Ticket · ServerSpot",
        &format!("{ACTIVE_THEME}/support/ticket.html"),
        detail_context(support_items(), id),
    )
}

#[component]
pub fn PublicBlog() -> Element {
    render_page(
        "blog",
        "Blog · ServerSpot",
        &format!("{ACTIVE_THEME}/blog/index.html"),
        list_context("blog", blog_items()),
    )
}

#[component]
pub fn PublicBlogPost(id: u32) -> Element {
    render_page(
        "blog",
        "Post · ServerSpot",
        &format!("{ACTIVE_THEME}/blog/post.html"),
        detail_context(blog_items(), id),
    )
}

#[component]
pub fn PublicPlayers() -> Element {
    render_page(
        "players",
        "Players · ServerSpot",
        &format!("{ACTIVE_THEME}/players/index.html"),
        list_context("players", player_items()),
    )
}

#[component]
pub fn PublicPlayer(id: u32) -> Element {
    render_page(
        "players",
        "Player · ServerSpot",
        &format!("{ACTIVE_THEME}/players/profile.html"),
        detail_context(player_items(), id),
    )
}

#[component]
pub fn PublicLeaderboards() -> Element {
    render_page(
        "leaderboards",
        "Leaderboards · ServerSpot",
        &format!("{ACTIVE_THEME}/leaderboards/index.html"),
        list_context("leaderboards", leaderboard_items()),
    )
}

#[component]
pub fn PublicLeaderboard(id: u32) -> Element {
    render_page(
        "leaderboards",
        "Board · ServerSpot",
        &format!("{ACTIVE_THEME}/leaderboards/board.html"),
        leaderboard_detail_context(id),
    )
}

#[component]
pub fn PublicVotes() -> Element {
    render_page(
        "votes",
        "Votes · ServerSpot",
        &format!("{ACTIVE_THEME}/votes/index.html"),
        list_context("votes", vote_items()),
    )
}

#[component]
pub fn PublicVotesClaim(id: u32) -> Element {
    render_page(
        "votes",
        "Claim · ServerSpot",
        &format!("{ACTIVE_THEME}/votes/claim.html"),
        detail_context(vote_items(), id),
    )
}

#[component]
pub fn PublicApplications() -> Element {
    render_page(
        "applications",
        "Applications · ServerSpot",
        &format!("{ACTIVE_THEME}/applications/index.html"),
        list_context("applications", application_items()),
    )
}

#[component]
pub fn PublicApplicationsForm(id: u32) -> Element {
    render_page(
        "applications",
        "Form · ServerSpot",
        &format!("{ACTIVE_THEME}/applications/form.html"),
        detail_context(application_items(), id),
    )
}

#[component]
pub fn PublicAnalytics() -> Element {
    render_page(
        "analytics",
        "Analytics · ServerSpot",
        &format!("{ACTIVE_THEME}/analytics/index.html"),
        list_context("analytics", analytics_items()),
    )
}

fn render_page(
    feature: &str,
    title: &str,
    template_path: &str,
    mut ctx: TemplateContext,
) -> Element {
    if let Some(state) = try_use_context::<ThemeConfigState>() {
        ctx.set_config(state.values.clone());
    } else if let Ok(state) = load_merged_config(ACTIVE_THEME) {
        ctx.set_config(state.values);
    }
    let css = sanitize_css_bundle(&theme_css_bundle(ACTIVE_THEME, feature));
    let source = read_theme_source_shared(template_path);
    let title = title.to_string();
    let script_src = format!("/theme-runtime/{feature}/scripts.js");
    rsx! {
        // Inline text children get a hydration marker comment, which CSS parses as the
        // start of a selector and swallows the first rule. Raw HTML avoids the marker.
        style { dangerous_inner_html: "{css}" }
        document::Title { "{title}" }
        {
            match source {
                Some(html) => render_template(&html, &ctx),
                None => rsx! {
                    div { class: "spot-error", "Missing theme template: {template_path}" }
                },
            }
        }
        script {
            src: "{script_src}",
        }
    }
}

fn site_context(ctx: &mut TemplateContext) {
    let site = object([
        ("name", TemplateValue::from("ServerSpot")),
        (
            "tagline",
            TemplateValue::from("Community hub for your game server."),
        ),
        ("logo", TemplateValue::from("/uploads/site-logo")),
    ]);
    ctx.insert(
        "site",
        site,
    );
}

fn basic_context() -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);
    ctx
}

fn profile_context() -> TemplateContext {
    let mut ctx = basic_context();
    ctx.insert(
        "user",
        object([
            ("initial", TemplateValue::from("C")),
            ("name", TemplateValue::from("Charlie")),
            ("role", TemplateValue::from("Community Member")),
            ("joined", TemplateValue::from("March 2024")),
            ("playtime", TemplateValue::from("148h")),
            ("vote_streak", TemplateValue::from("12 days")),
            ("tickets", TemplateValue::from("1")),
            ("rank", TemplateValue::from("VIP")),
            ("email", TemplateValue::from("charlie@example.com")),
            ("minecraft", TemplateValue::from("CharlieBuilds")),
            ("discord", TemplateValue::from("@charlie")),
        ]),
    );
    ctx
}

fn home_context() -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);
    ctx.insert("features_count", TemplateValue::from("8"));
    ctx.insert(
        "features",
        TemplateValue::Array(vec![
            feature_link(
                "F",
                "fa-solid fa-comments",
                "Forum",
                "Boards and community discussion.",
                "/forum",
            ),
            feature_link(
                "S",
                "fa-solid fa-cart-shopping",
                "Store",
                "Ranks, crates, and cosmetics.",
                "/store",
            ),
            feature_link(
                "T",
                "fa-solid fa-life-ring",
                "Support",
                "Tickets and help articles.",
                "/support",
            ),
            feature_link(
                "B",
                "fa-solid fa-newspaper",
                "Blog",
                "News and updates.",
                "/blog",
            ),
            feature_link(
                "P",
                "fa-solid fa-user-group",
                "Players",
                "Public player profiles.",
                "/players",
            ),
            feature_link(
                "L",
                "fa-solid fa-ranking-star",
                "Leaderboards",
                "Rankings across modes.",
                "/leaderboards",
            ),
            feature_link(
                "V",
                "fa-solid fa-gift",
                "Vote Rewards",
                "Claim daily vote rewards.",
                "/votes",
            ),
            feature_link(
                "A",
                "fa-solid fa-file-signature",
                "Applications",
                "Staff and builder apps.",
                "/applications",
            ),
        ]),
    );
    ctx
}

fn feature_link(
    initial: &str,
    icon: &str,
    name: &str,
    description: &str,
    href: &str,
) -> TemplateValue {
    object([
        ("initial", TemplateValue::from(initial)),
        ("icon", TemplateValue::from(icon)),
        ("name", TemplateValue::from(name)),
        ("description", TemplateValue::from(description)),
        ("href", TemplateValue::from(href)),
    ])
}

struct SeedItem {
    id: u32,
    name: &'static str,
    description: &'static str,
    meta: &'static str,
    href: String,
}

fn list_context(feature: &str, items: Vec<SeedItem>) -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);
    let values: Vec<TemplateValue> = items.into_iter().map(seed_to_value).collect();
    ctx.insert("items", TemplateValue::Array(values));
    let _ = feature;
    ctx
}

fn detail_context(items: Vec<SeedItem>, id: u32) -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);
    if let Some(item) = items.into_iter().find(|item| item.id == id) {
        ctx.insert("item", seed_to_value(item));
    } else {
        ctx.insert("item", TemplateValue::Null);
    }
    ctx
}

fn seed_to_value(item: SeedItem) -> TemplateValue {
    let initial = item.name.chars().next().unwrap_or('?').to_string();
    object([
        ("id", TemplateValue::from(item.id)),
        ("name", TemplateValue::from(item.name)),
        ("description", TemplateValue::from(item.description)),
        ("meta", TemplateValue::from(item.meta)),
        ("href", TemplateValue::from(item.href.as_str())),
        ("initial", TemplateValue::from(initial.as_str())),
    ])
}

fn store_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "VIP Rank",
            description: "Priority queue, /fly in spawn, and monthly crate keys.",
            meta: "£9.99",
            href: "/store/product/1".into(),
        },
        SeedItem {
            id: 2,
            name: "MVP Rank",
            description: "Everything in VIP plus particle trails and nick colors.",
            meta: "£19.99",
            href: "/store/product/2".into(),
        },
        SeedItem {
            id: 3,
            name: "Crate Key Bundle",
            description: "Ten premium crate keys delivered on purchase.",
            meta: "£4.99",
            href: "/store/product/3".into(),
        },
    ]
}

fn support_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Can't connect",
            description: "Connection timed out after the last restart.",
            meta: "Open",
            href: "/support/ticket/1".into(),
        },
        SeedItem {
            id: 2,
            name: "Store payment pending",
            description: "Paid for VIP but rank not applied yet.",
            meta: "Pending",
            href: "/support/ticket/2".into(),
        },
    ]
}

fn blog_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Season 4 launch",
            description: "New spawn, economy tweaks, and vote milestones.",
            meta: "News · 2d ago",
            href: "/blog/post/1".into(),
        },
        SeedItem {
            id: 2,
            name: "Builder applications open",
            description: "We're hiring builders for the hub refresh.",
            meta: "Community · 5d ago",
            href: "/blog/post/2".into(),
        },
    ]
}

fn player_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "NovaCraft",
            description: "Survival regular · 412 hours played.",
            meta: "Online",
            href: "/players/1".into(),
        },
        SeedItem {
            id: 2,
            name: "SkyBuilder",
            description: "Lead builder · gothic roofs specialist.",
            meta: "Away",
            href: "/players/2".into(),
        },
    ]
}

fn leaderboard_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Monthly Votes",
            description: "Top voters this month.",
            meta: "48 entries",
            href: "/leaderboards/1".into(),
        },
        SeedItem {
            id: 2,
            name: "Playtime",
            description: "Most hours on survival worlds.",
            meta: "120 entries",
            href: "/leaderboards/2".into(),
        },
    ]
}

fn leaderboard_detail_context(id: u32) -> TemplateContext {
    let mut ctx = detail_context(leaderboard_items(), id);
    let ranks = match id {
        1 => vec![
            rank_row(1, "NovaCraft", "128"),
            rank_row(2, "SkyBuilder", "97"),
            rank_row(3, "QuietLeaf", "84"),
            rank_row(4, "Charlie Admin", "71"),
            rank_row(5, "AshWalker", "63"),
        ],
        2 => vec![
            rank_row(1, "SkyBuilder", "412h"),
            rank_row(2, "NovaCraft", "388h"),
            rank_row(3, "QuietLeaf", "301h"),
            rank_row(4, "AshWalker", "274h"),
            rank_row(5, "MossKnight", "219h"),
        ],
        _ => Vec::new(),
    };
    ctx.insert("ranks", TemplateValue::Array(ranks));
    ctx
}

fn rank_row(place: u32, name: &str, score: &str) -> TemplateValue {
    object([
        ("place", TemplateValue::from(place)),
        ("name", TemplateValue::from(name)),
        ("score", TemplateValue::from(score)),
    ])
}

fn vote_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Minecraft-MP",
            description: "Claim your daily vote key.",
            meta: "Ready",
            href: "/votes/claim/1".into(),
        },
        SeedItem {
            id: 2,
            name: "Planet Minecraft",
            description: "Vote streak bonus available.",
            meta: "Ready",
            href: "/votes/claim/2".into(),
        },
    ]
}

fn application_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Builder application",
            description: "Show us your best builds and style.",
            meta: "Open",
            href: "/applications/form/1".into(),
        },
        SeedItem {
            id: 2,
            name: "Helper application",
            description: "Help moderate chat and tickets.",
            meta: "Open",
            href: "/applications/form/2".into(),
        },
    ]
}

fn analytics_items() -> Vec<SeedItem> {
    vec![
        SeedItem {
            id: 1,
            name: "Website traffic",
            description: "Public overview of visits this week.",
            meta: "12.4k",
            href: "/analytics".into(),
        },
        SeedItem {
            id: 2,
            name: "Players online",
            description: "Peak concurrent players today.",
            meta: "186",
            href: "/analytics".into(),
        },
    ]
}

fn board(
    id: u32,
    slug: &str,
    name: &str,
    description: &str,
    threads: u32,
    today: u32,
) -> TemplateValue {
    let href = format!("/forum/thread/{id}");
    let initial = name.chars().next().unwrap_or('F').to_string();
    let _ = slug;
    object([
        ("id", TemplateValue::from(id)),
        ("name", TemplateValue::from(name)),
        ("description", TemplateValue::from(description)),
        ("threads", TemplateValue::from(threads)),
        ("today", TemplateValue::from(today)),
        ("href", TemplateValue::from(href.as_str())),
        ("initial", TemplateValue::from(initial.as_str())),
    ])
}

fn forum_index_context() -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);
    let boards = vec![
        board(
            1,
            "announcements",
            "Announcements",
            "Official updates, patch notes, and maintenance windows.",
            18,
            2,
        ),
        board(
            2,
            "survival",
            "Survival",
            "Builds, bases, and day-to-day talk on the survival worlds.",
            412,
            24,
        ),
        board(
            3,
            "suggestions",
            "Suggestions",
            "Player ideas for ranks, plugins, and quality-of-life changes.",
            96,
            7,
        ),
        board(
            4,
            "off-topic",
            "Off-topic",
            "Community lounge for screenshots, memes, and side chats.",
            220,
            11,
        ),
    ];
    let count = boards.len();
    ctx.insert("boards", TemplateValue::Array(boards));
    ctx.insert("boards_count", TemplateValue::from(count));
    ctx.insert(
        "latest",
        TemplateValue::Array(vec![
            latest_item(1, "Patch notes 1.21.4", "Charlie Admin", "1h"),
            latest_item(2, "Season 4 spawn redesign", "SkyBuilder", "2m"),
            latest_item(3, "Rank perks feedback", "NovaCraft", "18m"),
            latest_item(4, "Looking for builders", "QuietLeaf", "3h"),
        ]),
    );
    ctx
}

fn latest_item(id: u32, title: &str, author: &str, when: &str) -> TemplateValue {
    let href = format!("/forum/thread/{id}");
    let initial = author.chars().next().unwrap_or('U').to_string();
    object([
        ("title", TemplateValue::from(title)),
        ("author", TemplateValue::from(author)),
        ("when", TemplateValue::from(when)),
        ("href", TemplateValue::from(href.as_str())),
        ("initial", TemplateValue::from(initial.as_str())),
    ])
}

fn forum_thread_context(id: u32) -> TemplateContext {
    let mut ctx = TemplateContext::new(ACTIVE_THEME);
    site_context(&mut ctx);

    let thread = match id {
        1 => Some(thread_data(
            "Patch notes 1.21.4",
            "Announcements",
            "Charlie Admin",
            "1h",
            "Economy tweaks, new crate cosmetics, and a short downtime window this Thursday.",
            vec![
                ("NovaCraft", "12m", "Love the crate cosmetics — when do keys drop?"),
                ("SkyBuilder", "8m", "Please post the exact downtime window in Discord too."),
            ],
        )),
        2 => Some(thread_data(
            "Season 4 spawn redesign",
            "Survival",
            "SkyBuilder",
            "2m",
            "Posted concept art and a block palette — looking for builder feedback before we freeze the layout.",
            vec![
                ("QuietLeaf", "1m", "Gothic roofs would look great on the north gate."),
                ("AshRidge", "45s", "I can help with the lighting pass this weekend."),
            ],
        )),
        3 => Some(thread_data(
            "Rank perks feedback",
            "Suggestions",
            "NovaCraft",
            "18m",
            "Is VIP still worth it after the flight nerf? Collecting honest takes before we rebalance.",
            vec![("RedstoneRex", "10m", "Flight was the only reason I bought VIP.")],
        )),
        4 => Some(thread_data(
            "Looking for builders",
            "Off-topic",
            "QuietLeaf",
            "3h",
            "Need two people comfortable with gothic roofs for the castle hub. Paid in store credit.",
            vec![],
        )),
        _ => None,
    };

    if let Some((thread_val, replies)) = thread {
        ctx.insert("thread", thread_val);
        ctx.insert("replies", replies);
    } else {
        ctx.insert("thread", TemplateValue::Null);
        ctx.insert("replies", TemplateValue::Array(vec![]));
    }

    ctx
}

fn thread_data(
    title: &str,
    board: &str,
    author: &str,
    when: &str,
    body: &str,
    replies: Vec<(&str, &str, &str)>,
) -> (TemplateValue, TemplateValue) {
    let reply_values: Vec<TemplateValue> = replies
        .into_iter()
        .map(|(a, w, b)| {
            let initial = a.chars().next().unwrap_or('U').to_string();
            object([
                ("author", TemplateValue::from(a)),
                ("when", TemplateValue::from(w)),
                ("body", TemplateValue::from(b)),
                ("initial", TemplateValue::from(initial.as_str())),
            ])
        })
        .collect();
    let replies_count = reply_values.len();
    let author_initial = author.chars().next().unwrap_or('U').to_string();
    let thread = object([
        ("title", TemplateValue::from(title)),
        ("board", TemplateValue::from(board)),
        ("author", TemplateValue::from(author)),
        ("author_initial", TemplateValue::from(author_initial.as_str())),
        ("when", TemplateValue::from(when)),
        ("body", TemplateValue::from(body)),
        ("replies_count", TemplateValue::from(replies_count)),
    ]);
    (thread, TemplateValue::Array(reply_values))
}

fn object<const N: usize>(entries: [(&str, TemplateValue); N]) -> TemplateValue {
    TemplateValue::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
