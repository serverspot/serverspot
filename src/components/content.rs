use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, PageHeader, SettingRow, SettingsControl, SettingsField,
};
use crate::components::ui::*;
use crate::router::Route;

pub const CONTENT_ACCENT: &str = "#f071a5";

pub const BLOG_CSS: Asset = asset!("/css-partials/blog.css");

const SECTIONS: &[&str] = &[
    "News",
    "Patch notes",
    "Community",
    "Guides",
    "Announcements",
];
const TEMPLATES: &[&str] = &["Landing", "Standalone", "Widget"];

#[component]
fn PressHeader(
    eyebrow: &'static str,
    title: &'static str,
    #[props(default)] subtitle: &'static str,
    #[props(default)] action: Option<Element>,
) -> Element {
    rsx! {
        document::Stylesheet { href: BLOG_CSS }
        p { class: "press-eyebrow", "{eyebrow}" }
        PageHeader { title, subtitle, action }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostStatus {
    Draft,
    InReview,
    Scheduled,
    Published,
}

impl PostStatus {
    const ALL: [PostStatus; 4] = [
        PostStatus::Draft,
        PostStatus::InReview,
        PostStatus::Scheduled,
        PostStatus::Published,
    ];

    fn label(self) -> &'static str {
        match self {
            PostStatus::Draft => "Draft",
            PostStatus::InReview => "In review",
            PostStatus::Scheduled => "Scheduled",
            PostStatus::Published => "Published",
        }
    }

    fn stamp(self) -> &'static str {
        match self {
            PostStatus::Draft => "DRAFT",
            PostStatus::InReview => "PROOF",
            PostStatus::Scheduled => "SET",
            PostStatus::Published => "LIVE",
        }
    }

    fn tone(self) -> &'static str {
        match self {
            PostStatus::Draft => "#87d1fe",
            PostStatus::InReview => "#f5c14a",
            PostStatus::Scheduled => "#5eead4",
            PostStatus::Published => "#f071a5",
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Post {
    id: u64,
    headline: String,
    dek: String,
    byline: String,
    section: String,
    status: PostStatus,
    scheduled_for: String,
    updated: String,
    words: u32,
    cover: String,
    featured: bool,
    body: String,
}

pub(crate) fn placeholder_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            headline: String::from("Season 4 launch recap"),
            dek: String::from(
                "New biomes, the rebuilt economy, and everything that shipped this week.",
            ),
            byline: String::from("Mira Chen"),
            section: String::from("News"),
            status: PostStatus::Published,
            scheduled_for: String::new(),
            updated: String::from("2h ago"),
            words: 640,
            cover: String::from(
                "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=1200&h=680&fit=crop",
            ),
            featured: true,
            body: String::from(
                "Season 4 is live. Here's a full recap of the new biomes, the rebuilt player economy, and the community events running this month…",
            ),
        },
        Post {
            id: 2,
            headline: String::from("Economy rebalance notes"),
            dek: String::from("Shop prices, vote rewards, and drop rates are all shifting."),
            byline: String::from("Kestrel"),
            section: String::from("Patch notes"),
            status: PostStatus::Draft,
            scheduled_for: String::new(),
            updated: String::from("Yesterday"),
            words: 210,
            cover: String::new(),
            featured: false,
            body: String::from("Draft: outline the economy changes before the editorial pass…"),
        },
        Post {
            id: 3,
            headline: String::from("Weekend crate event"),
            dek: String::from("Double drop rates and a new cosmetic crate, Friday to Sunday."),
            byline: String::from("Ivy Sato"),
            section: String::from("Announcements"),
            status: PostStatus::Scheduled,
            scheduled_for: String::from("2026-08-08"),
            updated: String::from("3 days ago"),
            words: 180,
            cover: String::new(),
            featured: false,
            body: String::from("This weekend only — double crate drop rates across every server…"),
        },
        Post {
            id: 4,
            headline: String::from("Builder spotlight: ClayMage"),
            dek: String::from("A tour of the survival world's most ambitious build yet."),
            byline: String::from("Priya N."),
            section: String::from("Community"),
            status: PostStatus::Published,
            scheduled_for: String::new(),
            updated: String::from("5d ago"),
            words: 420,
            cover: String::from(
                "https://images.unsplash.com/photo-1605806616949-1e87b487bc2a?w=1200&h=680&fit=crop",
            ),
            featured: false,
            body: String::from("This week's spotlight goes to ClayMage, whose skyline build has taken over the front page of the server map…"),
        },
        Post {
            id: 5,
            headline: String::from("Interview: the map team"),
            dek: String::from("How the new continent went from sketch to survival spawn."),
            byline: String::from("Otis R."),
            section: String::from("Community"),
            status: PostStatus::InReview,
            scheduled_for: String::new(),
            updated: String::from("1h ago"),
            words: 900,
            cover: String::new(),
            featured: false,
            body: String::from("Sat down with the map team to talk through the new continent, from first sketch to launch day…"),
        },
        Post {
            id: 6,
            headline: String::from("Server merge FAQ"),
            dek: String::from("What changes for your inventory, rank, and claims."),
            byline: String::from("Mira Chen"),
            section: String::from("Guides"),
            status: PostStatus::Scheduled,
            scheduled_for: String::from("2026-08-10"),
            updated: String::from("Today"),
            words: 260,
            cover: String::new(),
            featured: false,
            body: String::from("Everything you need to know before the two survival worlds merge next week…"),
        },
    ]
}

fn next_post_id(posts: &[Post]) -> u64 {
    posts.iter().map(|post| post.id).max().unwrap_or(0) + 1
}

fn read_minutes(words: u32) -> u32 {
    (words / 200).max(1)
}

#[component]
pub fn ContentOverview() -> Element {
    let navigator = use_navigator();
    let posts = use_context::<Signal<Vec<Post>>>();
    let posts_now = posts();

    let published = posts_now
        .iter()
        .filter(|post| post.status == PostStatus::Published)
        .count();
    let drafts = posts_now
        .iter()
        .filter(|post| matches!(post.status, PostStatus::Draft | PostStatus::InReview))
        .count();
    let scheduled_count = posts_now
        .iter()
        .filter(|post| post.status == PostStatus::Scheduled)
        .count();

    let lead = posts_now
        .iter()
        .find(|post| post.featured && post.status == PostStatus::Published)
        .or_else(|| {
            posts_now
                .iter()
                .find(|post| post.status == PostStatus::Published)
        })
        .cloned();

    let mut draft_pile: Vec<Post> = posts_now
        .iter()
        .filter(|post| matches!(post.status, PostStatus::Draft | PostStatus::InReview))
        .cloned()
        .collect();
    draft_pile.truncate(4);

    let mut schedule: Vec<Post> = posts_now
        .iter()
        .filter(|post| post.status == PostStatus::Scheduled)
        .cloned()
        .collect();
    schedule.sort_by(|a, b| a.scheduled_for.cmp(&b.scheduled_for));

    rsx! {
        document::Stylesheet { href: BLOG_CSS }

        div { class: "press-masthead",
            div { class: "min-w-0",
                p { class: "press-eyebrow", "The ServerSpot Press" }
                p { class: "press-edition",
                    "Editorial desk · {published} live · {drafts} in the pile · {scheduled_count} on the strip"
                }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::ContentPostNew {});
                },
                IconPlus {}
                "New post"
            }
        }

        if let Some(lead) = lead {
            {
                let lead_id = lead.id;
                rsx! {
                    section { class: "press-front",
                        p { class: "press-front-kicker", "Above the fold" }
                        button {
                            class: "press-lead",
                            r#type: "button",
                            onclick: move |_| {
                                navigator
                                    .push(Route::ContentPostEdit {
                                        id: lead_id,
                                    });
                            },
                            if lead.cover.trim().is_empty() {
                                div { class: "press-lead-media press-lead-media-empty", IconNews {} }
                            } else {
                                div { class: "press-lead-media",
                                    img { src: "{lead.cover}", alt: "" }
                                }
                            }
                            div { class: "press-lead-body",
                                span { class: "press-lead-tag", "{lead.section}" }
                                h1 { class: "press-lead-title", "{lead.headline}" }
                                p { class: "press-lead-dek", "{lead.dek}" }
                                p { class: "press-lead-byline", "By {lead.byline} · {lead.updated}" }
                            }
                        }
                    }
                }
            }
        } else {
            section { class: "press-front",
                p { class: "press-front-kicker", "Above the fold" }
                div { class: "press-lead press-lead-empty",
                    p { class: "press-lead-dek",
                        "No published lead story yet — publish a post to feature it here."
                    }
                }
            }
        }

        div { class: "motion-cascade press-desk-grid",
            section {
                div { class: "press-section-head",
                    h2 { class: "press-section-title", "Draft pile" }
                    button {
                        class: "press-section-link",
                        r#type: "button",
                        onclick: move |_| {
                            navigator.push(Route::ContentBlog {});
                        },
                        "All posts →"
                    }
                }
                if draft_pile.is_empty() {
                    p { class: "py-4 text-sm text-text-muted",
                        "The pile is empty — every story has moved on."
                    }
                } else {
                    div { class: "motion-cascade motion-cascade-tight press-draft-pile",
                        for post in draft_pile {
                            {
                                let post_id = post.id;
                                rsx! {
                                    button {
                                        key: "{post.id}",
                                        class: "press-draft-card",
                                        r#type: "button",
                                        onclick: move |_| {
                                            navigator
                                                .push(Route::ContentPostEdit {
                                                    id: post_id,
                                                });
                                        },
                                        span {
                                            class: "press-proof-stamp",
                                            style: "color: {post.status.tone()}; border-color: color-mix(in srgb, {post.status.tone()} 45%, transparent);",
                                            "{post.status.stamp()}"
                                        }
                                        div { class: "min-w-0",
                                            p { class: "truncate text-sm font-medium text-text", "{post.headline}" }
                                            p { class: "mt-0.5 truncate text-xs text-text-muted", "{post.byline} · {post.section}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            section {
                div { class: "press-section-head",
                    h2 { class: "press-section-title", "Schedule strip" }
                    span { class: "text-xs text-text-muted", "{scheduled_count} queued" }
                }
                if schedule.is_empty() {
                    p { class: "py-4 text-sm text-text-muted",
                        "Nothing queued — schedule a post to see it on the strip."
                    }
                } else {
                    div { class: "motion-cascade motion-cascade-tight press-schedule",
                        for post in schedule {
                            {
                                let post_id = post.id;
                                let display_date = format_display_date(&post.scheduled_for);
                                rsx! {
                                    button {
                                        key: "{post.id}",
                                        class: "press-schedule-item",
                                        r#type: "button",
                                        onclick: move |_| {
                                            navigator.push(Route::ContentPostEdit { id: post_id });
                                        },
                                        span { class: "press-schedule-date", "{display_date}" }
                                        span { class: "min-w-0 truncate text-sm text-text", "{post.headline}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ContentBlog() -> Element {
    let navigator = use_navigator();
    let posts = use_context::<Signal<Vec<Post>>>();
    let search = use_signal(String::new);
    let status_filter = use_signal(|| String::from("all"));

    let visible = use_memo(move || {
        let query = search().trim().to_ascii_lowercase();
        let filter = status_filter();
        let mut list: Vec<Post> = posts
            .read()
            .iter()
            .filter(|post| {
                let status_ok = match filter.as_str() {
                    "draft" => matches!(post.status, PostStatus::Draft | PostStatus::InReview),
                    "scheduled" => post.status == PostStatus::Scheduled,
                    "published" => post.status == PostStatus::Published,
                    _ => true,
                };
                let query_ok = query.is_empty()
                    || post.headline.to_ascii_lowercase().contains(&query)
                    || post.byline.to_ascii_lowercase().contains(&query);
                status_ok && query_ok
            })
            .cloned()
            .collect();
        list.sort_by(|a, b| b.id.cmp(&a.id));
        list
    });
    let total_posts = use_memo(move || posts.read().len());

    let status_options = [
        SelectOption::new("all", "All statuses"),
        SelectOption::new("draft", "Draft & review"),
        SelectOption::new("scheduled", "Scheduled"),
        SelectOption::new("published", "Published"),
    ];

    rsx! {
        PressHeader {
            eyebrow: "Copy desk",
            title: "Posts",
            subtitle: "Every headline in production — proofs, bylines, and press dates.",
            action: rsx! {
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ContentPostNew {});
                    },
                    IconPlus {}
                    "New post"
                }
            },
        }

        div { class: "mb-5 flex flex-col gap-3 sm:flex-row sm:items-center",
            SearchInput {
                value: search,
                placeholder: "Search headlines or bylines…",
                class: "sm:max-w-xs",
            }
            SignalSelect {
                value: status_filter,
                options: status_options.to_vec(),
                class: "sm:w-48",
            }
            p { class: "text-xs text-text-muted sm:ml-auto",
                "{visible().len()} of {total_posts()} stories"
            }
        }

        if visible().is_empty() {
            div { class: "motion-cascade motion-cascade-tight press-proof-list",
                p { class: "px-4 py-8 text-center text-sm text-text-muted",
                    "No stories match this filter."
                }
            }
        } else {
            div { class: "motion-cascade motion-cascade-tight press-proof-list",
                for post in visible() {
                    {
                        let post_id = post.id;
                        let meta = if post.status == PostStatus::Scheduled {
                            format!(
                                "{} · runs {}",
                                post.byline,
                                format_display_date(&post.scheduled_for),
                            )
                        } else {
                            format!("{} · {}", post.byline, post.updated)
                        };
                        rsx! {
                            button {
                                key: "{post.id}",
                                class: "press-proof",
                                r#type: "button",
                                onclick: move |_| {
                                    navigator
                                        .push(Route::ContentPostEdit {
                                            id: post_id,
                                        });
                                },
                                span {
                                    class: "press-proof-stamp",
                                    style: "color: {post.status.tone()}; border-color: color-mix(in srgb, {post.status.tone()} 45%, transparent);",
                                    "{post.status.stamp()}"
                                }
                                div { class: "press-proof-body",
                                    p { class: "truncate text-sm font-semibold text-text", "{post.headline}" }
                                    p { class: "mt-1 truncate text-xs text-text-muted", "{post.dek}" }
                                }
                                div { class: "press-proof-meta",
                                    span { class: "press-proof-section", "{post.section}" }
                                    span { "{meta}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ContentSiteSettings() -> Element {
    let public_path = use_signal(|| String::from("/news"));
    let masthead = use_signal(|| String::from("The ServerSpot Press"));
    let tagline = use_signal(|| String::from("News, patch notes, and stories from the team."));

    rsx! {
        FeatureSettingsChrome { subtitle: "Path, masthead branding, and how the newsroom behaves on your site.",
            DataPanel { title: "On your website",
                SettingsControl { label: "Public path",
                    SignalInput { value: public_path, class: "max-w-md".to_string() }
                }
                SettingsField {
                    label: "Full URL",
                    value: format!("www.example.com{}", public_path()),
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Masthead & branding",
                SettingsControl { label: "Masthead title",
                    SignalInput { value: masthead, class: "max-w-md".to_string() }
                }
                SettingsControl { label: "Tagline",
                    SignalInput { value: tagline, class: "max-w-md".to_string() }
                }
            }
            DataPanel { title: "Syndication",
                SettingRow {
                    title: "RSS feed",
                    description: "Publish an RSS feed of live posts at /news/feed.",
                    enabled: true,
                }
                SettingRow {
                    title: "Homepage feed widget",
                    description: "Show the three latest posts on the homepage.",
                    enabled: true,
                }
                SettingRow {
                    title: "Newsletter signup",
                    description: "Embed an email signup at the bottom of every post.",
                    enabled: false,
                }
                SettingRow {
                    title: "Comments",
                    description: "Allow signed-in players to comment on posts.",
                    enabled: false,
                }
            }
            DataPanel { title: "Editorial workflow",
                SettingRow {
                    title: "Require editorial approval",
                    description: "Drafts need a second staff sign-off before publishing.",
                    enabled: true,
                }
                SettingRow {
                    title: "Stale draft reminders",
                    description: "Notify authors when a draft sits idle for 30 days.",
                    enabled: false,
                }
            }
        }
    }
}

#[component]
pub fn ContentPostNew() -> Element {
    rsx! {
        PostEditor { post_id: None }
    }
}

#[component]
pub fn ContentPostEdit(id: u64) -> Element {
    rsx! {
        PostEditor { post_id: Some(id) }
    }
}

#[component]
fn PostEditor(post_id: Option<u64>) -> Element {
    let mut posts = use_context::<Signal<Vec<Post>>>();
    let navigator = use_navigator();
    let is_new = post_id.is_none();

    let (seed, missing) = use_hook(|| {
        let existing =
            post_id.and_then(|id| posts.peek().iter().find(|post| post.id == id).cloned());
        let missing = !is_new && existing.is_none();
        let seed = existing.unwrap_or(Post {
            id: 0,
            headline: String::new(),
            dek: String::new(),
            byline: String::new(),
            section: SECTIONS[0].to_string(),
            status: PostStatus::Draft,
            scheduled_for: String::new(),
            updated: String::from("Just now"),
            words: 0,
            cover: String::new(),
            featured: false,
            body: String::new(),
        });
        (seed, missing)
    });

    let headline = use_signal(|| seed.headline.clone());
    let dek = use_signal(|| seed.dek.clone());
    let byline = use_signal(|| seed.byline.clone());
    let section = use_signal(|| seed.section.clone());
    let mut status = use_signal(|| seed.status);
    let scheduled_for = use_signal(|| seed.scheduled_for.clone());
    let cover = use_signal(|| seed.cover.clone());
    let featured = use_signal(|| seed.featured);
    let body = use_signal(|| seed.body.clone());

    let headline_now = headline();
    let dek_now = dek();
    let byline_now = byline();
    let section_now = section();
    let status_now = status();
    let scheduled_now = scheduled_for();
    let cover_now = cover();
    let featured_now = featured();
    let body_now = body();
    let can_save = !headline_now.trim().is_empty();
    let words = body_now.split_whitespace().count().max(seed.words as usize) as u32;

    let preview_headline = if headline_now.trim().is_empty() {
        String::from("Untitled story")
    } else {
        headline_now.trim().to_string()
    };
    let preview_dek = if dek_now.trim().is_empty() {
        String::from("No deck written yet.")
    } else {
        dek_now.trim().to_string()
    };
    let preview_byline = if byline_now.trim().is_empty() {
        String::from("Unassigned")
    } else {
        byline_now.trim().to_string()
    };

    if missing {
        return rsx! {
            document::Stylesheet { href: BLOG_CSS }
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ContentBlog {});
                    },
                    "← Posts"
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", "Story not found" }
            p { class: "mt-2 text-sm text-text-muted",
                "This post may have been pulled from the desk."
            }
        };
    }

    let section_options: Vec<SelectOption> = SECTIONS
        .iter()
        .map(|value| SelectOption::new(*value, *value))
        .collect();

    let save = move |_| {
        let headline_value = headline().trim().to_string();
        if headline_value.is_empty() {
            return;
        }
        let dek_value = dek().trim().to_string();
        let byline_value = byline().trim().to_string();
        let section_value = {
            let value = section().trim().to_string();
            if value.is_empty() {
                SECTIONS[0].to_string()
            } else {
                value
            }
        };
        let status_value = status();
        let scheduled_value = scheduled_for();
        let cover_value = cover();
        let featured_value = featured();
        let body_value = body();
        let word_count = body_value.split_whitespace().count() as u32;

        posts.with_mut(|list| {
            if let Some(id) = post_id {
                if let Some(post) = list.iter_mut().find(|post| post.id == id) {
                    post.headline = headline_value;
                    post.dek = dek_value;
                    post.byline = byline_value;
                    post.section = section_value;
                    post.status = status_value;
                    post.scheduled_for = scheduled_value;
                    post.cover = cover_value;
                    post.featured = featured_value;
                    post.body = body_value;
                    post.words = word_count;
                    post.updated = String::from("Just now");
                }
            } else {
                let id = next_post_id(list);
                list.push(Post {
                    id,
                    headline: headline_value,
                    dek: dek_value,
                    byline: byline_value,
                    section: section_value,
                    status: status_value,
                    scheduled_for: scheduled_value,
                    updated: String::from("Just now"),
                    words: word_count,
                    cover: cover_value,
                    featured: featured_value,
                    body: body_value,
                });
            }
        });

        navigator.push(Route::ContentBlog {});
    };

    rsx! {
        document::Stylesheet { href: BLOG_CSS }
        div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    navigator.push(Route::ContentBlog {});
                },
                "← Posts"
            }
            div { class: "flex flex-wrap items-center gap-2",
                if let Some(id) = post_id {
                    Button {
                        variant: ButtonVariant::Danger,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            posts.with_mut(|list| list.retain(|post| post.id != id));
                            navigator.push(Route::ContentBlog {});
                        },
                        "Delete"
                    }
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ContentBlog {});
                    },
                    "Cancel"
                }
                Button {
                    size: ButtonSize::Sm,
                    disabled: !can_save,
                    onclick: save,
                    if is_new {
                        "Create post"
                    } else {
                        "Save changes"
                    }
                }
            }
        }

        p { class: "press-eyebrow", "Copy desk" }
        div { class: "mb-8",
            h1 { class: "text-3xl font-semibold tracking-tight",
                if is_new {
                    "New post"
                } else {
                    "Edit post"
                }
            }
            p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                "Headline, deck, byline, and the press date players will see."
            }
        }

        div { class: "press-editor-layout",
            div { class: "motion-cascade press-editor-main space-y-8",
                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Headline & deck" }
                    p { class: "press-editor-lede",
                        "The headline runs big; the deck is the one-line summary underneath."
                    }
                    div { class: "mt-4 space-y-4",
                        FieldLabel { label: "Headline",
                            SignalInput {
                                value: headline,
                                placeholder: "Season 4 launch recap",
                            }
                        }
                        FieldLabel { label: "Deck",
                            SignalTextarea {
                                value: dek,
                                placeholder: "One line that sums up the story…",
                                class: "min-h-[4.5rem]",
                            }
                        }
                    }
                }

                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Byline & desk" }
                    p { class: "press-editor-lede", "Who wrote it, and which desk it runs under." }
                    div { class: "mt-4 grid gap-4 sm:grid-cols-2",
                        FieldLabel { label: "Byline",
                            SignalInput { value: byline, placeholder: "Mira Chen" }
                        }
                        FieldLabel { label: "Section",
                            SignalSelect { value: section, options: section_options }
                        }
                    }
                }

                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Cover art" }
                    p { class: "press-editor-lede",
                        "Wide image shown on the lead story and post header."
                    }
                    div { class: "mt-4",
                        CoverUploadField { value: cover }
                    }
                }

                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Status & press date" }
                    p { class: "press-editor-lede",
                        "Move the story through the desk — draft, review, set, or live."
                    }
                    div { class: "mt-4 space-y-3",
                        div { class: "press-segment",
                            for option in PostStatus::ALL {
                                button {
                                    key: "{option.label()}",
                                    r#type: "button",
                                    class: if status_now == option { "press-segment-btn is-active" } else { "press-segment-btn" },
                                    style: if status_now == option { format!(
                                        "background: color-mix(in srgb, {} 22%, transparent); color: {};",
                                        option.tone(),
                                        option.tone(),
                                    ) } else { String::new() },
                                    onclick: move |_| status.set(option),
                                    "{option.label()}"
                                }
                            }
                        }
                        if status_now == PostStatus::Scheduled {
                            FieldLabel { label: "Runs on",
                                SignalDatePicker {
                                    value: scheduled_for,
                                    placeholder: "Choose a press date",
                                }
                            }
                        }
                    }
                }

                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Copy" }
                    p { class: "press-editor-lede", "The full story body." }
                    div { class: "mt-4",
                        SignalTextarea {
                            value: body,
                            placeholder: "Write the story…",
                            class: "min-h-[14rem]",
                        }
                    }
                }

                section { class: "press-editor-section",
                    h2 { class: "press-editor-heading", "Placement" }
                    p { class: "press-editor-lede",
                        "Feature this story as the newsroom lead once it's published."
                    }
                    div { class: "mt-4",
                        ToggleRow {
                            title: "Feature as lead story",
                            description: "Shown large at the top of the Newsroom overview.",
                            checked: featured,
                        }
                    }
                }
            }

            aside { class: "press-editor-aside",
                div { class: "press-editor-preview",
                    p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                        "Proof preview"
                    }
                    div { class: "mt-4 flex items-start gap-3",
                        if cover_now.trim().is_empty() {
                            div { class: "press-proof-thumb", IconNews {} }
                        } else {
                            img {
                                src: "{cover_now}",
                                alt: "",
                                class: "press-proof-thumb-img",
                            }
                        }
                        div { class: "min-w-0 flex-1",
                            div { class: "flex flex-wrap items-center gap-2",
                                span {
                                    class: "press-proof-stamp",
                                    style: "color: {status_now.tone()}; border-color: color-mix(in srgb, {status_now.tone()} 45%, transparent);",
                                    "{status_now.stamp()}"
                                }
                                span { class: "text-xs text-text-muted", "{section_now}" }
                            }
                            p { class: "mt-1.5 text-base font-semibold tracking-tight text-text",
                                "{preview_headline}"
                            }
                            p { class: "mt-1 text-sm text-text-muted", "{preview_dek}" }
                        }
                    }
                    ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                        li { "By {preview_byline}" }
                        li { "{read_minutes(words)} min read · {words} words" }
                        if status_now == PostStatus::Scheduled && !scheduled_now.trim().is_empty() {
                            li { "Runs {format_display_date(&scheduled_now)}" }
                        }
                        if featured_now {
                            li { "Featured as lead story" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldLabel(
    label: &'static str,
    #[props(default)] hint: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div {
            label { class: "mb-1.5 block text-xs font-medium text-text-muted", "{label}" }
            {children}
            if let Some(hint) = hint {
                p { class: "mt-1.5 text-xs text-text-muted", "{hint}" }
            }
        }
    }
}

#[component]
fn ToggleRow(title: &'static str, description: &'static str, mut checked: Signal<bool>) -> Element {
    let on = checked();
    rsx! {
        div { class: "flex flex-col gap-3 border-b border-border-subtle py-4 last:border-0 sm:flex-row sm:items-start sm:justify-between sm:gap-4",
            div { class: "min-w-0",
                p { class: "text-sm font-medium text-text", "{title}" }
                p { class: "mt-1 text-sm text-text-muted", "{description}" }
            }
            Button {
                class: "self-start",
                variant: if on { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                size: ButtonSize::Sm,
                onclick: move |_| {
                    let next = !*checked.peek();
                    checked.set(next);
                },
                if on {
                    "On"
                } else {
                    "Off"
                }
            }
        }
    }
}

#[component]
fn CoverUploadField(mut value: Signal<String>) -> Element {
    let mut file_name = use_signal(String::new);
    let current = value();
    let name_now = file_name();

    rsx! {
        div { class: "space-y-2",
            div { class: "flex items-start justify-between gap-3",
                div {
                    p { class: "text-xs font-medium text-text-muted", "Cover image" }
                    p { class: "text-xs text-text-muted/80",
                        "Wide image for the lead story and post header"
                    }
                }
                if !current.trim().is_empty() {
                    Button {
                        variant: ButtonVariant::Danger,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            value.set(String::new());
                            file_name.set(String::new());
                        },
                        "Remove"
                    }
                }
            }
            label { class: "press-cover-upload",
                if current.trim().is_empty() {
                    span { class: "pointer-events-none px-3 text-center text-xs leading-relaxed text-text-muted",
                        "Click to upload cover art"
                    }
                } else {
                    img {
                        src: "{current}",
                        alt: "",
                        class: "pointer-events-none h-full w-full object-cover",
                    }
                }
                input {
                    r#type: "file",
                    accept: "image/png,image/jpeg,image/webp,image/gif",
                    class: "absolute inset-0 cursor-pointer opacity-0",
                    onchange: move |evt| {
                        async move {
                            let Some(file) = evt.files().into_iter().next() else {
                                return;
                            };
                            let mime = file
                                .content_type()
                                .unwrap_or_else(|| String::from("image/png"));
                            let Ok(bytes) = file.read_bytes().await else {
                                return;
                            };
                            use base64::Engine as _;
                            let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
                            let data_url = format!("data:{mime};base64,{encoded}");
                            file_name.set(file.name());
                            value.set(data_url);
                        }
                    },
                }
            }
            if !name_now.is_empty() {
                p { class: "truncate text-xs text-text-secondary", "{name_now}" }
            }
        }
    }
}
