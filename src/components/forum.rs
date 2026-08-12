use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, SettingRow, SettingsControl, SettingsField,
};
use crate::components::ui::*;
use crate::i18n::t_key;
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
    fn label(self) -> String {
        match self {
            Self::Public => t_key("forum-visibility-public"),
            Self::Staff => t_key("forum-visibility-staff"),
            Self::Private => t_key("forum-visibility-private"),
        }
    }

    const fn tone(self) -> &'static str {
        match self {
            Self::Public => "#3ecf8e",
            Self::Staff => "#5b9dff",
            Self::Private => "#e5484d",
        }
    }

    fn hint(self) -> String {
        match self {
            Self::Public => t_key("forum-visibility-public-hint"),
            Self::Staff => t_key("forum-visibility-staff-hint"),
            Self::Private => t_key("forum-visibility-private-hint"),
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
        (true, true) => t_key("forum-thread-badge-pinned-locked"),
        (true, false) => t_key("forum-thread-badge-pinned"),
        (false, true) => t_key("forum-thread-badge-locked"),
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

    fn label(self) -> String {
        match self {
            Self::Pinned => t_key("forum-thread-status-pinned"),
            Self::Locked => t_key("forum-thread-status-locked"),
            Self::Open => t_key("forum-thread-status-open"),
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

    fn label(self) -> String {
        match self {
            Self::Recent => t_key("forum-thread-sort-recent"),
            Self::Replies => t_key("forum-thread-sort-replies"),
            Self::Title => t_key("forum-thread-sort-title"),
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

fn selection_summary(
    selected: &[String],
    empty: String,
    plural: impl Fn(usize) -> String,
) -> String {
    match selected.len() {
        0 => empty,
        1 => selected[0].clone(),
        n => plural(n),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportSeverity {
    High,
    Medium,
    Low,
}

impl ReportSeverity {
    fn label(self) -> String {
        match self {
            Self::High => t_key("forum-report-severity-high"),
            Self::Medium => t_key("forum-report-severity-medium"),
            Self::Low => t_key("forum-report-severity-low"),
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
    let _lang = i18n();
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
                    p { class: "forum-desk-eyebrow", { t!("forum-overview-eyebrow") } }
                    h1 { class: "forum-desk-title", { t!("forum-overview-title") } }
                    p { class: "forum-desk-sub", { t!("forum-overview-subtitle") } }
                }
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, { t!("forum-overview-view-site") } }
            }

            div { class: "motion-cascade ops-meter",
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-overview-posts-today") } }
                    p { class: "ops-meter-value is-accent", "{posts_today}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-overview-threads") } }
                    p { class: "ops-meter-value", "{thread_count}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-overview-members") } }
                    p { class: "ops-meter-value", "{members}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-overview-open-reports") } }
                    p { class: "ops-meter-value", "{report_count}" }
                }
            }

            if let Some(featured) = featured {
                section { class: "mb-9",
                    div { class: "ops-section-head",
                        h2 { class: "ops-section-title", { t!("forum-overview-hot-now") } }
                        span { class: "ops-section-sub", { t!("forum-overview-hot-sub") } }
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
                                    { t!(
                                        "forum-overview-featured-meta",
                                        category: featured.category.clone(),
                                        replies: featured.replies,
                                        when: featured.when.clone()
                                    ) }
                                }
                            }
                        }
                    }
                }
            }

            section { class: "mb-9",
                div { class: "ops-section-head",
                    h2 { class: "ops-section-title", { t!("forum-overview-needs-look") } }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumModeration {});
                        },
                        { t!("forum-overview-open-queue") }
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
                        p { class: "ops-pulse-title", { t!("forum-overview-boards") } }
                        p { class: "ops-pulse-meta", { t!("forum-overview-boards-meta") } }
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
                        p { class: "ops-pulse-title", { t!("forum-overview-threads") } }
                        p { class: "ops-pulse-meta", { t!("forum-overview-threads-meta") } }
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
                        p { class: "ops-pulse-title", { t!("forum-overview-auto-mod") } }
                        p { class: "ops-pulse-meta", { t!("forum-overview-auto-mod-meta") } }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
            }
        }
    }
}

#[component]
fn OverviewReportRow(report: Report) -> Element {
    let _lang = i18n();
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
    let _lang = i18n();
    let boards = use_context::<Signal<Vec<Board>>>();
    let navigator = use_navigator();
    let list = boards();

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", { t!("forum-boards-eyebrow") } }
                    h1 { class: "forum-desk-title", { t!("forum-boards-title") } }
                    p { class: "forum-desk-sub", { t!("forum-boards-subtitle") } }
                }
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ForumBoardNew {});
                    },
                    IconPlus {}
                    { t!("forum-boards-new") }
                }
            }

            div { class: "forum-vis-legend",
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Public.tone()};",
                    { t!("forum-visibility-public") }
                }
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Staff.tone()};",
                    { t!("forum-visibility-staff") }
                }
                span {
                    class: "forum-vis-chip",
                    style: "--chip-tone: {BoardVisibility::Private.tone()};",
                    { t!("forum-visibility-private") }
                }
            }

            div { class: "motion-cascade motion-cascade-tight forum-board-list",
                for board in list.into_iter() {
                    {
                        let board_id = board.id;
                        let counts = t!(
                            "forum-boards-counts",
                            threads: format_count(board.threads),
                            today: board.posts_today
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
    let _lang = i18n();
    rsx! {
        BoardEditor { board_id: None }
    }
}

#[component]
pub fn ForumBoardEdit(id: u64) -> Element {
    let _lang = i18n();
    rsx! {
        BoardEditor { board_id: Some(id) }
    }
}

#[component]
fn BoardEditor(board_id: Option<u64>) -> Element {
    let _lang = i18n();
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
        t_key("forum-board-preview-untitled")
    } else {
        name_now.trim().to_string()
    };
    let preview_description = if description_now.trim().is_empty() {
        t_key("forum-board-preview-no-description")
    } else {
        description_now.trim().to_string()
    };
    let activity_label = t!(
        "forum-boards-counts",
        threads: format_count(thread_count),
        today: posts_today
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
                    { t!("forum-board-back") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("forum-board-not-found-title") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("forum-board-not-found-desc") } }
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
                    { t!("forum-board-back") }
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
                            { t!("forum-board-delete") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumBoards {});
                        },
                        { t!("forum-board-cancel") }
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: save,
                        if is_new {
                            { t!("forum-board-create") }
                        } else {
                            { t!("forum-board-save") }
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("forum-boards-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        { t!("forum-board-new-title") }
                    } else {
                        { t!("forum-board-edit-title") }
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted", { t!("forum-board-lede") } }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-board-section-basics") } }
                        p { class: "forum-editor-lede", { t!("forum-board-section-basics-lede") } }
                        div { class: "mt-4 space-y-4",
                            FormField { label: t_key("forum-board-field-name"),
                                SignalInput { value: name, placeholder: t_key("forum-board-name-placeholder") }
                            }
                            FormField { label: t_key("forum-board-field-description"),
                                SignalTextarea {
                                    value: description,
                                    placeholder: t_key("forum-board-description-placeholder"),
                                    class: "min-h-[5.5rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-board-section-links") } }
                        p { class: "forum-editor-lede",
                            { t!("forum-board-section-links-lede", max: MAX_BOARD_LINKS) }
                        }
                        div { class: "mt-4 space-y-3",
                            div { class: "flex items-center justify-between gap-3",
                                p { class: "text-xs text-text-muted",
                                    { t!("forum-board-links-count", current: links_now.len(), max: MAX_BOARD_LINKS) }
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
                                    { t!("forum-board-add-link") }
                                }
                            }
                            if links_now.is_empty() {
                                p { class: "rounded-squircle-sm border border-dashed border-border-subtle px-3 py-4 text-sm text-text-muted",
                                    { t!("forum-board-no-links", max: MAX_BOARD_LINKS) }
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
                        h2 { class: "forum-editor-heading", { t!("forum-board-section-appearance") } }
                        p { class: "forum-editor-lede", { t!("forum-board-section-appearance-lede") } }
                        div { class: "mt-4",
                            FormField { label: t_key("forum-board-field-colour"),
                                ColorPicker { value: accent }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-board-section-visibility") } }
                        p { class: "forum-editor-lede", { t!("forum-board-section-visibility-lede") } }
                        div { class: "mt-4",
                            div {
                                class: "inline-flex w-full rounded-squircle-sm border border-border-subtle p-1",
                                style: "background: var(--color-surface);",
                                SegmentChoice {
                                    label: t_key("forum-visibility-public"),
                                    tone: BoardVisibility::Public.tone(),
                                    active: visibility_now == BoardVisibility::Public,
                                    onclick: move |_| visibility.set(BoardVisibility::Public),
                                }
                                SegmentChoice {
                                    label: t_key("forum-visibility-staff"),
                                    tone: BoardVisibility::Staff.tone(),
                                    active: visibility_now == BoardVisibility::Staff,
                                    onclick: move |_| visibility.set(BoardVisibility::Staff),
                                }
                                SegmentChoice {
                                    label: t_key("forum-visibility-private"),
                                    tone: BoardVisibility::Private.tone(),
                                    active: visibility_now == BoardVisibility::Private,
                                    onclick: move |_| visibility.set(BoardVisibility::Private),
                                }
                            }
                            p { class: "mt-2 text-xs text-text-muted", "{visibility_now.hint()}" }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-board-section-media") } }
                        p { class: "forum-editor-lede", { t!("forum-board-section-media-lede") } }
                        div { class: "mt-4 grid gap-5 sm:grid-cols-2",
                            MediaUploadField {
                                label: t_key("forum-board-field-image"),
                                hint: t_key("forum-board-field-image-hint"),
                                value: image,
                                tall: false,
                            }
                            MediaUploadField {
                                label: t_key("forum-board-field-banner"),
                                hint: t_key("forum-board-field-banner-hint"),
                                value: banner,
                                tall: true,
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("forum-board-preview") }
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
                            li { { t!("forum-board-preview-visibility", label: visibility_now.label()) } }
                            if filled_links > 0 {
                                li { { t!("forum-board-preview-links", count: filled_links) } }
                            } else {
                                li { { t!("forum-board-preview-no-links") } }
                            }
                            if !is_new {
                                li { "{activity_label}" }
                            }
                            if !banner_now.trim().is_empty() {
                                li { { t!("forum-board-preview-banner-set") } }
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
    let _lang = i18n();
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
                p { class: "text-xs font-medium text-text-secondary",
                    { t!("forum-board-link-index", index: index + 1) }
                }
                Button {
                    variant: ButtonVariant::Danger,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        links.write().retain(|item| item.id != link_id);
                    },
                    { t!("forum-board-link-remove") }
                }
            }
            div { class: "grid gap-2 sm:grid-cols-2",
                input {
                    r#type: "text",
                    class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                    placeholder: "{t!(\"forum-board-link-label-placeholder\")}",
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
                    placeholder: t_key("form-placeholder-url"),
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
    #[props(into)] label: String,
    #[props(into)] hint: String,
    mut value: Signal<String>,
    tall: bool,
) -> Element {
    let _lang = i18n();
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
                        { t!("forum-media-remove") }
                    }
                }
            }
            label { class: "{frame}", style: "background: var(--color-surface);",
                if current.trim().is_empty() {
                    span { class: "pointer-events-none px-3 text-center text-xs leading-relaxed text-text-muted",
                        { t!("forum-media-upload") }
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
    #[props(into)] label: String,
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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SegmentIdle {
    #[default]
    Soft,
    Outline,
}

#[component]
fn SegmentChoice(
    #[props(into)] label: String,
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
fn ToneChip(#[props(into)] label: String, tone: &'static str) -> Element {
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
    let _lang = i18n();
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
            t_key("forum-threads-result-one")
        } else {
            t!("forum-threads-result-many", count: len)
        }
    });
    let status_summary = use_memo(move || {
        let labels: Vec<String> = statuses()
            .iter()
            .filter_map(|key| ThreadStatusOption::from_key(key).map(|opt| opt.label()))
            .collect();
        selection_summary(
            &labels,
            t_key("forum-threads-filter-any-status"),
            |n| t!("forum-threads-filter-n-statuses", count: n),
        )
    });
    let board_summary = use_memo(move || {
        selection_summary(
            &board_filters(),
            t_key("forum-threads-filter-all-boards"),
            |n| t!("forum-threads-filter-n-boards", count: n),
        )
    });
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
                    p { class: "forum-desk-eyebrow", { t!("forum-threads-eyebrow") } }
                    h1 { class: "forum-desk-title", { t!("forum-threads-title") } }
                    p { class: "forum-desk-sub", { t!("forum-threads-subtitle") } }
                }
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ForumThreadNew {});
                    },
                    IconPlus {}
                    { t!("forum-threads-new") }
                }
            }

            div { class: "forum-toolbar mb-5",
                SearchInput {
                    value: query,
                    placeholder: "{t!(\"forum-threads-search-placeholder\")}",
                    class: "forum-toolbar-search",
                }

                div { class: "forum-toolbar-filters",
                    FilterMultiSelect {
                        label: t_key("forum-threads-filter-status-label"),
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
                                    label: option.label(),
                                    checked: statuses_now.iter().any(|item| item == option.key()),
                                    onclick: move |_| toggle_selection(statuses, option.key()),
                                }
                            }
                        },
                    }

                    FilterMultiSelect {
                        label: t_key("forum-threads-filter-boards-label"),
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
                                p { class: "px-3 py-2 text-xs text-text-muted", { t!("forum-threads-no-boards") } }
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
                        label: t_key("forum-threads-filter-sort-label"),
                        summary: sort_now.label(),
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
                                    label: option.label(),
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
                            { t!("forum-threads-clear-filters") }
                        }
                    }
                }

                p { class: "text-xs text-text-secondary", "{result_label()}" }
            }

            if filtered().is_empty() {
                p { class: "rounded-squircle-lg border border-dashed border-border-subtle px-4 py-10 text-center text-sm text-text-muted",
                    { t!("forum-threads-empty") }
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
    #[props(into)] label: String,
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
    let _lang = i18n();
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
        t_key("forum-thread-preview-title")
    } else {
        title_now.trim().to_string()
    };
    let preview_body = if body_now.trim().is_empty() {
        t_key("forum-thread-preview-body")
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
        t_key("forum-thread-preview-board")
    } else {
        board_now.clone()
    };
    let badge_line = thread_badge_line(pinned_now, locked_now);
    let preview_meta = t!(
        "forum-thread-preview-meta",
        author: user.name.clone()
    );

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
                    { t!("forum-thread-back") }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::ForumThreads {});
                        },
                        { t!("forum-thread-cancel") }
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: create,
                        { t!("forum-thread-create") }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("forum-thread-new-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("forum-thread-new-title") } }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted", { t!("forum-thread-new-lede") } }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-thread-section-basics") } }
                        p { class: "forum-editor-lede", { t!("forum-thread-section-basics-lede") } }
                        div { class: "mt-4 space-y-4",
                            FormField { label: t_key("forum-thread-field-title"),
                                SignalInput {
                                    value: title,
                                    placeholder: t_key("forum-thread-title-placeholder"),
                                }
                            }
                            FormField { label: t_key("forum-thread-field-opening"),
                                SignalTextarea {
                                    value: body,
                                    placeholder: "{t!(\"forum-thread-placeholder-opening\")}",
                                    class: "min-h-[9rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-thread-section-board") } }
                        p { class: "forum-editor-lede", { t!("forum-thread-section-board-lede") } }
                        div { class: "mt-4",
                            if boards_now.is_empty() {
                                SignalInput { value: board, placeholder: t_key("forum-thread-board-placeholder") }
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
                        h2 { class: "forum-editor-heading", { t!("forum-thread-section-options") } }
                        p { class: "forum-editor-lede", { t!("forum-thread-section-options-lede") } }
                        div { class: "mt-4 inline-flex w-full gap-2",
                            SegmentChoice {
                                label: t_key("forum-thread-status-pinned"),
                                tone: "#69bdf2",
                                active: pinned_now,
                                idle: SegmentIdle::Outline,
                                onclick: move |_| pinned.set(!pinned()),
                            }
                            SegmentChoice {
                                label: t_key("forum-thread-status-locked"),
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
                            { t!("forum-board-preview") }
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
                            li { { t!("forum-thread-preview-posted-as", name: user.name.clone()) } }
                            li {
                                if pinned_now {
                                    { t!("forum-thread-preview-pinned") }
                                } else {
                                    { t!("forum-thread-preview-not-pinned") }
                                }
                            }
                            li {
                                if locked_now {
                                    { t!("forum-thread-preview-locked") }
                                } else {
                                    { t!("forum-thread-preview-open") }
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
    let _lang = i18n();
    let navigator = use_navigator();
    let thread_id = thread.id;
    let replies = t!("forum-thread-card-replies", count: thread.replies);
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
    let _lang = i18n();
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
                    { t!("forum-thread-back") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("forum-thread-not-found-title") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("forum-thread-not-found-desc") } }
        };
    };

    let meta = {
        let mut parts = vec![thread.category.clone()];
        let badges = thread_badge_line(thread.pinned, thread.locked);
        if !badges.is_empty() {
            parts.push(badges);
        }
        parts.push(t!("forum-thread-card-replies", count: thread.replies));
        parts.push(thread.when.clone());
        parts.join(" · ")
    };
    let reply_samples = placeholder_replies(&thread);
    let pin_label = if thread.pinned {
        t_key("forum-thread-unpin")
    } else {
        t_key("forum-thread-pin")
    };
    let lock_label = if thread.locked {
        t_key("forum-thread-unlock")
    } else {
        t_key("forum-thread-lock")
    };
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
                { t!("forum-thread-back") }
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
                    { t!("forum-thread-move") }
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
                    { t!("forum-thread-delete") }
                }
                if move_menu_open && !boards_now.is_empty() {
                    div { class: "absolute right-0 top-full z-20 mt-2 w-56 rounded-squircle-sm border border-border-subtle bg-bg-elevated p-1 shadow-lg",
                        p { class: "px-2.5 py-1.5 text-[11px] font-medium uppercase tracking-wide text-text-muted",
                            { t!("forum-thread-move-to-board") }
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
                    span { class: "text-xs text-text-muted",
                        { t!("forum-thread-original-post", when: thread.when.clone()) }
                    }
                }
                p { class: "mt-3 text-[0.95rem] leading-relaxed text-text-secondary whitespace-pre-wrap",
                    "{thread.preview}"
                }
            }
        }

        section { class: "mt-2 mb-4 flex items-baseline justify-between gap-3 border-t border-border-subtle pt-6",
            p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                { t!("forum-thread-replies") }
            }
            span { class: "text-xs text-text-secondary", "{thread.replies}" }
        }

        if thread.locked {
            p { class: "mb-4 text-sm text-text-muted", { t!("forum-thread-locked-notice") } }
        }

        if reply_samples.is_empty() {
            p { class: "py-8 text-sm text-text-muted", { t!("forum-thread-no-replies") } }
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
                p { class: "mb-3 text-xs font-medium text-text-muted", { t!("forum-thread-reply-label") } }
                textarea {
                    class: "ui-input ui-squircle min-h-28 w-full resize-y px-4 py-3 text-sm outline-none",
                    placeholder: "{t!(\"forum-thread-reply-placeholder\")}",
                }
                div { class: "mt-3",
                    Button { { t!("forum-thread-post-reply") } }
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
    let _lang = i18n();
    let current_user = use_context::<Signal<CurrentUser>>();
    let user = current_user();
    let acting_as = t!(
        "forum-moderation-acting-as",
        name: user.name.clone(),
        role: user.role.clone()
    );
    let report_count = REPORTS.len();

    rsx! {
        div { class: "motion-cascade forum-desk",
            div { class: "forum-desk-masthead",
                div { class: "min-w-0",
                    p { class: "forum-desk-eyebrow", { t!("forum-moderation-eyebrow") } }
                    h1 { class: "forum-desk-title", { t!("forum-moderation-title") } }
                    p { class: "forum-desk-sub", "{acting_as}" }
                }
            }

            div { class: "motion-cascade ops-meter",
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-moderation-open-reports") } }
                    p { class: "ops-meter-value is-accent", "{report_count}" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-moderation-locked") } }
                    p { class: "ops-meter-value", "2" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-moderation-auto-hidden") } }
                    p { class: "ops-meter-value", "1" }
                }
                span { class: "ops-meter-divider" }
                div { class: "ops-meter-item",
                    p { class: "ops-meter-label", { t!("forum-moderation-avg-review") } }
                    p { class: "ops-meter-value", "18m" }
                }
            }

            section { class: "mb-4",
                div { class: "ops-section-head",
                    h2 { class: "ops-section-title", { t!("forum-moderation-report-queue") } }
                    span { class: "ops-section-sub", { t!("forum-moderation-report-queue-sub") } }
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
    let _lang = i18n();
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
                    Button { variant: ButtonVariant::Danger, size: ButtonSize::Sm, { t!("forum-moderation-hide-post") } }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        { t!("forum-moderation-warn-user") }
                    }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, { t!("forum-moderation-dismiss") } }
                }
            }
        }
    }
}

#[component]
pub fn ForumAutoModeration() -> Element {
    let _lang = i18n();
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
                    p { class: "forum-desk-eyebrow", { t!("forum-moderation-eyebrow") } }
                    h1 { class: "forum-desk-title", { t!("forum-auto-mod-title") } }
                    p { class: "forum-desk-sub", { t!("forum-auto-mod-subtitle") } }
                }
                Button { { t!("forum-auto-mod-save") } }
            }

            div { class: "motion-cascade forum-status-strip",
                div { class: "min-w-0",
                    p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                        { t!("forum-auto-mod-status-label") }
                    }
                    p { class: "mt-1 text-base font-semibold tracking-tight",
                        { t!("forum-auto-mod-status-title") }
                    }
                    p { class: "mt-0.5 text-sm text-text-muted", { t!("forum-auto-mod-status-desc") } }
                }
                div { class: "w-full shrink-0 sm:max-w-xs",
                    SettingRow {
                        title: t_key("forum-auto-mod-enable-title"),
                        description: t_key("forum-auto-mod-enable-desc"),
                        enabled: true,
                    }
                }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-identity") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-identity-lede") } }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: t_key("forum-auto-mod-field-display-name"),
                                hint: Some(t_key("forum-auto-mod-field-display-name-hint")),
                                SignalInput {
                                    value: bot_name,
                                    placeholder: t_key("forum-auto-mod-name-placeholder"),
                                }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-2",
                                FormField {
                                    label: t_key("forum-auto-mod-field-badge"),
                                    hint: Some(t_key("forum-auto-mod-field-badge-hint")),
                                    SignalInput { value: bot_tag, placeholder: t_key("forum-auto-mod-badge-placeholder") }
                                }
                                FormField { label: t_key("forum-auto-mod-field-accent"),
                                    ColorPicker { value: bot_accent }
                                }
                            }
                            MediaUploadField {
                                label: t_key("forum-auto-mod-field-avatar"),
                                hint: t_key("forum-auto-mod-field-avatar-hint"),
                                value: bot_avatar,
                                tall: false,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-templates") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-templates-lede") } }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: t_key("forum-auto-mod-field-warn-message"),
                                hint: Some(t_key("forum-auto-mod-field-warn-message-hint")),
                                SignalTextarea {
                                    value: warn_message,
                                    placeholder: t_key("forum-auto-mod-warn-placeholder"),
                                }
                            }
                            FormField {
                                label: t_key("forum-auto-mod-field-mute-message"),
                                hint: Some(t_key("forum-auto-mod-field-mute-message-hint")),
                                SignalTextarea {
                                    value: mute_message,
                                    placeholder: t_key("forum-auto-mod-mute-placeholder"),
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-thresholds") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-thresholds-lede") } }
                        div { class: "mt-4 space-y-4",
                            FormField {
                                label: t_key("forum-auto-mod-field-blocked-words"),
                                hint: Some(t_key("forum-auto-mod-field-blocked-words-hint")),
                                SignalTextarea {
                                    value: blocked_words,
                                    placeholder: t_key("forum-auto-mod-blocked-words-placeholder"),
                                }
                            }
                            div { class: "motion-cascade grid gap-4 sm:grid-cols-3",
                                FormField { label: t_key("forum-auto-mod-field-max-links"),
                                    SignalInput { value: max_links, placeholder: "2" }
                                }
                                FormField {
                                    label: t_key("forum-auto-mod-field-mute-min"),
                                    hint: Some(t_key("forum-auto-mod-field-mute-min-hint")),
                                    SignalInput {
                                        value: mute_minutes,
                                        placeholder: "30",
                                    }
                                }
                                FormField { label: t_key("forum-auto-mod-field-new-acct-hrs"),
                                    SignalInput {
                                        value: new_account_hours,
                                        placeholder: "24",
                                    }
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-filters") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-filters-lede") } }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: t_key("forum-auto-mod-filter-words-title"),
                                description: t_key("forum-auto-mod-filter-words-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-filter-links-title"),
                                description: t_key("forum-auto-mod-filter-links-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-filter-duplicate-title"),
                                description: t_key("forum-auto-mod-filter-duplicate-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-filter-new-acct-title"),
                                description: t_key("forum-auto-mod-filter-new-acct-desc"),
                                enabled: false,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-actions") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-actions-lede") } }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: t_key("forum-auto-mod-action-hide-title"),
                                description: t_key("forum-auto-mod-action-hide-desc"),
                                enabled: false,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-action-warn-title"),
                                description: t_key("forum-auto-mod-action-warn-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-action-shadow-mute-title"),
                                description: t_key("forum-auto-mod-action-shadow-mute-desc"),
                                enabled: false,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-action-bot-post-title"),
                                description: t_key("forum-auto-mod-action-bot-post-desc"),
                                enabled: true,
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("forum-auto-mod-section-notifications") } }
                        p { class: "forum-editor-lede", { t!("forum-auto-mod-section-notifications-lede") } }
                        div { class: "mt-2 divide-y divide-border-subtle",
                            SettingRow {
                                title: t_key("forum-auto-mod-notify-discord-title"),
                                description: t_key("forum-auto-mod-notify-discord-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-notify-dm-title"),
                                description: t_key("forum-auto-mod-notify-dm-desc"),
                                enabled: true,
                            }
                            SettingRow {
                                title: t_key("forum-auto-mod-notify-sign-dm-title"),
                                description: t_key("forum-auto-mod-notify-sign-dm-desc"),
                                enabled: true,
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("forum-board-preview") }
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
                                p { class: "mt-2 text-xs text-text-muted",
                                    { t!("forum-auto-mod-preview-meta") }
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
pub fn ForumSiteSettings() -> Element {
    let _lang = i18n();
    let stats = placeholder_forum_stats();
    let public_path = use_signal(|| String::from(stats.public_path));
    let page_title = use_signal(|| String::from("Forums"));

    rsx! {
        FeatureSettingsChrome { subtitle: t_key("forum-settings-subtitle"),
            DataPanel { title: t_key("forum-settings-panel-path"),
                SettingsControl { label: t_key("forum-settings-field-public-path"),
                    SignalInput { value: public_path, placeholder: t_key("forum-settings-placeholder-path") }
                }
                SettingsField {
                    label: t_key("forum-settings-field-full-url"),
                    value: format!("www.example.com{}", public_path()),
                }
                SettingsControl { label: t_key("forum-settings-field-page-title"),
                    SignalInput { value: page_title, placeholder: t_key("forum-settings-placeholder-page-title") }
                }
                p { class: "pt-3 text-xs text-text-muted", { t!("forum-settings-domain-hint") } }
            }
            DataPanel { title: t_key("forum-settings-panel-defaults"),
                SettingRow {
                    title: t_key("forum-settings-guest-reading-title"),
                    description: t_key("forum-settings-guest-reading-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("forum-settings-login-reply-title"),
                    description: t_key("forum-settings-login-reply-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("forum-settings-markdown-title"),
                    description: t_key("forum-settings-markdown-desc"),
                    enabled: true,
                }
            }
        }
    }
}
