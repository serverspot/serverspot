use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, SettingRow, SettingsControl, SettingsField,
};
use crate::components::ui::*;
use crate::router::Route;
use crate::user::CurrentUser;

const FORUM_ACCENT: &str = "#5b9dff";

#[derive(Clone, Copy, PartialEq)]
struct ForumStats {
    threads: u32,
    posts_today: u32,
    members: u32,
    public_path: &'static str,
}

fn placeholder_forum_stats() -> ForumStats {
    ForumStats {
        threads: 1_204,
        posts_today: 52,
        members: 3_481,
        public_path: "/forum",
    }
}

fn format_count(value: u32) -> String {
    if value < 1_000 {
        value.to_string()
    } else {
        format!("{},{:03}", value / 1_000, value % 1_000)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoardVisibility {
    Public,
    Staff,
    Private,
}

impl BoardVisibility {
    const fn label(self) -> &'static str {
        match self {
            Self::Public => "Public",
            Self::Staff => "Staff",
            Self::Private => "Private",
        }
    }

    const fn tone(self) -> &'static str {
        match self {
            Self::Public => "#3ecf8e",
            Self::Staff => "#5b9dff",
            Self::Private => "#e5484d",
        }
    }

    const fn hint(self) -> &'static str {
        match self {
            Self::Public => "Anyone on the site can browse this board.",
            Self::Staff => "Only staff roles can open this board.",
            Self::Private => "Invite-only — hidden from the public list.",
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct BoardLink {
    label: String,
    url: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Board {
    id: u64,
    name: String,
    description: String,
    image: String,
    banner: String,
    links: Vec<BoardLink>,
    threads: u32,
    posts_today: u32,
    visibility: BoardVisibility,
    accent: String,
}

pub(crate) fn placeholder_boards() -> Vec<Board> {
    vec![
        Board {
            id: 1,
            name: String::from("Announcements"),
            description: String::from(
                "Official updates, patch notes, and maintenance windows.",
            ),
            image: String::from(
                "https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=160&h=160&fit=crop",
            ),
            banner: String::from(
                "https://images.unsplash.com/photo-1550745165-9bc0b252726f?w=1200&h=320&fit=crop",
            ),
            links: vec![
                BoardLink {
                    label: String::from("Patch notes"),
                    url: String::from("https://example.com/patches"),
                },
                BoardLink {
                    label: String::from("Status page"),
                    url: String::from("https://status.example.com"),
                },
            ],
            threads: 18,
            posts_today: 2,
            visibility: BoardVisibility::Staff,
            accent: String::from("#69bdf2"),
        },
        Board {
            id: 2,
            name: String::from("Survival"),
            description: String::from(
                "Builds, bases, and day-to-day talk on the survival worlds.",
            ),
            image: String::from(
                "https://images.unsplash.com/photo-1605806616949-1e87b487bc2a?w=160&h=160&fit=crop",
            ),
            banner: String::from(
                "https://images.unsplash.com/photo-1511512578047-dfb367046420?w=1200&h=320&fit=crop",
            ),
            links: vec![BoardLink {
                label: String::from("World map"),
                url: String::from("https://example.com/map"),
            }],
            threads: 412,
            posts_today: 24,
            visibility: BoardVisibility::Public,
            accent: String::from("#3ecf8e"),
        },
        Board {
            id: 3,
            name: String::from("Suggestions"),
            description: String::from(
                "Player ideas for ranks, plugins, and quality-of-life changes.",
            ),
            image: String::new(),
            banner: String::new(),
            links: Vec::new(),
            threads: 96,
            posts_today: 11,
            visibility: BoardVisibility::Public,
            accent: String::from("#f5c14a"),
        },
        Board {
            id: 4,
            name: String::from("Staff room"),
            description: String::from("Internal discussion for moderators and senior staff."),
            image: String::new(),
            banner: String::from(
                "https://images.unsplash.com/photo-1558494949-ef010cbdcc31?w=1200&h=320&fit=crop",
            ),
            links: vec![BoardLink {
                label: String::from("Mod handbook"),
                url: String::from("https://example.com/staff"),
            }],
            threads: 44,
            posts_today: 6,
            visibility: BoardVisibility::Private,
            accent: String::from("#f071a5"),
        },
        Board {
            id: 5,
            name: String::from("Off-topic"),
            description: String::from(
                "Community lounge for screenshots, memes, and side chats.",
            ),
            image: String::from(
                "https://images.unsplash.com/photo-1511512578047-dfb367046420?w=160&h=160&fit=crop",
            ),
            banner: String::new(),
            links: Vec::new(),
            threads: 220,
            posts_today: 9,
            visibility: BoardVisibility::Public,
            accent: String::from("#5eead4"),
        },
    ]
}

fn next_board_id(boards: &[Board]) -> u64 {
    boards.iter().map(|board| board.id).max().unwrap_or(0) + 1
}

#[derive(Clone, PartialEq)]
struct BoardLinkDraft {
    id: u64,
    label: String,
    url: String,
}

const MAX_BOARD_LINKS: usize = 3;

fn default_board_accent() -> String {
    String::from(DEFAULT_COLOR_PRESETS[0])
}

fn collect_board_links(drafts: &[BoardLinkDraft]) -> Vec<BoardLink> {
    drafts
        .iter()
        .filter(|link| !link.label.trim().is_empty() && !link.url.trim().is_empty())
        .take(MAX_BOARD_LINKS)
        .map(|link| BoardLink {
            label: link.label.trim().to_string(),
            url: link.url.trim().to_string(),
        })
        .collect()
}

fn links_to_drafts(links: &[BoardLink]) -> (Vec<BoardLinkDraft>, u64) {
    let drafts: Vec<BoardLinkDraft> = links
        .iter()
        .enumerate()
        .map(|(index, link)| BoardLinkDraft {
            id: (index as u64) + 1,
            label: link.label.clone(),
            url: link.url.clone(),
        })
        .collect();
    let next_id = drafts.last().map(|link| link.id + 1).unwrap_or(1);
    (drafts, next_id)
}

#[derive(Clone, PartialEq)]
pub(crate) struct Thread {
    id: u64,
    title: String,
    preview: String,
    author: String,
    author_email: String,
    category: String,
    replies: u32,
    when: String,
    pinned: bool,
    locked: bool,
}

pub(crate) fn placeholder_threads() -> Vec<Thread> {
    vec![
        Thread {
            id: 1,
            title: String::from("Season 4 spawn redesign"),
            preview: String::from(
                "Posted concept art and a block palette — looking for builder feedback before we freeze the layout.",
            ),
            author: String::from("SkyBuilder"),
            author_email: String::from("skybuilder@players.local"),
            category: String::from("Survival"),
            replies: 24,
            when: String::from("2m"),
            pinned: false,
            locked: false,
        },
        Thread {
            id: 2,
            title: String::from("Patch notes 1.21.4"),
            preview: String::from(
                "Economy tweaks, new crate cosmetics, and a short downtime window this Thursday.",
            ),
            author: String::from("Charlie Admin"),
            author_email: String::from("admin@serverspot.app"),
            category: String::from("Announcements"),
            replies: 41,
            when: String::from("1h"),
            pinned: true,
            locked: true,
        },
        Thread {
            id: 3,
            title: String::from("Rank perks feedback"),
            preview: String::from(
                "Is VIP still worth it after the flight nerf? Collecting honest takes before we rebalance.",
            ),
            author: String::from("NovaCraft"),
            author_email: String::from("nova@players.local"),
            category: String::from("Suggestions"),
            replies: 11,
            when: String::from("18m"),
            pinned: false,
            locked: false,
        },
        Thread {
            id: 4,
            title: String::from("Looking for builders"),
            preview: String::from(
                "Need two people comfortable with gothic roofs for the castle hub. Paid in store credit.",
            ),
            author: String::from("QuietLeaf"),
            author_email: String::from("quiet@players.local"),
            category: String::from("Off-topic"),
            replies: 7,
            when: String::from("3h"),
            pinned: false,
            locked: false,
        },
        Thread {
            id: 5,
            title: String::from("Screenshot dump — nether hub"),
            preview: String::from(
                "Three angles of the new portal atrium plus the lighting settings we used.",
            ),
            author: String::from("AshRidge"),
            author_email: String::from("ash@players.local"),
            category: String::from("Survival"),
            replies: 15,
            when: String::from("5h"),
            pinned: false,
            locked: false,
        },
    ]
}

fn next_thread_id(threads: &[Thread]) -> u64 {
    threads.iter().map(|thread| thread.id).max().unwrap_or(0) + 1
}

fn default_thread_board(boards: &[Board]) -> String {
    boards
        .first()
        .map(|board| board.name.clone())
        .unwrap_or_else(|| String::from("General"))
}

fn thread_badge_line(pinned: bool, locked: bool) -> String {
    match (pinned, locked) {
        (true, true) => String::from("Pinned · Locked"),
        (true, false) => String::from("Pinned"),
        (false, true) => String::from("Locked"),
        (false, false) => String::new(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ThreadStatusOption {
    Pinned,
    Locked,
    Open,
}

impl ThreadStatusOption {
    const ALL: [Self; 3] = [Self::Pinned, Self::Locked, Self::Open];

    const fn label(self) -> &'static str {
        match self {
            Self::Pinned => "Pinned",
            Self::Locked => "Locked",
            Self::Open => "Open",
        }
    }

    const fn key(self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::Locked => "locked",
            Self::Open => "open",
        }
    }

    fn from_key(key: &str) -> Option<Self> {
        match key {
            "pinned" => Some(Self::Pinned),
            "locked" => Some(Self::Locked),
            "open" => Some(Self::Open),
            _ => None,
        }
    }

    fn matches(self, thread: &Thread) -> bool {
        match self {
            Self::Pinned => thread.pinned,
            Self::Locked => thread.locked,
            Self::Open => !thread.locked,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ThreadSort {
    #[default]
    Recent,
    Replies,
    Title,
}

impl ThreadSort {
    const ALL: [Self; 3] = [Self::Recent, Self::Replies, Self::Title];

    const fn label(self) -> &'static str {
        match self {
            Self::Recent => "Recent",
            Self::Replies => "Most replies",
            Self::Title => "Title A–Z",
        }
    }
}

fn filter_threads(
    threads: &[Thread],
    query: &str,
    boards: &[String],
    statuses: &[String],
    sort: ThreadSort,
) -> Vec<Thread> {
    let query = query.trim().to_ascii_lowercase();
    let status_opts: Vec<ThreadStatusOption> = statuses
        .iter()
        .filter_map(|key| ThreadStatusOption::from_key(key))
        .collect();

    let mut filtered: Vec<Thread> = threads
        .iter()
        .filter(|thread| {
            status_opts.is_empty() || status_opts.iter().any(|status| status.matches(thread))
        })
        .filter(|thread| boards.is_empty() || boards.iter().any(|board| board == &thread.category))
        .filter(|thread| {
            if query.is_empty() {
                return true;
            }
            thread.title.to_ascii_lowercase().contains(&query)
                || thread.preview.to_ascii_lowercase().contains(&query)
                || thread.author.to_ascii_lowercase().contains(&query)
                || thread.category.to_ascii_lowercase().contains(&query)
        })
        .cloned()
        .collect();

    filtered.sort_by(|a, b| {
        b.pinned.cmp(&a.pinned).then_with(|| match sort {
            ThreadSort::Recent => b.id.cmp(&a.id),
            ThreadSort::Replies => b.replies.cmp(&a.replies).then_with(|| {
                a.title
                    .to_ascii_lowercase()
                    .cmp(&b.title.to_ascii_lowercase())
            }),
            ThreadSort::Title => a
                .title
                .to_ascii_lowercase()
                .cmp(&b.title.to_ascii_lowercase()),
        })
    });

    filtered
}

fn toggle_selection(mut selected: Signal<Vec<String>>, value: &str) {
    selected.with_mut(|list| {
        if let Some(index) = list.iter().position(|item| item == value) {
            list.remove(index);
        } else {
            list.push(value.to_string());
        }
    });
}

fn selection_summary(selected: &[String], empty: &str, singular: &str) -> String {
    match selected.len() {
        0 => empty.to_string(),
        1 => selected[0].clone(),
        n => format!("{n} {singular}"),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportSeverity {
    High,
    Medium,
    Low,
}

impl ReportSeverity {
    const fn label(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
        }
    }

    const fn tone(self) -> &'static str {
        match self {
            Self::High => "#f87171",
            Self::Medium => "#f0a35e",
            Self::Low => "#858899",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Report {
    title: &'static str,
    detail: &'static str,
    reporter: &'static str,
    board: &'static str,
    when: &'static str,
    severity: ReportSeverity,
}

impl Report {
    fn meta(self) -> String {
        format!("{} · {} · {}", self.board, self.reporter, self.when)
    }
}

const REPORTS: &[Report] = &[
    Report {
        title: "Spam reply chain in Suggestions",
        detail: "Three near-identical store promo links posted under the rank perks thread.",
        reporter: "NovaCraft",
        board: "Suggestions",
        when: "12m",
        severity: ReportSeverity::High,
    },
    Report {
        title: "Toxic thread title",
        detail: "Personal attack aimed at a staff member in Off-topic.",
        reporter: "QuietLeaf",
        board: "Off-topic",
        when: "41m",
        severity: ReportSeverity::Medium,
    },
    Report {
        title: "Duplicate announcement",
        detail: "Looks like an accidental re-post of yesterday’s patch notes.",
        reporter: "Staff",
        board: "Announcements",
        when: "2h",
        severity: ReportSeverity::Low,
    },
];

#[component]
pub fn ForumOverview() -> Element {
    let threads = use_context::<Signal<Vec<Thread>>>();
    let stats = placeholder_forum_stats();
    let navigator = use_navigator();

    let posts_today = format_count(stats.posts_today);
    let members = format_count(stats.members);
    let thread_count = format_count(stats.threads);
    let report_count = REPORTS.len();
    let featured = threads.read().first().cloned();
    let featured_id = featured.as_ref().map(|thread| thread.id);

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", "Community desk" }
                    h1 { class: "forum-desk-title", "Forum" }
                    p { class: "forum-desk-sub",
                        "Boards, threads, and the moderation queue in one place."
                    }
                }
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "View on website" }
            }

            div { class: "motion-cascade ops-meter",
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Posts today" }
                    p { class: "ops-meter-value is-accent", "{posts_today}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Threads" }
                    p { class: "ops-meter-value", "{thread_count}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Members" }
                    p { class: "ops-meter-value", "{members}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Open reports" }
                    p { class: "ops-meter-value", "{report_count}" }
                }
            }

            if let Some(featured) = featured {
                section { class: "mb-9",
                    div { class: "ops-section-head",
                        h2 { class: "ops-section-title", "Hot right now" }
                        span { class: "ops-section-sub", "Most active conversation" }
                    }
                    button {
                        r#type: "button",
                        class: "forum-featured",
                        onclick: move |_| {
                            if let Some(id) = featured_id {
                                navigator.push(Route::ForumThread { id });
                            }
                        },
                        h2 { class: "forum-featured-title", "{featured.title}" }
                        p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-text-secondary",
                            "{featured.preview}"
                        }
                        div { class: "mt-5 flex items-center gap-3",
                            Avatar {
                                email: featured.author_email.clone(),
                                size: 36,
                                alt: featured.author.clone(),
                                class: "shrink-0",
                            }
                            div { class: "min-w-0 text-left",
                                p { class: "text-sm font-medium text-text", "{featured.author}" }
                                p { class: "text-xs text-text-muted",
                                    "{featured.category} · {featured.replies} replies · {featured.when}"
                                }
                            }
                        }
                    }
                }
            }

            section { class: "mb-9",
                div { class: "ops-section-head",
                    h2 { class: "ops-section-title", "Needs a look" }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumModeration {});
                        },
                        "Open queue →"
                    }
                }
                div { class: "motion-cascade motion-cascade-tight forum-queue",
                    for report in REPORTS.iter().copied() {
                        OverviewReportRow { report }
                    }
                }
            }

            section { class: "motion-cascade ops-pulse",
                button {
                    r#type: "button",
                    class: "ops-pulse-link",
                    onclick: move |_| {
                        navigator.push(Route::ForumBoards {});
                    },
                    div { class: "min-w-0",
                        p { class: "ops-pulse-title", "Boards" }
                        p { class: "ops-pulse-meta", "Structure and visibility" }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
                button {
                    r#type: "button",
                    class: "ops-pulse-link",
                    onclick: move |_| {
                        navigator.push(Route::ForumThreads {});
                    },
                    div { class: "min-w-0",
                        p { class: "ops-pulse-title", "Threads" }
                        p { class: "ops-pulse-meta", "Search and moderate conversations" }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
                button {
                    r#type: "button",
                    class: "ops-pulse-link",
                    onclick: move |_| {
                        navigator.push(Route::ForumAutoModeration {});
                    },
                    div { class: "min-w-0",
                        p { class: "ops-pulse-title", "Auto Mod" }
                        p { class: "ops-pulse-meta", "Filters, mutes, and bot identity" }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
            }
        }
    }
}

#[component]
fn OverviewReportRow(report: Report) -> Element {
    let meta = report.meta();
    let severity = report.severity;

    rsx! {
        article { class: "forum-report", style: "--ops-accent: {severity.tone()};",
            span { class: "forum-report-rail" }
            div { class: "forum-report-body",
                div { class: "flex flex-wrap items-center gap-2",
                    h3 { class: "text-sm font-medium tracking-tight", "{report.title}" }
                    ToneChip { label: severity.label(), tone: severity.tone() }
                }
                p { class: "mt-1 text-xs text-text-muted", "{meta}" }
            }
        }
    }
}

#[component]
pub fn ForumBoards() -> Element {
    let boards = use_context::<Signal<Vec<Board>>>();
    let navigator = use_navigator();
    let list = boards();

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", "Community structure" }
                    h1 { class: "forum-desk-title", "Boards" }
                    p { class: "forum-desk-sub",
                        "Spaces players browse on the forum. Click a board to edit it."
                    }
                }
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ForumBoardNew {});
                    },
                    IconPlus {}
                    "New board"
                }
            }

            div { class: "forum-vis-legend",
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Public.tone()};",
                    "Public"
                }
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Staff.tone()};",
                    "Staff"
                }
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Private.tone()};",
                    "Private"
                }
            }

            div { class: "motion-cascade motion-cascade-tight forum-board-list",
                for board in list.into_iter() {
                    {
                        let board_id = board.id;
                        let counts = format!(
                            "{} threads · {} today",
                            format_count(board.threads),
                            board.posts_today,
                        );
                        let has_banner = !board.banner.is_empty();
                        rsx! {
                            button {
                                r#type: "button",
                                class: "forum-board-item",
                                onclick: move |_| {
                                    navigator
                                        .push(Route::ForumBoardEdit {
                                            id: board_id,
                                        });
                                },
                                if has_banner {
                                    div { class: "forum-board-item-banner",
                                        img {
                                            src: "{board.banner}",
                                            alt: "",
                                            class: "h-full w-full object-cover",
                                        }
                                    }
                                }
                                div { class: "forum-board-item-body",
                                    if board.image.is_empty() {
                                        div {
                                            class: "forum-board-thumb",
                                            style: "background: {board.accent};",
                                            IconForum {}
                                        }
                                    } else {
                                        img {
                                            src: "{board.image}",
                                            alt: "{board.name}",
                                            class: "forum-board-thumb-img",
                                        }
                                    }
                                    div { class: "min-w-0 flex-1 text-left",
                                        div { class: "flex flex-wrap items-center gap-2",
                                            p { class: "text-base font-semibold tracking-tight text-text", "{board.name}" }
                                            span {
                                                class: "forum-vis-chip",
                                                style: "--chip-tone: {board.visibility.tone()};",
                                                "{board.visibility.label()}"
                                            }
                                        }
                                        p { class: "mt-1 text-sm leading-relaxed text-text-muted", "{board.description}" }
                                    }
                                    div { class: "shrink-0 text-right",
                                        p { class: "text-sm tabular-nums tracking-tight text-text-secondary",
                                            "{counts}"
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
}

#[component]
pub fn ForumBoardNew() -> Element {
    rsx! {
        BoardEditor { board_id: None }
    }
}

#[component]
pub fn ForumBoardEdit(id: u64) -> Element {
    rsx! {
        BoardEditor { board_id: Some(id) }
    }
}

#[component]
fn BoardEditor(board_id: Option<u64>) -> Element {
    let mut boards = use_context::<Signal<Vec<Board>>>();
    let navigator = use_navigator();
    let is_new = board_id.is_none();

    let existing =
        board_id.and_then(|id| boards.read().iter().find(|board| board.id == id).cloned());
    let missing = !is_new && existing.is_none();
    let seed = existing.clone().unwrap_or(Board {
        id: 0,
        name: String::new(),
        description: String::new(),
        image: String::new(),
        banner: String::new(),
        links: Vec::new(),
        threads: 0,
        posts_today: 0,
        visibility: BoardVisibility::Public,
        accent: default_board_accent(),
    });
    let thread_count = seed.threads;
    let posts_today = seed.posts_today;
    let (link_drafts, next_id) = links_to_drafts(&seed.links);

    let name = use_signal(|| seed.name.clone());
    let description = use_signal(|| seed.description.clone());
    let image = use_signal(|| seed.image.clone());
    let banner = use_signal(|| seed.banner.clone());
    let mut links = use_signal(|| link_drafts);
    let mut next_link_id = use_signal(|| next_id);
    let mut visibility = use_signal(|| seed.visibility);
    let accent = use_signal(|| seed.accent.clone());

    let name_now = name();
    let description_now = description();
    let image_now = image();
    let banner_now = banner();
    let links_now = links();
    let visibility_now = visibility();
    let accent_now = accent();
    let can_save = !name_now.trim().is_empty();
    let filled_links = links_now
        .iter()
        .filter(|link| !link.label.trim().is_empty() || !link.url.trim().is_empty())
        .count();
    let preview_name = if name_now.trim().is_empty() {
        String::from("Untitled board")
    } else {
        name_now.trim().to_string()
    };
    let preview_description = if description_now.trim().is_empty() {
        String::from("No description yet")
    } else {
        description_now.trim().to_string()
    };
    let activity_label = format!(
        "{} threads · {} today",
        format_count(thread_count),
        posts_today
    );

    if missing {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ForumBoards {});
                    },
                    "← Boards"
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", "Board not found" }
            p { class: "mt-2 text-sm text-text-muted", "This board may have been deleted." }
        };
    }

    let save = move |_| {
        let name_value = name().trim().to_string();
        if name_value.is_empty() {
            return;
        }
        let collected = collect_board_links(&links());
        let visibility_value = visibility();
        let accent_value = {
            let value = accent().trim().to_string();
            if value.is_empty() {
                default_board_accent()
            } else {
                value
            }
        };
        let description_value = description().trim().to_string();
        let image_value = image();
        let banner_value = banner();

        boards.with_mut(|list| {
            if let Some(id) = board_id {
                if let Some(board) = list.iter_mut().find(|board| board.id == id) {
                    board.name = name_value;
                    board.description = description_value;
                    board.image = image_value;
                    board.banner = banner_value;
                    board.links = collected;
                    board.visibility = visibility_value;
                    board.accent = accent_value;
                }
            } else {
                let id = next_board_id(list);
                list.push(Board {
                    id,
                    name: name_value,
                    description: description_value,
                    image: image_value,
                    banner: banner_value,
                    links: collected,
                    threads: 0,
                    posts_today: 0,
                    visibility: visibility_value,
                    accent: accent_value,
                });
            }
        });

        navigator.push(Route::ForumBoards {});
    };

    rsx! {
        div { class: "motion-cascade forum-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ForumBoards {});
                    },
                    "← Boards"
                }
                div { class: "flex flex-wrap items-center gap-2",
                    if let Some(id) = board_id {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                boards.with_mut(|list| list.retain(|board| board.id != id));
                                navigator.push(Route::ForumBoards {});
                            },
                            "Delete"
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumBoards {});
                        },
                        "Cancel"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: save,
                        if is_new {
                            "Create board"
                        } else {
                            "Save changes"
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", "Community structure" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        "New board"
                    } else {
                        "Edit board"
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    "Name, media, visibility, and links players see on this board."
                }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Basics" }
                        p { class: "forum-editor-lede",
                            "Name and short description shown in the board list."
                        }
                        div { class: "mt-4 space-y-4",
                            FormField { label: "Name",
                                SignalInput { value: name, placeholder: "Survival" }
                            }
                            FormField { label: "Description",
                                SignalTextarea {
                                    value: description,
                                    placeholder: "Builds, bases, and day-to-day talk…",
                                    class: "min-h-[5.5rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Links" }
                        p { class: "forum-editor-lede",
                            "Optional shortcuts — Discord, maps, docs. Up to {MAX_BOARD_LINKS}."
                        }
                        div { class: "mt-4 space-y-3",
                            div { class: "flex items-center justify-between gap-3",
                                p { class: "text-xs text-text-muted",
                                    "{links_now.len()} / {MAX_BOARD_LINKS}"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Sm,
                                    disabled: links_now.len() >= MAX_BOARD_LINKS,
                                    onclick: move |_| {
                                        if links().len() >= MAX_BOARD_LINKS {
                                            return;
                                        }
                                        let id = next_link_id();
                                        next_link_id.set(id + 1);
                                        links
                                            .write()
                                            .push(BoardLinkDraft {
                                                id,
                                                label: String::new(),
                                                url: String::new(),
                                            });
                                    },
                                    IconPlus {}
                                    "Add link"
                                }
                            }
                            if links_now.is_empty() {
                                p { class: "rounded-squircle-sm border border-dashed border-border-subtle px-3 py-4 text-sm text-text-muted",
                                    "No links yet. Add up to {MAX_BOARD_LINKS}."
                                }
                            } else {
                                div { class: "space-y-3",
                                    for (index, link) in links_now.iter().enumerate() {
                                        BoardLinkEditor {
                                            key: "{link.id}",
                                            links,
                                            index,
                                            link_id: link.id,
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Appearance" }
                        p { class: "forum-editor-lede",
                            "Accent colour for the board thumbnail when no image is set."
                        }
                        div { class: "mt-4",
                            FormField { label: "Colour",
                                ColorPicker { value: accent }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Visibility" }
                        p { class: "forum-editor-lede",
                            "Who can browse this board on the public forum."
                        }
                        div { class: "mt-4",
                            div {
                                class: "inline-flex w-full rounded-squircle-sm border border-border-subtle p-1",
                                style: "background: var(--color-surface);",
                                SegmentChoice {
                                    label: "Public",
                                    tone: BoardVisibility::Public.tone(),
                                    active: visibility_now == BoardVisibility::Public,
                                    onclick: move |_| visibility.set(BoardVisibility::Public),
                                }
                                SegmentChoice {
                                    label: "Staff",
                                    tone: BoardVisibility::Staff.tone(),
                                    active: visibility_now == BoardVisibility::Staff,
                                    onclick: move |_| visibility.set(BoardVisibility::Staff),
                                }
                                SegmentChoice {
                                    label: "Private",
                                    tone: BoardVisibility::Private.tone(),
                                    active: visibility_now == BoardVisibility::Private,
                                    onclick: move |_| visibility.set(BoardVisibility::Private),
                                }
                            }
                            p { class: "mt-2 text-xs text-text-muted", "{visibility_now.hint()}" }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Media" }
                        p { class: "forum-editor-lede",
                            "Square icon and optional wide banner for the board page."
                        }
                        div { class: "mt-4 grid gap-5 sm:grid-cols-2",
                            MediaUploadField {
                                label: "Image",
                                hint: "Square icon next to the board name",
                                value: image,
                                tall: false,
                            }
                            MediaUploadField {
                                label: "Banner",
                                hint: "Wide header shown above the board",
                                value: banner,
                                tall: true,
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            "Preview"
                        }
                        div { class: "mt-4 flex items-start gap-3",
                            if image_now.trim().is_empty() {
                                div {
                                    class: "forum-board-thumb",
                                    style: "background: {accent_now};",
                                    IconForum {}
                                }
                            } else {
                                img {
                                    src: "{image_now}",
                                    alt: "",
                                    class: "forum-board-thumb-img",
                                }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "flex flex-wrap items-center gap-2",
                                    p { class: "text-base font-semibold tracking-tight text-text",
                                        "{preview_name}"
                                    }
                                    span {
                                        class: "text-[11px] font-medium",
                                        style: "color: {visibility_now.tone()};",
                                        "{visibility_now.label()}"
                                    }
                                }
                                p { class: "mt-1 text-sm text-text-muted", "{preview_description}" }
                            }
                        }
                        ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                            li { "{visibility_now.label()} board" }
                            if filled_links > 0 {
                                li { "{filled_links} links" }
                            } else {
                                li { "No links yet" }
                            }
                            if !is_new {
                                li { "{activity_label}" }
                            }
                            if !banner_now.trim().is_empty() {
                                li { "Banner set" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BoardLinkEditor(mut links: Signal<Vec<BoardLinkDraft>>, index: usize, link_id: u64) -> Element {
    let link = links().get(index).cloned().unwrap_or(BoardLinkDraft {
        id: link_id,
        label: String::new(),
        url: String::new(),
    });

    rsx! {
        div {
            class: "rounded-squircle-sm border border-border-subtle p-3",
            style: "background: var(--color-surface);",
            div { class: "mb-2 flex items-center justify-between gap-2",
                p { class: "text-xs font-medium text-text-secondary", "Link {index + 1}" }
                Button {
                    variant: ButtonVariant::Danger,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        links.write().retain(|item| item.id != link_id);
                    },
                    "Remove"
                }
            }
            div { class: "grid gap-2 sm:grid-cols-2",
                input {
                    r#type: "text",
                    class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                    placeholder: "Label",
                    value: "{link.label}",
                    oninput: move |evt: FormEvent| {
                        let next = evt.value();
                        links
                            .with_mut(|list| {
                                if let Some(item) = list.get_mut(index) {
                                    item.label = next;
                                }
                            });
                    },
                }
                input {
                    r#type: "url",
                    class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                    placeholder: "https://…",
                    value: "{link.url}",
                    oninput: move |evt: FormEvent| {
                        let next = evt.value();
                        links
                            .with_mut(|list| {
                                if let Some(item) = list.get_mut(index) {
                                    item.url = next;
                                }
                            });
                    },
                }
            }
        }
    }
}

#[component]
fn MediaUploadField(
    label: &'static str,
    hint: &'static str,
    mut value: Signal<String>,
    tall: bool,
) -> Element {
    let mut file_name = use_signal(String::new);
    let current = value();
    let name_now = file_name();

    let frame = if tall {
        "relative flex h-32 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm border border-dashed border-border-subtle transition-colors hover:border-border"
    } else {
        "relative flex h-32 w-32 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm border border-dashed border-border-subtle transition-colors hover:border-border"
    };

    rsx! {
        div { class: "space-y-2",
            div { class: "flex items-start justify-between gap-3",
                div {
                    p { class: "text-xs font-medium text-text-muted", "{label}" }
                    p { class: "text-xs text-text-muted/80", "{hint}" }
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
            label { class: "{frame}", style: "background: var(--color-surface);",
                if current.trim().is_empty() {
                    span { class: "pointer-events-none px-3 text-center text-xs leading-relaxed text-text-muted",
                        "Click to upload image"
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

#[component]
fn FormField(
    label: &'static str,
    #[props(default)] hint: Option<&'static str>,
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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SegmentIdle {
    #[default]
    Soft,
    Outline,
}

#[component]
fn SegmentChoice(
    label: &'static str,
    tone: &'static str,
    active: bool,
    #[props(default)] idle: SegmentIdle,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = if active {
        "flex-1 rounded-squircle-sm px-3 py-2 text-center text-xs font-semibold transition-colors"
    } else {
        "flex-1 rounded-squircle-sm px-3 py-2 text-center text-xs font-medium transition-colors"
    };
    let style = if active {
        format!("background: color-mix(in srgb, {tone} 22%, transparent); color: {tone};")
    } else {
        match idle {
            SegmentIdle::Soft => {
                String::from("background: transparent; color: var(--color-text-muted); opacity: 0.85;")
            }
            SegmentIdle::Outline => String::from(
                "background: transparent; color: var(--color-text-muted); outline: 1px solid var(--color-border-subtle);",
            ),
        }
    };

    rsx! {
        button {
            class,
            r#type: "button",
            style: "{style}",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

#[component]
fn BoardChoiceChip(
    name: String,
    tone: String,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let style = if selected {
        format!(
            "background: color-mix(in srgb, {tone} 22%, transparent); color: {tone}; outline: 1px solid color-mix(in srgb, {tone} 45%, transparent);"
        )
    } else {
        String::from(
            "background: transparent; color: var(--color-text-muted); outline: 1px solid var(--color-border-subtle);",
        )
    };

    rsx! {
        button {
            r#type: "button",
            class: "rounded-squircle-sm px-3 py-2 text-xs font-medium transition-colors",
            style: "{style}",
            onclick: move |evt| onclick.call(evt),
            "{name}"
        }
    }
}

#[component]
fn ToneChip(label: &'static str, tone: &'static str) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center rounded-squircle-sm px-2.5 py-1 text-xs font-medium",
            style: "background: color-mix(in srgb, {tone} 16%, transparent); color: {tone};",
            "{label}"
        }
    }
}

#[component]
pub fn ForumThreads() -> Element {
    let boards = use_context::<Signal<Vec<Board>>>();
    let threads = use_context::<Signal<Vec<Thread>>>();
    let navigator = use_navigator();

    let mut query = use_signal(String::new);
    let mut statuses = use_signal(Vec::<String>::new);
    let mut board_filters = use_signal(Vec::<String>::new);
    let mut sort = use_signal(ThreadSort::default);
    let mut status_menu = use_signal(|| false);
    let mut board_menu = use_signal(|| false);
    let mut sort_menu = use_signal(|| false);

    let filtered = use_memo(move || {
        let threads = threads.read();
        let query = query.read();
        let boards = board_filters.read();
        let statuses = statuses.read();
        filter_threads(&threads, &query, &boards, &statuses, sort())
    });
    let result_label = use_memo(move || {
        let len = filtered().len();
        if len == 1 {
            String::from("1 thread")
        } else {
            format!("{len} threads")
        }
    });
    let status_summary = use_memo(move || {
        let labels: Vec<String> = statuses()
            .iter()
            .filter_map(|key| ThreadStatusOption::from_key(key).map(|opt| opt.label().to_string()))
            .collect();
        selection_summary(&labels, "Any status", "statuses")
    });
    let board_summary =
        use_memo(move || selection_summary(&board_filters(), "All boards", "boards"));
    let filters_active = !query().trim().is_empty()
        || !statuses().is_empty()
        || !board_filters().is_empty()
        || sort() != ThreadSort::Recent;

    let boards_now = boards();
    let statuses_now = statuses();
    let board_filters_now = board_filters();
    let sort_now = sort();

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", "Conversations" }
                    h1 { class: "forum-desk-title", "Threads" }
                    p { class: "forum-desk-sub", "Search, filter, and open any conversation." }
                }
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ForumThreadNew {});
                    },
                    IconPlus {}
                    "New thread"
                }
            }

            div { class: "forum-toolbar mb-5",
                SearchInput {
                    value: query,
                    placeholder: "Search title, author, board…",
                    class: "forum-toolbar-search",
                }

                div { class: "forum-toolbar-filters",
                    FilterMultiSelect {
                        label: "Status",
                        summary: status_summary(),
                        open: status_menu,
                        on_toggle_menu: move |_| {
                            let next = !status_menu();
                            status_menu.set(next);
                            if next {
                                board_menu.set(false);
                                sort_menu.set(false);
                            }
                        },
                        body: rsx! {
                            for option in ThreadStatusOption::ALL {
                                FilterCheckOption {
                                    label: option.label().to_string(),
                                    checked: statuses_now.iter().any(|item| item == option.key()),
                                    onclick: move |_| toggle_selection(statuses, option.key()),
                                }
                            }
                        },
                    }

                    FilterMultiSelect {
                        label: "Boards",
                        summary: board_summary(),
                        open: board_menu,
                        on_toggle_menu: move |_| {
                            let next = !board_menu();
                            board_menu.set(next);
                            if next {
                                status_menu.set(false);
                                sort_menu.set(false);
                            }
                        },
                        body: rsx! {
                            if boards_now.is_empty() {
                                p { class: "px-3 py-2 text-xs text-text-muted", "No boards yet." }
                            } else {
                                for option in boards_now.iter() {
                                    FilterCheckOption {
                                        key: "{option.id}",
                                        label: option.name.clone(),
                                        checked: board_filters_now.iter().any(|item| item == &option.name),
                                        onclick: {
                                            let name = option.name.clone();
                                            move |_| toggle_selection(board_filters, &name)
                                        },
                                    }
                                }
                            }
                        },
                    }

                    FilterMultiSelect {
                        label: "Sort",
                        summary: sort_now.label().to_string(),
                        open: sort_menu,
                        on_toggle_menu: move |_| {
                            let next = !sort_menu();
                            sort_menu.set(next);
                            if next {
                                status_menu.set(false);
                                board_menu.set(false);
                            }
                        },
                        body: rsx! {
                            for option in ThreadSort::ALL {
                                FilterCheckOption {
                                    label: option.label().to_string(),
                                    checked: sort_now == option,
                                    onclick: move |_| {
                                        sort.set(option);
                                        sort_menu.set(false);
                                    },
                                }
                            }
                        },
                    }

                    if filters_active {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                query.set(String::new());
                                statuses.set(Vec::new());
                                board_filters.set(Vec::new());
                                sort.set(ThreadSort::Recent);
                                status_menu.set(false);
                                board_menu.set(false);
                                sort_menu.set(false);
                            },
                            "Clear filters"
                        }
                    }
                }

                p { class: "text-xs text-text-secondary", "{result_label()}" }
            }

            if filtered().is_empty() {
                p { class: "rounded-squircle-lg border border-dashed border-border-subtle px-4 py-10 text-center text-sm text-text-muted",
                    "No threads match these filters."
                }
            } else {
                div { class: "motion-cascade motion-cascade-tight forum-thread-panel",
                    for thread in filtered() {
                        ThreadCard { key: "{thread.id}", thread }
                    }
                }
            }
        }
    }
}

#[component]
fn FilterMultiSelect(
    label: &'static str,
    #[props(into)] summary: String,
    open: Signal<bool>,
    on_toggle_menu: EventHandler<MouseEvent>,
    body: Element,
) -> Element {
    let is_open = open();

    rsx! {
        div { class: "relative space-y-1.5",
            label { class: "block text-xs font-medium text-text-muted", "{label}" }
            button {
                r#type: "button",
                class: "ui-input ui-squircle flex h-10 w-full items-center justify-between gap-2 px-3 text-left text-sm outline-none",
                onclick: move |evt| on_toggle_menu.call(evt),
                span { class: "min-w-0 truncate text-text", "{summary}" }
                span { class: "shrink-0 text-xs text-text-muted",
                    if is_open {
                        "▴"
                    } else {
                        "▾"
                    }
                }
            }
            if is_open {
                div { class: "absolute left-0 right-0 z-20 mt-1 max-h-56 overflow-y-auto rounded-squircle-sm border border-border-subtle bg-bg-elevated p-1 shadow-lg",
                    {body}
                }
            }
        }
    }
}

#[component]
fn FilterCheckOption(
    #[props(into)] label: String,
    checked: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "flex w-full items-center gap-2.5 rounded-squircle-sm px-2.5 py-2 text-left text-sm transition-colors hover:bg-surface-2",
            onclick: move |evt| onclick.call(evt),
            span {
                class: if checked { "flex h-4 w-4 shrink-0 items-center justify-center rounded-[0.35rem] text-[10px] font-bold text-text-on-accent" } else { "flex h-4 w-4 shrink-0 items-center justify-center rounded-[0.35rem] border border-border-subtle text-[10px]" },
                style: if checked { format!("background: {FORUM_ACCENT};") } else { String::from("background: var(--color-surface);") },
                if checked {
                    "✓"
                } else {
                    ""
                }
            }
            span { class: "min-w-0 truncate text-text", "{label}" }
        }
    }
}

#[component]
pub fn ForumThreadNew() -> Element {
    let boards = use_context::<Signal<Vec<Board>>>();
    let mut threads = use_context::<Signal<Vec<Thread>>>();
    let current_user = use_context::<Signal<CurrentUser>>();
    let navigator = use_navigator();

    let title = use_signal(String::new);
    let body = use_signal(String::new);
    let mut board = use_signal(|| default_thread_board(&boards()));
    let mut pinned = use_signal(|| false);
    let mut locked = use_signal(|| false);

    let title_now = title();
    let body_now = body();
    let board_now = board();
    let pinned_now = pinned();
    let locked_now = locked();
    let boards_now = boards();
    let user = current_user();
    let can_save = !title_now.trim().is_empty() && !board_now.trim().is_empty();

    let preview_title = if title_now.trim().is_empty() {
        String::from("Thread title")
    } else {
        title_now.trim().to_string()
    };
    let preview_body = if body_now.trim().is_empty() {
        String::from("The opening post players will read first.")
    } else {
        body_now.trim().to_string()
    };
    let preview_title_class = if title_now.trim().is_empty() {
        "text-base font-semibold tracking-tight text-text-muted sm:text-lg"
    } else {
        "text-base font-semibold tracking-tight sm:text-lg"
    };
    let preview_body_class = if body_now.trim().is_empty() {
        "mt-1.5 max-w-3xl text-sm leading-relaxed text-text-muted/70"
    } else {
        "mt-1.5 max-w-3xl text-sm leading-relaxed text-text-muted"
    };
    let preview_board = if board_now.trim().is_empty() {
        String::from("Board")
    } else {
        board_now.clone()
    };
    let badge_line = thread_badge_line(pinned_now, locked_now);
    let preview_meta = format!("{} · 0 replies · just now", user.name);

    let create = move |_| {
        if title().trim().is_empty() || board().trim().is_empty() {
            return;
        }
        let user = current_user();
        let id = next_thread_id(&threads());
        threads.write().insert(
            0,
            Thread {
                id,
                title: title().trim().to_string(),
                preview: body().trim().to_string(),
                author: user.name,
                author_email: user.email,
                category: board().trim().to_string(),
                replies: 0,
                when: String::from("just now"),
                pinned: pinned(),
                locked: locked(),
            },
        );
        navigator.push(Route::ForumThread { id });
    };

    rsx! {
        div { class: "motion-cascade forum-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ForumThreads {});
                    },
                    "← Threads"
                }
                div { class: "flex flex-wrap items-center gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumThreads {});
                        },
                        "Cancel"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: create,
                        "Create thread"
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", "Start a conversation" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", "New thread" }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    "Title, opening post, board, and whether it should be pinned or locked."
                }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Basics" }
                        p { class: "forum-editor-lede", "Title and opening post players see first." }
                        div { class: "mt-4 space-y-4",
                            FormField { label: "Title",
                                SignalInput {
                                    value: title,
                                    placeholder: "Season 4 spawn redesign",
                                }
                            }
                            FormField { label: "Opening post",
                                SignalTextarea {
                                    value: body,
                                    placeholder: "Share the context, ask for feedback, or drop the announcement…",
                                    class: "min-h-[9rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Board" }
                        p { class: "forum-editor-lede", "Where this thread lives on the forum." }
                        div { class: "mt-4",
                            if boards_now.is_empty() {
                                SignalInput { value: board, placeholder: "General" }
                            } else {
                                div { class: "flex flex-wrap gap-2",
                                    for option in boards_now.into_iter() {
                                        BoardChoiceChip {
                                            key: "{option.id}",
                                            name: option.name.clone(),
                                            tone: option.accent.clone(),
                                            selected: board_now == option.name,
                                            onclick: {
                                                let name = option.name.clone();
                                                move |_| board.set(name.clone())
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Options" }
                        p { class: "forum-editor-lede",
                            "Pinned threads stay at the top. Locked threads stay visible but closed to replies."
                        }
                        div { class: "mt-4 inline-flex w-full gap-2",
                            SegmentChoice {
                                label: "Pinned",
                                tone: "#69bdf2",
                                active: pinned_now,
                                idle: SegmentIdle::Outline,
                                onclick: move |_| pinned.set(!pinned()),
                            }
                            SegmentChoice {
                                label: "Locked",
                                tone: "#f0a35e",
                                active: locked_now,
                                idle: SegmentIdle::Outline,
                                onclick: move |_| locked.set(!locked()),
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            "Preview"
                        }
                        article { class: "mt-4 forum-thread forum-thread-static pointer-events-none border-b-0",
                            Avatar {
                                email: user.email.clone(),
                                size: 40,
                                alt: user.name.clone(),
                                class: "mt-0.5 shrink-0",
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "flex flex-wrap items-center gap-x-2 gap-y-1",
                                    h3 { class: "{preview_title_class}", "{preview_title}" }
                                    span { class: "rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-[11px] font-medium text-text-secondary",
                                        "{preview_board}"
                                    }
                                    if !badge_line.is_empty() {
                                        span {
                                            class: "text-[11px] font-medium",
                                            style: "color: {FORUM_ACCENT};",
                                            "{badge_line}"
                                        }
                                    }
                                }
                                p { class: "{preview_body_class}", "{preview_body}" }
                                p { class: "mt-3 text-xs text-text-secondary", "{preview_meta}" }
                            }
                        }
                        ul { class: "mt-5 space-y-1.5 text-xs text-text-secondary",
                            li { "Posted as {user.name}" }
                            li {
                                if pinned_now {
                                    "Pinned to top"
                                } else {
                                    "Not pinned"
                                }
                            }
                            li {
                                if locked_now {
                                    "Locked · no replies yet"
                                } else {
                                    "Open for replies"
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
fn ThreadCard(thread: Thread) -> Element {
    let navigator = use_navigator();
    let thread_id = thread.id;
    let replies = format!("{} replies", thread.replies);
    let badge_line = thread_badge_line(thread.pinned, thread.locked);

    rsx! {
        button {
            r#type: "button",
            class: "forum-thread",
            onclick: move |_| {
                navigator
                    .push(Route::ForumThread {
                        id: thread_id,
                    });
            },
            Avatar {
                email: thread.author_email.clone(),
                size: 40,
                alt: thread.author.clone(),
                class: "mt-0.5 shrink-0",
            }
            div { class: "min-w-0 flex-1",
                div { class: "flex flex-wrap items-center gap-x-2 gap-y-1",
                    h3 { class: "text-base font-semibold tracking-tight sm:text-lg",
                        "{thread.title}"
                    }
                    span { class: "rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-[11px] font-medium text-text-secondary",
                        "{thread.category}"
                    }
                    if !badge_line.is_empty() {
                        span {
                            class: "text-[11px] font-medium",
                            style: "color: {FORUM_ACCENT};",
                            "{badge_line}"
                        }
                    }
                }
                p { class: "mt-1.5 max-w-3xl text-sm leading-relaxed text-text-muted",
                    "{thread.preview}"
                }
                p { class: "mt-3 text-xs text-text-secondary",
                    "{thread.author} · {replies} · {thread.when}"
                }
            }
        }
    }
}

#[component]
pub fn ForumThread(id: u64) -> Element {
    let mut threads = use_context::<Signal<Vec<Thread>>>();
    let boards = use_context::<Signal<Vec<Board>>>();
    let navigator = use_navigator();
    let mut move_open = use_signal(|| false);

    let thread = threads
        .read()
        .iter()
        .find(|thread| thread.id == id)
        .cloned();

    let Some(thread) = thread else {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ForumThreads {});
                    },
                    "← Threads"
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", "Thread not found" }
            p { class: "mt-2 text-sm text-text-muted", "This conversation may have been removed." }
        };
    };

    let meta = {
        let mut parts = vec![thread.category.clone()];
        let badges = thread_badge_line(thread.pinned, thread.locked);
        if !badges.is_empty() {
            parts.push(badges);
        }
        parts.push(format!("{} replies", thread.replies));
        parts.push(thread.when.clone());
        parts.join(" · ")
    };
    let reply_samples = placeholder_replies(&thread);
    let pin_label = if thread.pinned { "Unpin" } else { "Pin" };
    let lock_label = if thread.locked { "Unlock" } else { "Lock" };
    let boards_now = boards();
    let thread_board = thread.category.clone();
    let move_menu_open = move_open();

    rsx! {
        div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    navigator.push(Route::ForumThreads {});
                },
                "← Threads"
            }
            div { class: "relative flex flex-wrap items-center justify-end gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        threads
                            .with_mut(|list| {
                                if let Some(item) = list.iter_mut().find(|item| item.id == id) {
                                    item.pinned = !item.pinned;
                                }
                            });
                    },
                    "{pin_label}"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        threads
                            .with_mut(|list| {
                                if let Some(item) = list.iter_mut().find(|item| item.id == id) {
                                    item.locked = !item.locked;
                                }
                            });
                    },
                    "{lock_label}"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| move_open.set(!move_open()),
                    "Move"
                }
                Button {
                    variant: ButtonVariant::Danger,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        threads
                            .with_mut(|list| {
                                list.retain(|item| item.id != id);
                            });
                        navigator.push(Route::ForumThreads {});
                    },
                    "Delete"
                }
                if move_menu_open && !boards_now.is_empty() {
                    div { class: "absolute right-0 top-full z-20 mt-2 w-56 rounded-squircle-sm border border-border-subtle bg-bg-elevated p-1 shadow-lg",
                        p { class: "px-2.5 py-1.5 text-[11px] font-medium uppercase tracking-wide text-text-muted",
                            "Move to board"
                        }
                        for option in boards_now.into_iter() {
                            button {
                                r#type: "button",
                                class: if thread_board == option.name { "flex w-full items-center gap-2 rounded-squircle-sm px-2.5 py-2 text-left text-sm font-medium text-text" } else { "flex w-full items-center gap-2 rounded-squircle-sm px-2.5 py-2 text-left text-sm text-text-muted hover:bg-surface-2 hover:text-text" },
                                onclick: {
                                    let name = option.name.clone();
                                    move |_| {
                                        threads.with_mut(|list| {
                                            if let Some(item) = list.iter_mut().find(|item| item.id == id) {
                                                item.category = name.clone();
                                            }
                                        });
                                        move_open.set(false);
                                    }
                                },
                                span {
                                    class: "h-2.5 w-2.5 shrink-0 rounded-full",
                                    style: "background: {option.accent};",
                                }
                                "{option.name}"
                            }
                        }
                    }
                }
            }
        }

        header { class: "mb-8 max-w-3xl",
            h1 { class: "text-3xl font-semibold tracking-tight sm:text-4xl", "{thread.title}" }
            p { class: "mt-2 text-sm text-text-muted", "{meta}" }
        }

        article { class: "forum-post",
            Avatar {
                email: thread.author_email.clone(),
                size: 44,
                alt: thread.author.clone(),
            }
            div { class: "min-w-0 flex-1",
                div { class: "flex flex-wrap items-baseline gap-x-2 gap-y-1",
                    p { class: "text-sm font-semibold text-text", "{thread.author}" }
                    span { class: "text-xs text-text-muted", "Original post · {thread.when}" }
                }
                p { class: "mt-3 text-[0.95rem] leading-relaxed text-text-secondary whitespace-pre-wrap",
                    "{thread.preview}"
                }
            }
        }

        section { class: "mt-2 mb-4 flex items-baseline justify-between gap-3 border-t border-border-subtle pt-6",
            p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                "Replies"
            }
            span { class: "text-xs text-text-secondary", "{thread.replies}" }
        }

        if thread.locked {
            p { class: "mb-4 text-sm text-text-muted",
                "This thread is locked — new replies are disabled."
            }
        }

        if reply_samples.is_empty() {
            p { class: "py-8 text-sm text-text-muted", "No replies yet." }
        } else {
            for reply in reply_samples.into_iter() {
                article { class: "forum-post",
                    Avatar {
                        email: reply.email.clone(),
                        size: 36,
                        alt: reply.author.clone(),
                    }
                    div { class: "min-w-0 flex-1",
                        div { class: "flex flex-wrap items-baseline gap-x-2 gap-y-1",
                            p { class: "text-sm font-semibold text-text", "{reply.author}" }
                            span { class: "text-xs text-text-muted", "{reply.when}" }
                        }
                        p { class: "mt-2 text-sm leading-relaxed text-text-secondary",
                            "{reply.body}"
                        }
                    }
                }
            }
        }

        if !thread.locked {
            section { class: "mt-8 max-w-2xl border-t border-border-subtle pt-6",
                p { class: "mb-3 text-xs font-medium text-text-muted", "Reply" }
                textarea {
                    class: "ui-input ui-squircle min-h-28 w-full resize-y px-4 py-3 text-sm outline-none",
                    placeholder: "Write a reply…",
                }
                div { class: "mt-3",
                    Button { "Post reply" }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct ThreadReply {
    author: String,
    email: String,
    body: String,
    when: String,
}

fn placeholder_replies(thread: &Thread) -> Vec<ThreadReply> {
    if thread.replies == 0 {
        return Vec::new();
    }

    let mut replies = vec![
        ThreadReply {
            author: String::from("NovaCraft"),
            email: String::from("nova@players.local"),
            body: String::from(
                "Looks solid — I’d push the main path a bit wider before we lock the palette.",
            ),
            when: String::from("1h"),
        },
        ThreadReply {
            author: String::from("QuietLeaf"),
            email: String::from("quiet@players.local"),
            body: String::from(
                "Agreed on the lighting. Happy to mock a darker atrium variant this weekend.",
            ),
            when: String::from("42m"),
        },
    ];

    if thread.replies > 20 {
        replies.push(ThreadReply {
            author: String::from("AshRidge"),
            email: String::from("ash@players.local"),
            body: String::from("Dropping screenshots in Discord too so staff can compare angles."),
            when: String::from("18m"),
        });
    }

    replies
}

#[component]
pub fn ForumModeration() -> Element {
    let current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user();
    let acting_as = format!("Acting as {} · {}", user.name, user.role);
    let report_count = REPORTS.len();

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", "Staff tools" }
                    h1 { class: "forum-desk-title", "Moderation" }
                    p { class: "forum-desk-sub", "{acting_as}" }
                }
            }

            div { class: "motion-cascade ops-meter",
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Open reports" }
                    p { class: "ops-meter-value is-accent", "{report_count}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Locked" }
                    p { class: "ops-meter-value", "2" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Auto-hidden" }
                    p { class: "ops-meter-value", "1" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", "Avg. review" }
                    p { class: "ops-meter-value", "18m" }
                }
            }

            section { class: "mb-4",
                div { class: "ops-section-head",
                    h2 { class: "ops-section-title", "Report queue" }
                    span { class: "ops-section-sub", "Highest severity first" }
                }
                div { class: "motion-cascade motion-cascade-tight forum-queue",
                    for report in REPORTS.iter().copied() {
                        ReportCard { report }
                    }
                }
            }
        }
    }
}

#[component]
fn ReportCard(report: Report) -> Element {
    let meta = report.meta();
    let severity = report.severity;

    rsx! {
        article { class: "forum-report", style: "--ops-accent: {severity.tone()};",
            span { class: "forum-report-rail" }
            div { class: "forum-report-body",
                div { class: "flex flex-wrap items-center gap-2",
                    h3 { class: "text-sm font-medium tracking-tight sm:text-base",
                        "{report.title}"
                    }
                    ToneChip { label: severity.label(), tone: severity.tone() }
                }
                p { class: "mt-1.5 text-sm leading-relaxed text-text-muted", "{report.detail}" }
                p { class: "mt-2 text-xs text-text-secondary", "{meta}" }
                div { class: "mt-4 flex flex-wrap gap-2",
                    Button { variant: ButtonVariant::Danger, size: ButtonSize::Sm, "Hide post" }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "Warn user"
                    }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Dismiss" }
                }
            }
        }
    }
}

#[component]
pub fn ForumAutoModeration() -> Element {
    let bot_name = use_signal(|| String::from("ServerSpot AutoMod"));
    let bot_tag = use_signal(|| String::from("BOT"));
    let bot_avatar = use_signal(String::new);
    let bot_accent = use_signal(|| String::from("#f0a35e"));
    let warn_message = use_signal(|| {
        String::from(
            "Hey {author} — your post was flagged by Auto Mod for breaking {rule}. Please edit or remove it.",
        )
    });
    let mute_message = use_signal(|| {
        String::from(
            "You’ve been muted for {duration} minutes after repeated Auto Mod hits. Staff can review this.",
        )
    });
    let blocked_words = use_signal(|| String::from("buy ranks, free nitro, .gg/"));
    let max_links = use_signal(|| String::from("2"));
    let mute_minutes = use_signal(|| String::from("30"));
    let new_account_hours = use_signal(|| String::from("24"));

    let preview_name = bot_name();
    let preview_tag = bot_tag();
    let preview_avatar = bot_avatar();
    let preview_accent = bot_accent();
    let preview_initial = preview_name
        .chars()
        .next()
        .map(|ch| ch.to_uppercase().to_string())
        .unwrap_or_else(|| String::from("A"));

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", "Staff tools" }
                    h1 { class: "forum-desk-title", "Auto Moderation" }
                    p { class: "forum-desk-sub",
                        "Give the bot an identity, then tune the filters and actions it runs."
                    }
                }
                Button { "Save changes" }
            }

            div { class: "motion-cascade forum-status-strip",
                div { class: "min-w-0",
                    p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                        "Bot status"
                    }
                    p { class: "mt-1 text-base font-semibold tracking-tight",
                        "Auto Mod is watching public boards"
                    }
                    p { class: "mt-0.5 text-sm text-text-muted",
                        "Actions run instantly; staff still get a queue entry for high severity."
                    }
                }
                div { class: "w-full shrink-0 sm:max-w-xs",
                    SettingRow {
                        title: "Enable Auto Mod",
                        description: "Turn the bot on or off across the forum.",
                        enabled: true,
                    }
                }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Bot identity" }
                        p { class: "forum-editor-lede",
                            "Name, badge, and avatar shown on automated notices."
                        }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: "Display name",
                                hint: "Shown on every automated warning, mute, and hide notice.",
                                SignalInput {
                                    value: bot_name,
                                    placeholder: "ServerSpot AutoMod",
                                }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-2",
                                FormField {
                                    label: "Badge label",
                                    hint: "Short tag next to the name.",
                                    SignalInput { value: bot_tag, placeholder: "BOT" }
                                }
                                FormField { label: "Accent colour",
                                    ColorPicker { value: bot_accent }
                                }
                            }
                            MediaUploadField {
                                label: "Avatar",
                                hint: "Square image, PNG or WebP. Used in posts and DMs.",
                                value: bot_avatar,
                                tall: false,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Message templates" }
                        p { class: "forum-editor-lede",
                            "Copy sent when the bot warns or mutes a player."
                        }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: "Warning message",
                                hint: "Placeholders: {{author}}, {{rule}}, {{board}}.",
                                SignalTextarea {
                                    value: warn_message,
                                    placeholder: "Your post was flagged…",
                                }
                            }
                            FormField {
                                label: "Mute message",
                                hint: "Placeholders: {{author}}, {{duration}}, {{rule}}.",
                                SignalTextarea {
                                    value: mute_message,
                                    placeholder: "You’ve been muted…",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Bot thresholds" }
                        p { class: "forum-editor-lede",
                            "Word list and numeric limits the filters use."
                        }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: "Blocked words",
                                hint: "Comma-separated. Matching posts are held for review.",
                                SignalTextarea {
                                    value: blocked_words,
                                    placeholder: "spam phrase, invite link…",
                                }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-3",
                                FormField { label: "Max links",
                                    SignalInput { value: max_links, placeholder: "2" }
                                }
                                FormField {
                                    label: "Mute (min)",
                                    hint: "When Mute is chosen.",
                                    SignalInput {
                                        value: mute_minutes,
                                        placeholder: "30",
                                    }
                                }
                                FormField { label: "New acct (hrs)",
                                    SignalInput {
                                        value: new_account_hours,
                                        placeholder: "24",
                                    }
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Filters" }
                        p { class: "forum-editor-lede", "What the bot scans for on public boards." }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: "Block listed words & phrases",
                                description: "Flag or remove posts that match your blocked list.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "Limit external links",
                                description: "Stop posts that exceed the max link count.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "Detect duplicate spam",
                                description: "Catch near-identical replies posted in a short window.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "Throttle brand-new accounts",
                                description: "Require a waiting period before new accounts can post links.",
                                enabled: false,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Actions" }
                        p { class: "forum-editor-lede", "What happens when a filter trips." }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: "Auto-hide after three unique reports",
                                description: "Hide the post from public view until a moderator reviews it.",
                                enabled: false,
                            }
                            SettingRow {
                                title: "Warn on first offence",
                                description: "Send an automated warning before muting or hiding.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "Shadow-mute repeat offenders",
                                description: "Limit posting for accounts with three upheld reports in 7 days.",
                                enabled: false,
                            }
                            SettingRow {
                                title: "Post as the bot in-thread",
                                description: "Leave a public notice using the bot name and avatar when an action fires.",
                                enabled: true,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Notifications" }
                        p { class: "forum-editor-lede",
                            "Where staff and authors hear about auto-mod actions."
                        }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: "Notify staff Discord channel",
                                description: "Push high-severity auto-mod actions to your moderation webhook.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "DM the author",
                                description: "Tell the player what rule was triggered and what happens next.",
                                enabled: true,
                            }
                            SettingRow {
                                title: "Sign DMs with bot identity",
                                description: "Use the bot name and avatar on private warnings instead of a generic system sender.",
                                enabled: true,
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            "Preview"
                        }
                        div { class: "mt-4 flex items-start gap-3",
                            if preview_avatar.trim().is_empty() {
                                div {
                                    class: "flex h-11 w-11 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-text-on-accent",
                                    style: "background: {preview_accent};",
                                    "{preview_initial}"
                                }
                            } else {
                                img {
                                    src: "{preview_avatar}",
                                    alt: "{preview_name}",
                                    class: "h-11 w-11 shrink-0 rounded-full object-cover",
                                }
                            }
                            div { class: "min-w-0 flex-1",
                                div { class: "flex flex-wrap items-center gap-2",
                                    p { class: "text-sm font-semibold tracking-tight",
                                        "{preview_name}"
                                    }
                                    span {
                                        class: "inline-flex items-center rounded-squircle-sm px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide",
                                        style: "background: color-mix(in srgb, {preview_accent} 18%, transparent); color: {preview_accent};",
                                        "{preview_tag}"
                                    }
                                }
                                p { class: "mt-1.5 text-sm leading-relaxed text-text-secondary",
                                    "Hey NovaCraft — your post was flagged by Auto Mod for spam links. Please edit or remove it."
                                }
                                p { class: "mt-2 text-xs text-text-muted", "just now · automated" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ForumSiteSettings() -> Element {
    let stats = placeholder_forum_stats();
    let public_path = use_signal(|| String::from(stats.public_path));
    let page_title = use_signal(|| String::from("Forums"));

    rsx! {
        FeatureSettingsChrome { subtitle: "Path and defaults for the forum on your main website.",
            DataPanel { title: "Forum path",
                SettingsControl { label: "Public path",
                    SignalInput { value: public_path, placeholder: "/forum".to_string() }
                }
                SettingsField {
                    label: "Full URL",
                    value: format!("www.example.com{}", public_path()),
                }
                SettingsControl { label: "Page title",
                    SignalInput { value: page_title, placeholder: "Forums".to_string() }
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Community defaults",
                SettingRow {
                    title: "Allow guest reading",
                    description: "Anyone can browse public boards without an account.",
                    enabled: true,
                }
                SettingRow {
                    title: "Require login to reply",
                    description: "Guests can read; posting needs a linked player account.",
                    enabled: true,
                }
                SettingRow {
                    title: "Markdown & mentions",
                    description: "Enable formatting, @mentions, and spoiler tags.",
                    enabled: true,
                }
            }
        }
    }
}
