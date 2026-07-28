use dioxus::prelude::*;

use crate::components::page::{DataPanel, PageHeader, SettingRow};
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

#[derive(Clone, Copy)]
struct BoardForm {
    name: Signal<String>,
    description: Signal<String>,
    image: Signal<String>,
    banner: Signal<String>,
    links: Signal<Vec<BoardLinkDraft>>,
    next_link_id: Signal<u64>,
    visibility: Signal<BoardVisibility>,
    accent: Signal<String>,
    editing_id: Signal<Option<u64>>,
}

impl BoardForm {
    fn clear(mut self) {
        self.name.set(String::new());
        self.description.set(String::new());
        self.image.set(String::new());
        self.banner.set(String::new());
        self.links.set(Vec::new());
        self.next_link_id.set(1);
        self.visibility.set(BoardVisibility::Public);
        self.accent.set(default_board_accent());
        self.editing_id.set(None);
    }

    fn load(mut self, board: &Board) {
        self.name.set(board.name.clone());
        self.description.set(board.description.clone());
        self.image.set(board.image.clone());
        self.banner.set(board.banner.clone());
        let (drafts, next_id) = links_to_drafts(&board.links);
        self.links.set(drafts);
        self.next_link_id.set(next_id);
        self.visibility.set(board.visibility);
        self.accent.set(board.accent.clone());
        self.editing_id.set(Some(board.id));
    }
}

fn clear_thread_form(
    mut title: Signal<String>,
    mut body: Signal<String>,
    mut board: Signal<String>,
    mut pinned: Signal<bool>,
    mut locked: Signal<bool>,
    boards: &[Board],
) {
    title.set(String::new());
    body.set(String::new());
    board.set(default_thread_board(boards));
    pinned.set(false);
    locked.set(false);
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
    let current_user = use_context::<Signal<CurrentUser>>();
    let threads = use_context::<Signal<Vec<Thread>>>();
    let user = current_user();
    let stats = placeholder_forum_stats();
    let navigator = use_navigator();

    let posts_today = format_count(stats.posts_today);
    let members = format_count(stats.members);
    let thread_count = format_count(stats.threads);
    let domain = format!("www.example.com{}", stats.public_path);
    let hello = format!("Signed in as {} · {}", user.name, user.role);
    let open_reports = REPORTS.len();
    let mod_blurb = format!("{open_reports} reports need a look");
    let reports_open = format!("{open_reports} open");
    let featured = threads.read().first().cloned();

    rsx! {
        PageHeader {
            title: "Forum",
            subtitle: "What’s happening in your community — then jump into boards, threads, or moderation.",
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "View on website"
                }
            },
        }

        section {
            class: "mb-8 flex flex-col gap-4 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4 sm:flex-row sm:items-center sm:gap-5 sm:p-5",
            div {
                class: "flex h-14 w-14 shrink-0 items-center justify-center rounded-squircle-sm text-text-on-accent",
                style: "background: {FORUM_ACCENT};",
                IconForum {}
            }
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "mb-1 flex flex-wrap items-center gap-2",
                    span {
                        class: "inline-flex items-center gap-1.5 text-xs font-medium text-success",
                        span { class: "forum-live-dot h-1.5 w-1.5 rounded-full bg-success" }
                        "Live"
                    }
                    span {
                        class: "rounded-squircle-sm border border-border-subtle bg-surface/40 px-2 py-0.5 font-mono text-xs text-text-muted",
                        "{domain}"
                    }
                }
                p { class: "text-lg font-semibold tracking-tight", "{posts_today} posts today" }
                p { class: "mt-0.5 text-sm text-text-muted", "{thread_count} threads · {members} members · {hello}" }
            }
        }

        if let Some(featured) = featured {
            section {
                class: "mb-10",
                p { class: "mb-3 text-xs font-medium uppercase tracking-wide text-text-muted", "Hot right now" }
                article {
                    class: "forum-spotlight",
                    Avatar {
                        email: featured.author_email.clone(),
                        size: 44,
                        alt: featured.author.clone(),
                        class: "shrink-0",
                    }
                    div {
                        class: "min-w-0 flex-1",
                        h2 { class: "text-xl font-semibold tracking-tight sm:text-2xl", "{featured.title}" }
                        p { class: "mt-2 max-w-2xl text-sm leading-relaxed text-text-secondary", "{featured.preview}" }
                        p {
                            class: "mt-3 text-xs text-text-muted",
                            "{featured.category} · {featured.author} · {featured.replies} replies · {featured.when}"
                        }
                    }
                }
            }
        }

        section {
            class: "mb-10",
            div {
                class: "mb-4 flex flex-wrap items-end justify-between gap-3",
                div {
                    p { class: "text-xs font-medium uppercase tracking-wide text-text-muted", "For moderators" }
                    p { class: "mt-1 text-sm text-text-secondary", "Open reports and items waiting on staff." }
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ForumModeration {});
                    },
                    "Open queue"
                }
            }
            div {
                class: "mb-4 grid grid-cols-3 gap-3",
                ModStat { label: "Reports", value: reports_open, tone: "#f87171" }
                ModStat { label: "Locked", value: "2 threads", tone: "#f0a35e" }
                ModStat { label: "Auto-hidden", value: "1 post", tone: "#858899" }
            }
            for report in REPORTS.iter().copied() {
                OverviewReportRow { report }
            }
        }

        section {
            class: "mb-10",
            p { class: "mb-1 text-xs font-medium uppercase tracking-wide text-text-muted", "Jump to" }
            div {
                class: "sm:columns-2 sm:gap-x-12",
                OverviewLink {
                    title: "Boards",
                    blurb: "Visibility, structure, and daily heat",
                    accent: FORUM_ACCENT,
                    onclick: move |_| {
                        navigator.push(Route::ForumBoards {});
                    },
                }
                OverviewLink {
                    title: "Threads",
                    blurb: "Browse every active conversation",
                    accent: "#69bdf2",
                    onclick: move |_| {
                        navigator.push(Route::ForumThreads {});
                    },
                }
                OverviewLink {
                    title: "Moderation",
                    blurb: mod_blurb,
                    accent: "#f87171",
                    onclick: move |_| {
                        navigator.push(Route::ForumModeration {});
                    },
                }
                OverviewLink {
                    title: "Auto Moderation",
                    blurb: "Filters, actions, and bot guardrails",
                    accent: "#f0a35e",
                    onclick: move |_| {
                        navigator.push(Route::ForumAutoModeration {});
                    },
                }
                OverviewLink {
                    title: "Settings",
                    blurb: "Path and community defaults",
                    accent: "#5eead4",
                    onclick: move |_| {
                        navigator.push(Route::ForumSiteSettings {});
                    },
                }
            }
        }
    }
}

#[component]
fn OverviewLink(
    title: &'static str,
    #[props(into)] blurb: String,
    accent: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "module-index-row group break-inside-avoid",
            style: "--row-accent: {accent};",
            onclick: move |evt| onclick.call(evt),
            div {
                class: "module-index-icon rounded-squircle-sm",
                style: "background: {accent};",
                IconForum {}
            }
            div {
                class: "min-w-0 flex-1",
                p { class: "module-index-title", "{title}" }
                p { class: "module-index-blurb", "{blurb}" }
            }
        }
    }
}

#[component]
fn ModStat(label: &'static str, #[props(into)] value: String, tone: &'static str) -> Element {
    rsx! {
        div {
            class: "rounded-squircle-lg border border-border-subtle bg-surface/20 px-3 py-3",
            p {
                class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                "{label}"
            }
            p {
                class: "mt-1 text-sm font-semibold tracking-tight tabular-nums",
                style: "color: {tone};",
                "{value}"
            }
        }
    }
}

#[component]
fn OverviewReportRow(report: Report) -> Element {
    let meta = report.meta();
    let severity = report.severity;

    rsx! {
        article {
            class: "forum-report",
            style: "--report-tone: {severity.tone()};",
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "flex flex-wrap items-center gap-2",
                    h3 { class: "text-sm font-semibold tracking-tight", "{report.title}" }
                    ToneChip { label: severity.label(), tone: severity.tone() }
                }
                p { class: "mt-1 text-xs text-text-muted", "{meta}" }
            }
        }
    }
}

#[component]
fn BoardRow(
    board: Board,
    #[props(default)] on_edit: Option<EventHandler<u64>>,
    #[props(default)] on_delete: Option<EventHandler<u64>>,
) -> Element {
    let counts = format!(
        "{} threads · {} today",
        format_count(board.threads),
        board.posts_today
    );
    let links_meta = if board.links.is_empty() {
        String::new()
    } else {
        format!("{} links", board.links.len())
    };
    let board_id = board.id;
    let editable = on_edit.is_some();

    rsx! {
        article {
            class: if editable {
                "forum-board-card forum-board-card-clickable"
            } else {
                "forum-board-card"
            },
            onclick: move |_| {
                if let Some(on_edit) = on_edit {
                    on_edit.call(board_id);
                }
            },
            if !board.banner.is_empty() {
                div {
                    class: "forum-board-banner",
                    img {
                        src: "{board.banner}",
                        alt: "",
                        class: "h-full w-full object-cover",
                    }
                }
            }
            div {
                class: "module-index-row",
                style: "--row-accent: {board.accent};",
                if board.image.is_empty() {
                    div {
                        class: "module-index-icon rounded-squircle-sm",
                        style: "background: {board.accent};",
                        IconForum {}
                    }
                } else {
                    img {
                        src: "{board.image}",
                        alt: "{board.name}",
                        class: "module-index-icon rounded-squircle-sm object-cover",
                    }
                }
                div {
                    class: "min-w-0 flex-1",
                    div {
                        class: "flex flex-wrap items-center gap-2",
                        p { class: "module-index-title", "{board.name}" }
                        span {
                            class: "rounded-squircle-sm px-2 py-0.5 text-[11px] font-medium",
                            style: "background: color-mix(in srgb, {board.visibility.tone()} 16%, transparent); color: {board.visibility.tone()};",
                            "{board.visibility.label()}"
                        }
                    }
                    p { class: "module-index-blurb", "{board.description}" }
                    if !links_meta.is_empty() {
                        p { class: "mt-1.5 text-xs text-text-secondary", "{links_meta}" }
                    }
                }
                div {
                    class: "flex shrink-0 flex-col items-end gap-2",
                    span { class: "hidden text-xs tabular-nums text-text-muted sm:block", "{counts}" }
                    if let Some(on_delete) = on_delete {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |evt: MouseEvent| {
                                evt.stop_propagation();
                                on_delete.call(board_id);
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ForumBoards() -> Element {
    let mut boards = use_context::<Signal<Vec<Board>>>();
    let mut open = use_signal(|| false);
    let form = BoardForm {
        name: use_signal(String::new),
        description: use_signal(String::new),
        image: use_signal(String::new),
        banner: use_signal(String::new),
        links: use_signal(Vec::<BoardLinkDraft>::new),
        next_link_id: use_signal(|| 1_u64),
        visibility: use_signal(|| BoardVisibility::Public),
        accent: use_signal(default_board_accent),
        editing_id: use_signal(|| None::<u64>),
    };

    let open_create = move |_| {
        form.clear();
        open.set(true);
    };

    rsx! {
        PageHeader {
            title: "Boards",
            subtitle: "The boards players see on your site — keep the busy ones easy to find.",
            action: rsx! {
                Button {
                    onclick: open_create,
                    IconPlus {}
                    "New board"
                }
            },
        }

        div {
            class: "mb-5 flex flex-wrap gap-2",
            ToneChip { label: "Public", tone: BoardVisibility::Public.tone() }
            ToneChip { label: "Staff", tone: BoardVisibility::Staff.tone() }
            ToneChip { label: "Private", tone: BoardVisibility::Private.tone() }
        }

        div {
            class: "flex flex-col gap-3 sm:gap-4",
            for board in boards() {
                BoardRow {
                    board,
                    on_edit: move |id| {
                        if let Some(existing) =
                            boards.read().iter().find(|board| board.id == id).cloned()
                        {
                            form.load(&existing);
                            open.set(true);
                        }
                    },
                    on_delete: move |id| {
                        boards.with_mut(|list| {
                            list.retain(|board| board.id != id);
                        });
                        if (form.editing_id)() == Some(id) {
                            form.clear();
                            open.set(false);
                        }
                    },
                }
            }
        }

        BoardModal {
            open,
            editing_id: form.editing_id,
            boards,
            name: form.name,
            description: form.description,
            image: form.image,
            banner: form.banner,
            links: form.links,
            next_link_id: form.next_link_id,
            visibility: form.visibility,
            accent: form.accent,
            on_close: move |_| {
                form.clear();
                open.set(false);
            },
        }
    }
}

#[component]
fn BoardModal(
    open: Signal<bool>,
    editing_id: Signal<Option<u64>>,
    mut boards: Signal<Vec<Board>>,
    mut name: Signal<String>,
    mut description: Signal<String>,
    mut image: Signal<String>,
    mut banner: Signal<String>,
    mut links: Signal<Vec<BoardLinkDraft>>,
    mut next_link_id: Signal<u64>,
    mut visibility: Signal<BoardVisibility>,
    mut accent: Signal<String>,
    on_close: EventHandler<MouseEvent>,
) -> Element {
    let name_now = name();
    let description_now = description();
    let image_now = image();
    let banner_now = banner();
    let links_now = links();
    let visibility_now = visibility();
    let accent_now = accent();
    let editing = editing_id();
    let is_edit = editing.is_some();
    let can_save = !name_now.trim().is_empty();

    let title = if is_edit { "Edit board" } else { "New board" };
    let save_label = if is_edit {
        "Save changes"
    } else {
        "Create board"
    };
    let modal_description = if is_edit {
        "Update this board’s name, media, description, and links."
    } else {
        "Name, media, description, and any links players should see on this board."
    };

    let preview_name = if name_now.trim().is_empty() {
        String::from("Board name")
    } else {
        name_now.trim().to_string()
    };
    let preview_description = if description_now.trim().is_empty() {
        String::from("Short description players will see under the board.")
    } else {
        description_now.trim().to_string()
    };
    let preview_name_class = if name_now.trim().is_empty() {
        "module-index-title text-text-muted"
    } else {
        "module-index-title"
    };
    let preview_description_class = if description_now.trim().is_empty() {
        "module-index-blurb opacity-70"
    } else {
        "module-index-blurb"
    };
    let filled_links = links_now
        .iter()
        .filter(|link| !link.label.trim().is_empty() || !link.url.trim().is_empty())
        .count();

    rsx! {
        Modal {
            open,
            size: ModalSize::Lg,
            title,
            description: modal_description,
            footer: rsx! {
                if is_edit {
                    Button {
                        variant: ButtonVariant::Danger,
                        onclick: move |evt| {
                            if let Some(id) = editing_id() {
                                boards.with_mut(|list| {
                                    list.retain(|board| board.id != id);
                                });
                            }
                            on_close.call(evt);
                        },
                        "Delete board"
                    }
                }
                div { class: "flex-1" }
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |evt| on_close.call(evt),
                    "Cancel"
                }
                Button {
                    disabled: !can_save,
                    onclick: move |evt| {
                        let collected = collect_board_links(&links());
                        let visibility_value = visibility();
                        let accent_value = accent();
                        let name_value = name().trim().to_string();
                        let description_value = description().trim().to_string();
                        let image_value = image();
                        let banner_value = banner();

                        if let Some(id) = editing_id() {
                            boards.with_mut(|list| {
                                if let Some(board) = list.iter_mut().find(|board| board.id == id) {
                                    board.name = name_value;
                                    board.description = description_value;
                                    board.image = image_value;
                                    board.banner = banner_value;
                                    board.links = collected;
                                    board.visibility = visibility_value;
                                    board.accent = accent_value;
                                }
                            });
                        } else {
                            let id = next_board_id(&boards());
                            boards.write().push(Board {
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

                        on_close.call(evt);
                    },
                    "{save_label}"
                }
            },

            div {
                class: "grid h-full min-h-0 gap-6 lg:grid-cols-2 lg:gap-8",

                div {
                    class: "space-y-5",
                    FormField { label: "Name",
                        SignalInput {
                            value: name,
                            placeholder: "Survival",
                        }
                    }
                    FormField { label: "Description",
                        SignalTextarea {
                            value: description,
                            placeholder: "Builds, bases, and day-to-day talk…",
                            class: "min-h-[5.5rem]",
                        }
                    }

                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between gap-3",
                            div {
                                p { class: "text-xs font-medium text-text-muted", "Links" }
                                p {
                                    class: "text-xs text-text-muted/80",
                                    "{links_now.len()} / {MAX_BOARD_LINKS}"
                                }
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
                                    links.write().push(BoardLinkDraft {
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
                            p {
                                class: "rounded-squircle-sm border border-dashed border-border-subtle px-3 py-4 text-sm text-text-muted",
                                "No links yet. Add up to {MAX_BOARD_LINKS} — Discord, maps, docs, etc."
                            }
                        } else {
                            div {
                                class: "space-y-3",
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

                    FormField { label: "Colour",
                        ColorPicker { value: accent }
                    }

                    FormField { label: "Visibility",
                        div {
                            class: "inline-flex w-full rounded-squircle-sm border border-border-subtle bg-surface/20 p-1",
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
                        p {
                            class: "mt-2 text-xs text-text-muted",
                            "{visibility_now.hint()}"
                        }
                    }
                }

                div {
                    class: "space-y-5",
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

                    div {
                        class: "overflow-hidden rounded-squircle-lg border border-border-subtle bg-surface/20",
                        p {
                            class: "px-3 pt-3 text-[11px] font-medium uppercase tracking-wide text-text-muted",
                            "Preview"
                        }
                        if !banner_now.trim().is_empty() {
                            div {
                                class: "mt-3 h-24 w-full overflow-hidden bg-surface-2 sm:h-28",
                                img {
                                    src: "{banner_now}",
                                    alt: "",
                                    class: "h-full w-full object-cover",
                                }
                            }
                        }
                        div {
                            class: "module-index-row pointer-events-none",
                            style: "--row-accent: {accent_now};",
                            if image_now.trim().is_empty() {
                                div {
                                    class: "module-index-icon rounded-squircle-sm",
                                    style: "background: {accent_now};",
                                    IconForum {}
                                }
                            } else {
                                img {
                                    src: "{image_now}",
                                    alt: "",
                                    class: "module-index-icon rounded-squircle-sm object-cover",
                                }
                            }
                            div {
                                class: "min-w-0 flex-1",
                                div {
                                    class: "flex flex-wrap items-center gap-2",
                                    p { class: "{preview_name_class}", "{preview_name}" }
                                    span {
                                        class: "rounded-squircle-sm px-2 py-0.5 text-[11px] font-medium",
                                        style: "background: color-mix(in srgb, {visibility_now.tone()} 16%, transparent); color: {visibility_now.tone()};",
                                        "{visibility_now.label()}"
                                    }
                                }
                                p { class: "{preview_description_class}", "{preview_description}" }
                                if filled_links > 0 {
                                    p {
                                        class: "mt-1.5 text-xs text-text-secondary",
                                        "{filled_links} links"
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
fn BoardLinkEditor(mut links: Signal<Vec<BoardLinkDraft>>, index: usize, link_id: u64) -> Element {
    let link = links().get(index).cloned().unwrap_or(BoardLinkDraft {
        id: link_id,
        label: String::new(),
        url: String::new(),
    });

    rsx! {
        div {
            class: "rounded-squircle-sm border border-border-subtle bg-surface/15 p-3",
            div {
                class: "mb-2 flex items-center justify-between gap-2",
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
            div {
                class: "grid gap-2 sm:grid-cols-2",
                input {
                    r#type: "text",
                    class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                    placeholder: "Label",
                    value: "{link.label}",
                    oninput: move |evt: FormEvent| {
                        let next = evt.value();
                        links.with_mut(|list| {
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
                        links.with_mut(|list| {
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
        "relative flex h-32 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm border border-dashed border-border-subtle bg-surface/20 transition-colors hover:border-border hover:bg-surface/30"
    } else {
        "relative flex h-32 w-32 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm border border-dashed border-border-subtle bg-surface/20 transition-colors hover:border-border hover:bg-surface/30"
    };

    rsx! {
        div {
            class: "space-y-2",
            div {
                class: "flex items-start justify-between gap-3",
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
            label {
                class: "{frame}",
                if current.trim().is_empty() {
                    span {
                        class: "pointer-events-none px-3 text-center text-xs leading-relaxed text-text-muted",
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
            label {
                class: "mb-1.5 block text-xs font-medium text-text-muted",
                "{label}"
            }
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
    // Always set a full style string — clearing to "" leaves the previous colour stuck in the DOM.
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
    let current_user = use_context::<Signal<CurrentUser>>();
    let mut open = use_signal(|| false);
    let title = use_signal(String::new);
    let body = use_signal(String::new);
    let board = use_signal(|| default_thread_board(&boards()));
    let pinned = use_signal(|| false);
    let locked = use_signal(|| false);

    let mut query = use_signal(String::new);
    let mut statuses = use_signal(Vec::<String>::new);
    let mut board_filters = use_signal(Vec::<String>::new);
    let mut sort = use_signal(ThreadSort::default);
    let mut status_menu = use_signal(|| false);
    let mut board_menu = use_signal(|| false);
    let mut sort_menu = use_signal(|| false);

    let open_create = move |_| {
        clear_thread_form(title, body, board, pinned, locked, &boards());
        open.set(true);
    };

    let boards_now = boards();
    let statuses_now = statuses();
    let board_filters_now = board_filters();
    let sort_now = sort();
    let filtered = filter_threads(
        &threads(),
        &query(),
        &board_filters_now,
        &statuses_now,
        sort_now,
    );
    let result_label = if filtered.len() == 1 {
        String::from("1 thread")
    } else {
        format!("{} threads", filtered.len())
    };
    let status_summary = {
        let labels: Vec<String> = statuses_now
            .iter()
            .filter_map(|key| ThreadStatusOption::from_key(key).map(|opt| opt.label().to_string()))
            .collect();
        selection_summary(&labels, "Any status", "statuses")
    };
    let board_summary = selection_summary(&board_filters_now, "All boards", "boards");
    let filters_active = !query().trim().is_empty()
        || !statuses_now.is_empty()
        || !board_filters_now.is_empty()
        || sort_now != ThreadSort::Recent;

    rsx! {
        PageHeader {
            title: "Threads",
            subtitle: "Search, filter, and open any conversation.",
            action: rsx! {
                Button {
                    onclick: open_create,
                    IconPlus {}
                    "New thread"
                }
            },
        }

        section {
            class: "mb-6 space-y-3 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4",
            SearchInput {
                value: query,
                placeholder: "Search title, author, board…",
                class: "w-full",
            }

            div {
                class: "grid gap-3 sm:grid-cols-2 lg:grid-cols-4",

                FilterMultiSelect {
                    label: "Status",
                    summary: status_summary,
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
                    summary: board_summary,
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

                div {
                    class: "flex items-end",
                    Button {
                        variant: ButtonVariant::Ghost,
                        full_width: true,
                        disabled: !filters_active,
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

            p { class: "text-xs text-text-secondary", "{result_label}" }
        }

        if filtered.is_empty() {
            p {
                class: "rounded-squircle-lg border border-dashed border-border-subtle px-4 py-10 text-center text-sm text-text-muted",
                "No threads match these filters."
            }
        } else {
            for thread in filtered.into_iter() {
                ThreadCard { thread }
            }
        }

        ThreadModal {
            open,
            threads,
            boards,
            current_user,
            title,
            body,
            board,
            pinned,
            locked,
            on_close: move |_| {
                clear_thread_form(title, body, board, pinned, locked, &boards());
                open.set(false);
            },
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
        div {
            class: "relative space-y-1.5",
            label { class: "block text-xs font-medium text-text-muted", "{label}" }
            button {
                r#type: "button",
                class: "ui-input ui-squircle flex h-10 w-full items-center justify-between gap-2 px-3 text-left text-sm outline-none",
                onclick: move |evt| on_toggle_menu.call(evt),
                span { class: "min-w-0 truncate text-text", "{summary}" }
                span { class: "shrink-0 text-xs text-text-muted", if is_open { "▴" } else { "▾" } }
            }
            if is_open {
                div {
                    class: "absolute left-0 right-0 z-20 mt-1 max-h-56 overflow-y-auto rounded-squircle-sm border border-border-subtle bg-bg-elevated p-1 shadow-lg",
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
            class: "flex w-full items-center gap-2.5 rounded-squircle-sm px-2.5 py-2 text-left text-sm transition-colors hover:bg-surface/40",
            onclick: move |evt| onclick.call(evt),
            span {
                class: if checked {
                    "flex h-4 w-4 shrink-0 items-center justify-center rounded-[0.35rem] text-[10px] font-bold text-text-on-accent"
                } else {
                    "flex h-4 w-4 shrink-0 items-center justify-center rounded-[0.35rem] border border-border-subtle bg-surface/30 text-[10px]"
                },
                style: if checked {
                    format!("background: {FORUM_ACCENT};")
                } else {
                    String::new()
                },
                if checked { "✓" } else { "" }
            }
            span { class: "min-w-0 truncate text-text", "{label}" }
        }
    }
}

#[component]
fn ThreadModal(
    open: Signal<bool>,
    mut threads: Signal<Vec<Thread>>,
    boards: Signal<Vec<Board>>,
    current_user: Signal<CurrentUser>,
    mut title: Signal<String>,
    mut body: Signal<String>,
    mut board: Signal<String>,
    mut pinned: Signal<bool>,
    mut locked: Signal<bool>,
    on_close: EventHandler<MouseEvent>,
) -> Element {
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

    rsx! {
        Modal {
            open,
            size: ModalSize::Lg,
            title: "New thread",
            description: "Title, board, opening post, and whether it should be pinned or locked.",
            footer: rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |evt| on_close.call(evt),
                    "Cancel"
                }
                Button {
                    disabled: !can_save,
                    onclick: move |evt| {
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
                        on_close.call(evt);
                    },
                    "Create thread"
                }
            },

            div {
                class: "grid h-full min-h-0 gap-6 lg:grid-cols-2 lg:gap-8",

                div {
                    class: "space-y-5",
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
                    FormField { label: "Board",
                        if boards_now.is_empty() {
                            SignalInput {
                                value: board,
                                placeholder: "General",
                            }
                        } else {
                            div {
                                class: "flex flex-wrap gap-2",
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
                    FormField { label: "Options",
                        div {
                            class: "inline-flex w-full gap-2",
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
                        p {
                            class: "mt-2 text-xs text-text-muted",
                            "Pinned threads stay at the top. Locked threads stay visible but closed to replies."
                        }
                    }
                }

                div {
                    class: "space-y-5",
                    div {
                        class: "overflow-hidden rounded-squircle-lg border border-border-subtle bg-surface/20",
                        p {
                            class: "px-3 pt-3 text-[11px] font-medium uppercase tracking-wide text-text-muted",
                            "Preview"
                        }
                        article {
                            class: "forum-thread forum-thread-static pointer-events-none border-b-0",
                            Avatar {
                                email: user.email.clone(),
                                size: 40,
                                alt: user.name.clone(),
                                class: "mt-0.5 shrink-0",
                            }
                            div {
                                class: "min-w-0 flex-1",
                                div {
                                    class: "flex flex-wrap items-center gap-x-2 gap-y-1",
                                    h3 { class: "{preview_title_class}", "{preview_title}" }
                                    span {
                                        class: "rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-[11px] font-medium text-text-secondary",
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
                                p {
                                    class: "mt-3 text-xs text-text-secondary",
                                    "{preview_meta}"
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
                navigator.push(Route::ForumThread { id: thread_id });
            },
            Avatar {
                email: thread.author_email.clone(),
                size: 40,
                alt: thread.author.clone(),
                class: "mt-0.5 shrink-0",
            }
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "flex flex-wrap items-center gap-x-2 gap-y-1",
                    h3 { class: "text-base font-semibold tracking-tight sm:text-lg", "{thread.title}" }
                    span {
                        class: "rounded-squircle-sm bg-surface-2 px-2 py-0.5 text-[11px] font-medium text-text-secondary",
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
                p {
                    class: "mt-1.5 max-w-3xl text-sm leading-relaxed text-text-muted",
                    "{thread.preview}"
                }
                p {
                    class: "mt-3 text-xs text-text-secondary",
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
            PageHeader {
                title: "Thread not found",
                subtitle: "This conversation may have been removed.",
                action: rsx! {
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            navigator.push(Route::ForumThreads {});
                        },
                        "Back to threads"
                    }
                },
            }
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
        div {
            class: "mb-8 flex flex-wrap items-center justify-between gap-3",
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    navigator.push(Route::ForumThreads {});
                },
                "← Threads"
            }
            div {
                class: "relative flex flex-wrap items-center justify-end gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        threads.with_mut(|list| {
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
                        threads.with_mut(|list| {
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
                        threads.with_mut(|list| {
                            list.retain(|item| item.id != id);
                        });
                        navigator.push(Route::ForumThreads {});
                    },
                    "Delete"
                }
                if move_menu_open && !boards_now.is_empty() {
                    div {
                        class: "absolute right-0 top-full z-20 mt-2 w-56 rounded-squircle-sm border border-border-subtle bg-bg-elevated p-1 shadow-lg",
                        p {
                            class: "px-2.5 py-1.5 text-[11px] font-medium uppercase tracking-wide text-text-muted",
                            "Move to board"
                        }
                        for option in boards_now.into_iter() {
                            button {
                                r#type: "button",
                                class: if thread_board == option.name {
                                    "flex w-full items-center gap-2 rounded-squircle-sm px-2.5 py-2 text-left text-sm font-medium text-text"
                                } else {
                                    "flex w-full items-center gap-2 rounded-squircle-sm px-2.5 py-2 text-left text-sm text-text-muted hover:bg-surface/40 hover:text-text"
                                },
                                onclick: {
                                    let name = option.name.clone();
                                    move |_| {
                                        threads.with_mut(|list| {
                                            if let Some(item) =
                                                list.iter_mut().find(|item| item.id == id)
                                            {
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

        header {
            class: "mb-8 max-w-3xl",
            h1 {
                class: "text-3xl font-semibold tracking-tight sm:text-4xl",
                "{thread.title}"
            }
            p { class: "mt-2 text-sm text-text-muted", "{meta}" }
        }

        article {
            class: "forum-post",
            Avatar {
                email: thread.author_email.clone(),
                size: 44,
                alt: thread.author.clone(),
            }
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "flex flex-wrap items-baseline gap-x-2 gap-y-1",
                    p { class: "text-sm font-semibold text-text", "{thread.author}" }
                    span { class: "text-xs text-text-muted", "Original post · {thread.when}" }
                }
                p {
                    class: "mt-3 text-[0.95rem] leading-relaxed text-text-secondary whitespace-pre-wrap",
                    "{thread.preview}"
                }
            }
        }

        section {
            class: "mt-2 mb-4 flex items-baseline justify-between gap-3 border-t border-border-subtle pt-6",
            p { class: "text-xs font-medium uppercase tracking-wide text-text-muted", "Replies" }
            span { class: "text-xs text-text-secondary", "{thread.replies}" }
        }

        if thread.locked {
            p {
                class: "mb-4 text-sm text-text-muted",
                "This thread is locked — new replies are disabled."
            }
        }

        if reply_samples.is_empty() {
            p {
                class: "py-8 text-sm text-text-muted",
                "No replies yet."
            }
        } else {
            for reply in reply_samples.into_iter() {
                article {
                    class: "forum-post",
                    Avatar {
                        email: reply.email.clone(),
                        size: 36,
                        alt: reply.author.clone(),
                    }
                    div {
                        class: "min-w-0 flex-1",
                        div {
                            class: "flex flex-wrap items-baseline gap-x-2 gap-y-1",
                            p { class: "text-sm font-semibold text-text", "{reply.author}" }
                            span { class: "text-xs text-text-muted", "{reply.when}" }
                        }
                        p {
                            class: "mt-2 text-sm leading-relaxed text-text-secondary",
                            "{reply.body}"
                        }
                    }
                }
            }
        }

        if !thread.locked {
            section {
                class: "mt-8 max-w-2xl border-t border-border-subtle pt-6",
                p { class: "mb-3 text-xs font-medium text-text-muted", "Reply" }
                textarea {
                    class: "ui-input ui-squircle min-h-28 w-full resize-y px-4 py-3 text-sm outline-none",
                    placeholder: "Write a reply…",
                }
                div {
                    class: "mt-3",
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
    let queue = format!("{} open", REPORTS.len());

    rsx! {
        PageHeader {
            title: "Moderation",
            subtitle: "Clear the queue. Configure the bot under Auto Moderation.",
            action: rsx! {
                span {
                    class: "rounded-squircle-sm border border-border-subtle bg-surface/40 px-3 py-2 text-xs text-text-secondary",
                    "{acting_as}"
                }
            },
        }

        section {
            class: "mb-10",
            div {
                class: "mb-4 flex items-baseline justify-between gap-3",
                p { class: "text-xs font-medium uppercase tracking-wide text-text-muted", "Report queue" }
                span {
                    class: "text-xs font-medium tabular-nums",
                    style: "color: {FORUM_ACCENT};",
                    "{queue}"
                }
            }
            for report in REPORTS.iter().copied() {
                ReportCard { report }
            }
        }
    }
}

#[component]
fn ReportCard(report: Report) -> Element {
    let meta = report.meta();
    let severity = report.severity;

    rsx! {
        article {
            class: "forum-report",
            style: "--report-tone: {severity.tone()};",
            div {
                class: "min-w-0 flex-1",
                div {
                    class: "flex flex-wrap items-center gap-2",
                    h3 { class: "text-sm font-semibold tracking-tight sm:text-base", "{report.title}" }
                    ToneChip { label: severity.label(), tone: severity.tone() }
                }
                p { class: "mt-1.5 text-sm leading-relaxed text-text-muted", "{report.detail}" }
                p { class: "mt-2 text-xs text-text-secondary", "{meta}" }
                div {
                    class: "mt-4 flex flex-wrap gap-2",
                    Button {
                        variant: ButtonVariant::Danger,
                        size: ButtonSize::Sm,
                        "Hide post"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "Warn user"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        "Dismiss"
                    }
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
        PageHeader {
            title: "Auto Moderation",
            subtitle: "Give the bot an identity, then tune the filters and actions it runs.",
            action: rsx! {
                Button { "Save changes" }
            },
        }

        section {
            class: "mb-6 flex flex-col gap-4 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4 sm:mb-8 sm:flex-row sm:items-center sm:justify-between sm:gap-6 sm:p-5",
            div {
                class: "min-w-0",
                p { class: "text-xs font-medium uppercase tracking-wide text-text-muted", "Bot status" }
                p { class: "mt-1 text-lg font-semibold tracking-tight", "Auto Mod is watching public boards" }
                p { class: "mt-0.5 text-sm text-text-muted", "Actions run instantly; staff still get a queue entry for high severity." }
            }
            div {
                class: "w-full shrink-0 sm:max-w-xs sm:border-l sm:border-border-subtle sm:pl-6",
                SettingRow {
                    title: "Enable Auto Mod",
                    description: "Turn the bot on or off across the forum.",
                    enabled: true,
                }
            }
        }

        div {
            class: "grid gap-4 sm:gap-5 lg:grid-cols-2",

            DataPanel {
                title: "Bot identity",
                div {
                    class: "space-y-4",
                    FormField {
                        label: "Display name",
                        hint: "Shown on every automated warning, mute, and hide notice.",
                        SignalInput {
                            value: bot_name,
                            placeholder: "ServerSpot AutoMod",
                        }
                    }
                    div {
                        class: "grid gap-4 sm:grid-cols-2",
                        FormField {
                            label: "Badge label",
                            hint: "Short tag next to the name.",
                            SignalInput {
                                value: bot_tag,
                                placeholder: "BOT",
                            }
                        }
                        FormField {
                            label: "Accent colour",
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

            DataPanel {
                title: "Preview",
                div {
                    class: "flex items-start gap-3",
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
                    div {
                        class: "min-w-0 flex-1",
                        div {
                            class: "flex flex-wrap items-center gap-2",
                            p { class: "text-sm font-semibold tracking-tight", "{preview_name}" }
                            span {
                                class: "inline-flex items-center rounded-squircle-sm px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide",
                                style: "background: color-mix(in srgb, {preview_accent} 18%, transparent); color: {preview_accent};",
                                "{preview_tag}"
                            }
                        }
                        p {
                            class: "mt-1.5 text-sm leading-relaxed text-text-secondary",
                            "Hey NovaCraft — your post was flagged by Auto Mod for spam links. Please edit or remove it."
                        }
                        p { class: "mt-2 text-xs text-text-muted", "just now · automated" }
                    }
                }
            }

            DataPanel {
                title: "Message templates",
                div {
                    class: "space-y-4",
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

            DataPanel {
                title: "Bot thresholds",
                div {
                    class: "space-y-4",
                    FormField {
                        label: "Blocked words",
                        hint: "Comma-separated. Matching posts are held for review.",
                        SignalTextarea {
                            value: blocked_words,
                            placeholder: "spam phrase, invite link…",
                        }
                    }
                    div {
                        class: "grid gap-4 sm:grid-cols-3",
                        FormField {
                            label: "Max links",
                            SignalInput {
                                value: max_links,
                                placeholder: "2",
                            }
                        }
                        FormField {
                            label: "Mute (min)",
                            hint: "When Mute is chosen.",
                            SignalInput {
                                value: mute_minutes,
                                placeholder: "30",
                            }
                        }
                        FormField {
                            label: "New acct (hrs)",
                            SignalInput {
                                value: new_account_hours,
                                placeholder: "24",
                            }
                        }
                    }
                }
            }

            DataPanel {
                title: "Filters",
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

            DataPanel {
                title: "Actions",
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

            div {
                class: "lg:col-span-2",
                DataPanel {
                    title: "Notifications",
                    div {
                        class: "grid gap-x-8 lg:grid-cols-2",
                        div {
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
                        }
                        div {
                            SettingRow {
                                title: "Sign DMs with bot identity",
                                description: "Use the bot name and avatar on private warnings instead of a generic system sender.",
                                enabled: true,
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
    let full_url = format!("www.example.com{}", public_path());

    rsx! {
        PageHeader {
            title: "Settings",
            subtitle: "Path and defaults for the forum on your main website.",
            action: rsx! {
                Button { "Save changes" }
            },
        }

        section {
            class: "mb-8 flex flex-col gap-3 rounded-squircle-lg border border-border-subtle bg-surface/20 p-4 sm:p-5",
            p { class: "text-xs font-medium uppercase tracking-wide text-text-muted", "On your website" }
            p { class: "font-mono text-sm text-text", "{full_url}" }
            p { class: "text-xs text-text-muted", "Domain and HTTPS are managed in Settings → General." }
        }

        section {
            class: "mb-8 max-w-xl space-y-4",
            FormField {
                label: "Public path",
                SignalInput {
                    value: public_path,
                    placeholder: "/forum",
                }
            }
            FormField {
                label: "Page title",
                SignalInput {
                    value: page_title,
                    placeholder: "Forums",
                }
            }
        }

        section {
            p { class: "mb-1 text-xs font-medium uppercase tracking-wide text-text-muted", "Community defaults" }
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
