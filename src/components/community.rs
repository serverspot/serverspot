use dioxus::prelude::*;

use crate::components::page::{PageHeader, StatusChip};
use crate::components::ui::*;
use crate::router::Route;

pub const PLAYERS_LEADERBOARDS_CSS: Asset = asset!("/css-partials/players-leaderboards.css");

#[component]
pub fn PlayersLeaderboardsStyles() -> Element {
    rsx! {
        document::Stylesheet { href: PLAYERS_LEADERBOARDS_CSS }
    }
}

pub const VOTES_APPLICATIONS_CSS: Asset = asset!("/css-partials/votes-applications.css");

#[component]
pub fn VotesApplicationsStyles() -> Element {
    rsx! {
        document::Stylesheet { href: VOTES_APPLICATIONS_CSS }
    }
}

pub const VOTES_ACCENT: &str = "#fbbf24";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerRank {
    Member,
    Vip,
    Elite,
    Moderator,
    Admin,
}

impl PlayerRank {
    pub fn label(self) -> &'static str {
        match self {
            PlayerRank::Member => "Member",
            PlayerRank::Vip => "VIP",
            PlayerRank::Elite => "Elite",
            PlayerRank::Moderator => "Moderator",
            PlayerRank::Admin => "Admin",
        }
    }

    pub fn tone(self) -> &'static str {
        match self {
            PlayerRank::Member => "#8a8f98",
            PlayerRank::Vip => "#fbbf24",
            PlayerRank::Elite => "#a78bfa",
            PlayerRank::Moderator => "#5b9dff",
            PlayerRank::Admin => "#fb7185",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    Online,
    Away,
    Offline,
}

impl PlayerStatus {
    pub fn label(self) -> &'static str {
        match self {
            PlayerStatus::Online => "Online",
            PlayerStatus::Away => "Away",
            PlayerStatus::Offline => "Offline",
        }
    }

    pub fn tone(self) -> &'static str {
        match self {
            PlayerStatus::Online => "#3ecf8e",
            PlayerStatus::Away => "#fbbf24",
            PlayerStatus::Offline => "#5c6070",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Player {
    pub id: u64,
    pub username: &'static str,
    pub email: &'static str,
    pub rank: PlayerRank,
    pub status: PlayerStatus,
    pub level: u32,
    pub playtime_hours: u32,
    pub votes: u32,
    pub kills: u32,
    pub balance: &'static str,
    pub last_seen: &'static str,
    pub joined: &'static str,
    pub bio: &'static str,
    pub mc_name: &'static str,
    pub discord: &'static str,
    pub verified: bool,
}

pub fn placeholder_players() -> Vec<Player> {
    vec![
        Player {
            id: 1,
            username: "NovaCraft",
            email: "novacraft@players.serverspot.app",
            rank: PlayerRank::Admin,
            status: PlayerStatus::Online,
            level: 84,
            playtime_hours: 1240,
            votes: 512,
            kills: 1902,
            balance: "£12,480",
            last_seen: "Now",
            joined: "Mar 2023",
            bio: "Server founder. Builds the spawn, breaks the bugs.",
            mc_name: "NovaCraft",
            discord: "nova#0001",
            verified: true,
        },
        Player {
            id: 2,
            username: "SkyBuilder",
            email: "skybuilder@players.serverspot.app",
            rank: PlayerRank::Moderator,
            status: PlayerStatus::Online,
            level: 71,
            playtime_hours: 940,
            votes: 388,
            kills: 640,
            balance: "£8,210",
            last_seen: "Now",
            joined: "Jun 2023",
            bio: "Redstone tinkerer and part-time moderator.",
            mc_name: "SkyBuilder",
            discord: "sky#4420",
            verified: true,
        },
        Player {
            id: 3,
            username: "RedstoneRex",
            email: "redstonerex@players.serverspot.app",
            rank: PlayerRank::Elite,
            status: PlayerStatus::Away,
            level: 66,
            playtime_hours: 812,
            votes: 274,
            kills: 1902,
            balance: "£5,940",
            last_seen: "12m ago",
            joined: "Aug 2023",
            bio: "PvP main. Chasing the kills leaderboard.",
            mc_name: "RedstoneRex",
            discord: "rex#9931",
            verified: false,
        },
        Player {
            id: 4,
            username: "AetherFox",
            email: "aetherfox@players.serverspot.app",
            rank: PlayerRank::Vip,
            status: PlayerStatus::Online,
            level: 58,
            playtime_hours: 604,
            votes: 128,
            kills: 410,
            balance: "£3,120",
            last_seen: "Now",
            joined: "Nov 2023",
            bio: "Vote streak champion three months running.",
            mc_name: "AetherFox",
            discord: "fox#1177",
            verified: true,
        },
        Player {
            id: 5,
            username: "PixelPaws",
            email: "pixelpaws@players.serverspot.app",
            rank: PlayerRank::Member,
            status: PlayerStatus::Offline,
            level: 41,
            playtime_hours: 288,
            votes: 62,
            kills: 96,
            balance: "£1,040",
            last_seen: "3h ago",
            joined: "Jan 2024",
            bio: "Casual survival player and shopkeeper.",
            mc_name: "PixelPaws",
            discord: "paws#2048",
            verified: false,
        },
        Player {
            id: 6,
            username: "EmberQueen",
            email: "emberqueen@players.serverspot.app",
            rank: PlayerRank::Elite,
            status: PlayerStatus::Away,
            level: 62,
            playtime_hours: 733,
            votes: 210,
            kills: 1288,
            balance: "£4,760",
            last_seen: "40m ago",
            joined: "Sep 2023",
            bio: "Nether explorer with a taste for netherite.",
            mc_name: "EmberQueen",
            discord: "ember#7781",
            verified: true,
        },
        Player {
            id: 7,
            username: "GlacierGuy",
            email: "glacierguy@players.serverspot.app",
            rank: PlayerRank::Member,
            status: PlayerStatus::Offline,
            level: 33,
            playtime_hours: 174,
            votes: 44,
            kills: 58,
            balance: "£680",
            last_seen: "1d ago",
            joined: "Feb 2024",
            bio: "Ice-biome builder, still learning the ropes.",
            mc_name: "GlacierGuy",
            discord: "glacier#3390",
            verified: false,
        },
        Player {
            id: 8,
            username: "MythicMara",
            email: "mythicmara@players.serverspot.app",
            rank: PlayerRank::Vip,
            status: PlayerStatus::Online,
            level: 55,
            playtime_hours: 512,
            votes: 176,
            kills: 502,
            balance: "£2,880",
            last_seen: "Now",
            joined: "Dec 2023",
            bio: "Loves events, hates creepers.",
            mc_name: "MythicMara",
            discord: "mara#6612",
            verified: true,
        },
    ]
}

const PLAYER_RANK_FILTERS: &[(&str, Option<PlayerRank>)] = &[
    ("All", None),
    ("Admins", Some(PlayerRank::Admin)),
    ("Moderators", Some(PlayerRank::Moderator)),
    ("Elite", Some(PlayerRank::Elite)),
    ("VIP", Some(PlayerRank::Vip)),
    ("Members", Some(PlayerRank::Member)),
];

#[component]
pub fn CommunityPlayers() -> Element {
    let navigator = use_navigator();
    let players = use_hook(placeholder_players);
    let query = use_signal(String::new);
    let filter = use_signal(|| 0usize);

    let visible = use_memo(move || {
        let needle = query().to_lowercase();
        let active_filter = PLAYER_RANK_FILTERS[filter()].1;
        players
            .iter()
            .filter(|p| active_filter.map(|r| r == p.rank).unwrap_or(true))
            .filter(|p| needle.is_empty() || p.username.to_lowercase().contains(&needle))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        PlayersLeaderboardsStyles {}
        PageHeader {
            title: "Player roster",
            subtitle: "The roster, ranks, and linked identities for every player across your servers.",
        }

        div { class: "pl-toolbar",
            div { class: "pl-toolbar-row",
                SearchInput {
                    value: query,
                    placeholder: "Search players…".to_string(),
                    class: "min-w-0 flex-1",
                }
            }
            div { class: "pl-toolbar-row",
                for (index, (label, _)) in PLAYER_RANK_FILTERS.iter().enumerate() {
                    {
                        let mut filter = filter;
                        let is_active = filter() == index;
                        rsx! {
                            button {
                                key: "{label}",
                                r#type: "button",
                                class: if is_active { "roster-chip is-active" } else { "roster-chip" },
                                onclick: move |_| filter.set(index),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }

        if visible().is_empty() {
            p { class: "text-sm text-text-muted", "No players match those filters." }
        } else {
            div { class: "motion-cascade roster-grid",
                for player in visible() {
                    {
                        let player_id = player.id;
                        rsx! {
                            button {
                                key: "{player.id}",
                                r#type: "button",
                                class: "roster-card",
                                onclick: move |_| {
                                    navigator
                                        .push(Route::PlayersProfileDetail {
                                            id: player_id,
                                        });
                                },
                                div { class: "roster-card-top",
                                    div { class: "roster-avatar-wrap",
                                        Avatar { email: player.email, size: 44, alt: player.username }
                                        span {
                                            class: "roster-status-dot",
                                            style: "--dot-color: {player.status.tone()};",
                                        }
                                    }
                                    div { class: "roster-card-identity",
                                        p { class: "roster-card-name", "{player.username}" }
                                        p { class: "roster-card-case", "Level {player.level} · {player.playtime_hours}h" }
                                    }
                                    span {
                                        class: "roster-rank-badge",
                                        style: "--badge-color: {player.rank.tone()};",
                                        "{player.rank.label()}"
                                    }
                                }
                                div { class: "roster-card-links",
                                    span { class: if player.verified { "roster-link-chip is-verified" } else { "roster-link-chip" },
                                        "{player.mc_name}"
                                    }
                                    span { class: "roster-link-chip", "{player.discord}" }
                                }
                                div { class: "roster-card-foot",
                                    span { "{player.votes} votes" }
                                    span { "{player.status.label()} · {player.last_seen}" }
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
pub fn PlayersProfileDetail(id: u64) -> Element {
    let navigator = use_navigator();
    let player = placeholder_players().into_iter().find(|p| p.id == id);

    let Some(player) = player else {
        return rsx! {
            PlayersLeaderboardsStyles {}
            button {
                r#type: "button",
                class: "pl-back",
                onclick: move |_| {
                    navigator.push(Route::CommunityPlayers {});
                },
                "← Roster"
            }
            h1 { class: "mt-4 text-2xl font-semibold tracking-tight", "Player not found" }
            p { class: "mt-2 text-sm text-text-muted", "This profile may have been removed." }
        };
    };

    rsx! {
        PlayersLeaderboardsStyles {}
        button {
            r#type: "button",
            class: "pl-back",
            onclick: move |_| {
                navigator.push(Route::CommunityPlayers {});
            },
            "← Roster"
        }

        div { class: "roster-file mt-4",
            div { class: "roster-file-head",
                div { class: "roster-file-avatar-ring",
                    Avatar { email: player.email, size: 72, alt: player.username }
                }
                div { class: "roster-file-identity",
                    div { class: "roster-file-name-row",
                        h1 { class: "roster-file-name", "{player.username}" }
                        span {
                            class: "roster-rank-badge",
                            style: "--badge-color: {player.rank.tone()};",
                            "{player.rank.label()}"
                        }
                    }
                    p { class: "roster-file-case-number",
                        "Case #{player.id:04} · joined {player.joined}"
                    }
                    p { class: "roster-file-bio", "{player.bio}" }
                    div { class: "roster-file-meta-row",
                        span {
                            class: "roster-file-stamp",
                            style: "--stamp-color: {player.status.tone()};",
                            "{player.status.label()} · {player.last_seen}"
                        }
                    }
                }
            }

            div { class: "motion-cascade roster-file-grid",
                div {
                    section { class: "roster-file-section",
                        p { class: "roster-file-section-title", "Linked accounts" }
                        div { class: "roster-file-linked-row",
                            span { class: "roster-file-linked-icon", "MC" }
                            div { class: "min-w-0 flex-1",
                                p { class: "text-sm font-medium text-text", "{player.mc_name}" }
                                p { class: "text-xs text-text-muted", "Minecraft · Java" }
                            }
                            if player.verified {
                                StatusChip { label: "Verified", tone: "#3ecf8e" }
                            } else {
                                StatusChip { label: "Unlinked", tone: "#8a8f98" }
                            }
                        }
                        div { class: "roster-file-linked-row",
                            span { class: "roster-file-linked-icon", "DC" }
                            div { class: "min-w-0 flex-1",
                                p { class: "text-sm font-medium text-text", "{player.discord}" }
                                p { class: "text-xs text-text-muted", "Discord" }
                            }
                        }
                    }
                }
                aside {
                    div { class: "roster-file-aside-card",
                        div { class: "roster-file-fact",
                            span { class: "roster-file-fact-label", "Level" }
                            span { class: "roster-file-fact-value", "{player.level}" }
                        }
                        div { class: "roster-file-fact",
                            span { class: "roster-file-fact-label", "Playtime" }
                            span { class: "roster-file-fact-value", "{player.playtime_hours}h" }
                        }
                        div { class: "roster-file-fact",
                            span { class: "roster-file-fact-label", "Votes" }
                            span { class: "roster-file-fact-value", "{player.votes}" }
                        }
                        div { class: "roster-file-fact",
                            span { class: "roster-file-fact-label", "Kills" }
                            span { class: "roster-file-fact-value", "{player.kills}" }
                        }
                        div { class: "roster-file-fact",
                            span { class: "roster-file-fact-label", "Balance" }
                            span { class: "roster-file-fact-value", "{player.balance}" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardStat {
    Playtime,
    Kills,
    Votes,
    Balance,
    Blocks,
}

impl BoardStat {
    pub fn label(self) -> &'static str {
        match self {
            BoardStat::Playtime => "Playtime",
            BoardStat::Kills => "Kills",
            BoardStat::Votes => "Votes",
            BoardStat::Balance => "Balance",
            BoardStat::Blocks => "Blocks placed",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardReset {
    Never,
    Daily,
    Weekly,
    Monthly,
    Seasonal,
}

impl BoardReset {
    pub fn label(self) -> &'static str {
        match self {
            BoardReset::Never => "Never",
            BoardReset::Daily => "Daily",
            BoardReset::Weekly => "Weekly",
            BoardReset::Monthly => "Monthly",
            BoardReset::Seasonal => "Seasonal",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardSource {
    Plugin,
    Manual,
    Api,
}

impl BoardSource {
    pub fn label(self) -> &'static str {
        match self {
            BoardSource::Plugin => "In-game plugin",
            BoardSource::Manual => "Manual",
            BoardSource::Api => "API",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    Up,
    Down,
    Same,
}

#[derive(Clone, Copy, PartialEq)]
pub struct BoardEntry {
    pub rank: u32,
    pub name: &'static str,
    pub email: &'static str,
    pub value: &'static str,
    pub trend: Trend,
}

#[derive(Clone, PartialEq)]
pub struct RankReward {
    pub place: u32,
    pub summary: String,
}

#[derive(Clone, PartialEq)]
pub struct LeaderboardBoard {
    pub id: u64,
    pub name: String,
    pub stat: BoardStat,
    pub reset: BoardReset,
    pub source: BoardSource,
    pub accent: String,
    pub entries: Vec<BoardEntry>,
    pub rewards: Vec<RankReward>,
}

pub const BOARD_DEFAULT_ACCENT: &str = "#5eead4";

pub const BOARD_ACCENT_PRESETS: &[&str] = &[
    "#5eead4", "#69bdf2", "#a78bfa", "#f0a35e", "#f071a5", "#3ecf8e", "#fbbf24", "#fb7185",
];

pub fn next_board_accent(existing: &[LeaderboardBoard]) -> String {
    BOARD_ACCENT_PRESETS
        .iter()
        .find(|preset| {
            !existing
                .iter()
                .any(|b| b.accent.eq_ignore_ascii_case(preset))
        })
        .copied()
        .unwrap_or(BOARD_DEFAULT_ACCENT)
        .to_string()
}

fn entry(
    rank: u32,
    name: &'static str,
    email: &'static str,
    value: &'static str,
    trend: Trend,
) -> BoardEntry {
    BoardEntry {
        rank,
        name,
        email,
        value,
        trend,
    }
}

fn rank_reward(place: u32, summary: &str) -> RankReward {
    RankReward {
        place,
        summary: summary.into(),
    }
}

pub fn placeholder_leaderboard_boards() -> Vec<LeaderboardBoard> {
    vec![
        LeaderboardBoard {
            id: 1,
            name: "Top playtime".into(),
            stat: BoardStat::Playtime,
            reset: BoardReset::Never,
            source: BoardSource::Plugin,
            accent: "#5eead4".into(),
            entries: vec![
                entry(
                    1,
                    "NovaCraft",
                    "novacraft@players.serverspot.app",
                    "1,240h",
                    Trend::Same,
                ),
                entry(
                    2,
                    "SkyBuilder",
                    "skybuilder@players.serverspot.app",
                    "940h",
                    Trend::Up,
                ),
                entry(
                    3,
                    "RedstoneRex",
                    "redstonerex@players.serverspot.app",
                    "812h",
                    Trend::Down,
                ),
                entry(
                    4,
                    "EmberQueen",
                    "emberqueen@players.serverspot.app",
                    "733h",
                    Trend::Up,
                ),
                entry(
                    5,
                    "AetherFox",
                    "aetherfox@players.serverspot.app",
                    "604h",
                    Trend::Same,
                ),
            ],
            rewards: vec![
                rank_reward(1, "1× Legendary crate key"),
                rank_reward(2, "1× Rare crate key"),
                rank_reward(3, "500 credits"),
            ],
        },
        LeaderboardBoard {
            id: 2,
            name: "Most kills".into(),
            stat: BoardStat::Kills,
            reset: BoardReset::Monthly,
            source: BoardSource::Plugin,
            accent: "#fb7185".into(),
            entries: vec![
                entry(
                    1,
                    "RedstoneRex",
                    "redstonerex@players.serverspot.app",
                    "1,902",
                    Trend::Up,
                ),
                entry(
                    2,
                    "EmberQueen",
                    "emberqueen@players.serverspot.app",
                    "1,288",
                    Trend::Same,
                ),
                entry(
                    3,
                    "NovaCraft",
                    "novacraft@players.serverspot.app",
                    "1,040",
                    Trend::Down,
                ),
                entry(
                    4,
                    "SkyBuilder",
                    "skybuilder@players.serverspot.app",
                    "640",
                    Trend::Up,
                ),
            ],
            rewards: vec![
                rank_reward(1, "Warrior title + 2× Rare keys"),
                rank_reward(2, "1× Rare crate key"),
            ],
        },
        LeaderboardBoard {
            id: 3,
            name: "Vote champions".into(),
            stat: BoardStat::Votes,
            reset: BoardReset::Monthly,
            source: BoardSource::Api,
            accent: "#a78bfa".into(),
            entries: vec![
                entry(
                    1,
                    "NovaCraft",
                    "novacraft@players.serverspot.app",
                    "512",
                    Trend::Same,
                ),
                entry(
                    2,
                    "SkyBuilder",
                    "skybuilder@players.serverspot.app",
                    "388",
                    Trend::Up,
                ),
                entry(
                    3,
                    "EmberQueen",
                    "emberqueen@players.serverspot.app",
                    "210",
                    Trend::Up,
                ),
            ],
            rewards: vec![rank_reward(1, "Vote champion title")],
        },
        LeaderboardBoard {
            id: 4,
            name: "Richest players".into(),
            stat: BoardStat::Balance,
            reset: BoardReset::Never,
            source: BoardSource::Plugin,
            accent: "#fbbf24".into(),
            entries: vec![
                entry(
                    1,
                    "NovaCraft",
                    "novacraft@players.serverspot.app",
                    "£12,480",
                    Trend::Same,
                ),
                entry(
                    2,
                    "SkyBuilder",
                    "skybuilder@players.serverspot.app",
                    "£8,210",
                    Trend::Same,
                ),
                entry(
                    3,
                    "RedstoneRex",
                    "redstonerex@players.serverspot.app",
                    "£5,940",
                    Trend::Up,
                ),
            ],
            rewards: vec![],
        },
    ]
}

impl Trend {
    fn label(self) -> &'static str {
        match self {
            Trend::Up => "▲",
            Trend::Down => "▼",
            Trend::Same => "·",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Trend::Up => "board-standing-trend is-up",
            Trend::Down => "board-standing-trend is-down",
            Trend::Same => "board-standing-trend is-same",
        }
    }
}

#[component]
pub fn BoardPodium(entries: Vec<BoardEntry>) -> Element {
    let first = entries.iter().find(|e| e.rank == 1).copied();
    let second = entries.iter().find(|e| e.rank == 2).copied();
    let third = entries.iter().find(|e| e.rank == 3).copied();

    if first.is_none() && second.is_none() && third.is_none() {
        return rsx! {
            div { class: "board-podium-empty", "No standings yet — sync a source or seed entries." }
        };
    }

    rsx! {
        div { class: "board-podium",
            {
                let slot = second;
                rsx! {
                    div { class: "board-podium-slot is-second",
                        if let Some(e) = slot {
                            div { class: "board-podium-avatar",
                                Avatar { email: e.email, size: 40, alt: e.name }
                                span { class: "board-podium-medal is-silver", "2" }
                            }
                            span { class: "board-podium-name", "{e.name}" }
                            span { class: "board-podium-value", "{e.value}" }
                        } else {
                            span { class: "board-podium-name", "—" }
                            span { class: "board-podium-value", " " }
                        }
                        div { class: "board-podium-plinth" }
                    }
                }
            }
            {
                let slot = first;
                rsx! {
                    div { class: "board-podium-slot is-first",
                        if let Some(e) = slot {
                            div { class: "board-podium-avatar",
                                Avatar { email: e.email, size: 52, alt: e.name }
                                span { class: "board-podium-medal is-gold", "1" }
                            }
                            span { class: "board-podium-name", "{e.name}" }
                            span { class: "board-podium-value", "{e.value}" }
                        } else {
                            span { class: "board-podium-name", "—" }
                            span { class: "board-podium-value", " " }
                        }
                        div { class: "board-podium-plinth" }
                    }
                }
            }
            {
                let slot = third;
                rsx! {
                    div { class: "board-podium-slot is-third",
                        if let Some(e) = slot {
                            div { class: "board-podium-avatar",
                                Avatar { email: e.email, size: 36, alt: e.name }
                                span { class: "board-podium-medal is-bronze", "3" }
                            }
                            span { class: "board-podium-name", "{e.name}" }
                            span { class: "board-podium-value", "{e.value}" }
                        } else {
                            span { class: "board-podium-name", "—" }
                            span { class: "board-podium-value", " " }
                        }
                        div { class: "board-podium-plinth" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BoardStandings(entries: Vec<BoardEntry>, #[props(default = 0)] skip_top: usize) -> Element {
    let rows: Vec<BoardEntry> = entries.into_iter().skip(skip_top).collect();
    if rows.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "board-standings",
            for e in rows {
                div { key: "{e.rank}-{e.name}", class: "board-standing",
                    span { class: "board-standing-rank", "#{e.rank}" }
                    Avatar { email: e.email, size: 28, alt: e.name }
                    span { class: "board-standing-name", "{e.name}" }
                    span { class: e.trend.class(), "{e.trend.label()}" }
                    span { class: "board-standing-value", "{e.value}" }
                }
            }
        }
    }
}

#[component]
pub fn CommunityLeaderboards() -> Element {
    let navigator = use_navigator();
    let mut boards = use_context::<Signal<Vec<LeaderboardBoard>>>();
    let list = use_memo(move || boards());
    let mut expanded = use_signal(|| None::<u64>);

    rsx! {
        PlayersLeaderboardsStyles {}
        PageHeader {
            title: "Boards",
            subtitle: "Standings, podiums, and rank rewards — create and manage every leaderboard.",
            action: rsx! {
                Button {
                    onclick: move |_| {
                        navigator.push(Route::LeaderboardsBoardNew {});
                    },
                    IconPlus {}
                    "New board"
                }
            },
        }

        if list().is_empty() {
            p { class: "text-sm text-text-muted", "No boards yet. Create one to get started." }
        } else {
            div { class: "motion-cascade board-card-grid",
                for board in list() {
                    {
                        let board_id = board.id;
                        let is_open = expanded() == Some(board_id);
                        let top: Vec<BoardEntry> = board.entries.iter().copied().take(3).collect();
                        let rest: Vec<BoardEntry> = if is_open {
                            board.entries.iter().copied().skip(3).collect()
                        } else {
                            board.entries.iter().copied().skip(3).take(2).collect()
                        };
                        let reward_preview = board
                            .rewards
                            .first()
                            .map(|r| {
                                format!("#{place} · {summary}", place = r.place, summary = r.summary)
                            })
                            .unwrap_or_else(|| "No rank rewards yet".into());
                        rsx! {
                            article {
                                key: "{board.id}",
                                class: "board-card",
                                style: "--board-accent: {board.accent}",
                                div { class: "board-card-head",
                                    div { class: "min-w-0",
                                        h2 { class: "board-card-title", "{board.name}" }
                                        p { class: "board-card-meta", "{board.entries.len()} ranked · {board.source.label()}" }
                                    }
                                    div { class: "board-card-chips",
                                        span { class: "board-chip is-accent", "{board.stat.label()}" }
                                        span { class: "board-chip", "{board.reset.label()}" }
                                    }
                                }

                                BoardPodium { entries: top }
                                BoardStandings { entries: rest.clone(), skip_top: 0 }

                                if !is_open && board.entries.len() > 5 {
                                    p { class: "text-xs text-text-muted px-1",
                                        "+{board.entries.len().saturating_sub(5)} more in full standings"
                                    }
                                }

                                div { class: "board-card-foot",
                                    div { class: "board-card-reward",
                                        strong { "{board.rewards.len()}" }
                                        span { "{reward_preview}" }
                                    }
                                    div { class: "board-card-actions",
                                        button {
                                            r#type: "button",
                                            class: "board-card-btn",
                                            onclick: move |_| {
                                                expanded.with_mut(|cur| {
                                                    *cur = if *cur == Some(board_id) { None } else { Some(board_id) };
                                                });
                                            },
                                            if is_open {
                                                "Collapse"
                                            } else {
                                                "Standings"
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "board-card-btn is-primary",
                                            onclick: move |_| {
                                                navigator
                                                    .push(Route::LeaderboardsBoardEdit {
                                                        id: board_id,
                                                    });
                                            },
                                            "Edit"
                                        }
                                        button {
                                            r#type: "button",
                                            class: "board-card-btn is-danger",
                                            onclick: move |_| {
                                                boards.write().retain(|b| b.id != board_id);
                                                if expanded() == Some(board_id) {
                                                    expanded.set(None);
                                                }
                                            },
                                            "Delete"
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
pub fn LeaderboardsBoardNew() -> Element {
    rsx! {
        BoardEditor { board_id: None }
    }
}

#[component]
pub fn LeaderboardsBoardEdit(id: u64) -> Element {
    rsx! {
        BoardEditor { board_id: Some(id) }
    }
}

#[component]
fn BoardEditor(board_id: Option<u64>) -> Element {
    let navigator = use_navigator();
    let mut boards = use_context::<Signal<Vec<LeaderboardBoard>>>();
    let existing = board_id.and_then(|id| boards.read().iter().find(|b| b.id == id).cloned());

    let name = use_signal(|| {
        existing
            .as_ref()
            .map(|b| b.name.clone())
            .unwrap_or_default()
    });
    let stat = use_signal(|| {
        existing
            .as_ref()
            .map(|b| b.stat.label().to_string())
            .unwrap_or_else(|| BoardStat::Playtime.label().to_string())
    });
    let reset = use_signal(|| {
        existing
            .as_ref()
            .map(|b| b.reset.label().to_string())
            .unwrap_or_else(|| BoardReset::Never.label().to_string())
    });
    let source = use_signal(|| {
        existing
            .as_ref()
            .map(|b| b.source.label().to_string())
            .unwrap_or_else(|| BoardSource::Plugin.label().to_string())
    });
    let accent = use_signal(|| match existing.as_ref() {
        Some(b) => b.accent.clone(),
        None => next_board_accent(&boards.peek()),
    });
    let mut rewards = use_signal(|| {
        existing
            .as_ref()
            .map(|b| b.rewards.clone())
            .unwrap_or_else(|| {
                vec![RankReward {
                    place: 1,
                    summary: String::new(),
                }]
            })
    });

    let heading = if existing.is_some() {
        "Edit board"
    } else {
        "New board"
    };
    let is_edit = existing.is_some();

    rsx! {
        PlayersLeaderboardsStyles {}
        button {
            r#type: "button",
            class: "pl-back",
            onclick: move |_| {
                navigator.push(Route::CommunityLeaderboards {});
            },
            "← Boards"
        }

        div { class: "mb-6 mt-4",
            p { class: "board-console-eyebrow", "Leaderboard admin" }
            h1 { class: "mt-1 text-2xl font-semibold tracking-tight text-text", "{heading}" }
        }

        div { class: "board-editor-layout",
            div { class: "motion-cascade board-editor-main",
                section { class: "board-editor-section",
                    h2 { class: "board-editor-heading", "Board basics" }
                    p { class: "board-editor-lede",
                        "The name and the stat this board ranks players by."
                    }
                    LabeledField { label: "Board name",
                        SignalInput {
                            value: name,
                            placeholder: "Top playtime".to_string(),
                        }
                    }
                    LabeledField { label: "Ranked stat",
                        SignalSelect { value: stat, options: board_stat_options() }
                    }
                }
                section { class: "board-editor-section",
                    h2 { class: "board-editor-heading", "Reset & source" }
                    p { class: "board-editor-lede",
                        "How often the board clears and where its numbers come from."
                    }
                    LabeledField { label: "Reset schedule",
                        SignalSelect { value: reset, options: board_reset_options() }
                    }
                    LabeledField { label: "Data source",
                        SignalSelect { value: source, options: board_source_options() }
                    }
                    LabeledField { label: "Board colour",
                        ColorPicker {
                            value: accent,
                            presets: BOARD_ACCENT_PRESETS
                                                        .iter()
                                                        .map(|c| c.to_string())
                                                        .collect::<Vec<_>>(),
                        }
                    }
                }
                section { class: "board-editor-section",
                    div { class: "mb-3 flex flex-wrap items-center justify-between gap-2",
                        div { class: "min-w-0",
                            h2 { class: "board-editor-heading", "Rank rewards" }
                            p { class: "board-editor-lede",
                                "Payouts for finishing places when the board resets (or on demand)."
                            }
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                let next_place = rewards
                                    .read()
                                    .iter()
                                    .map(|r| r.place)
                                    .max()
                                    .unwrap_or(0)
                                    + 1;
                                rewards
                                    .write()
                                    .push(RankReward {
                                        place: next_place,
                                        summary: String::new(),
                                    });
                            },
                            IconPlus {}
                            "Add place"
                        }
                    }
                    if rewards.read().is_empty() {
                        p { class: "text-sm text-text-muted",
                            "No rank rewards yet. Add a place to start."
                        }
                    } else {
                        div { class: "motion-cascade motion-cascade-tight board-reward-list",
                            div { class: "board-reward-row is-head",
                                span { "Place" }
                                span { "Reward" }
                                span { "" }
                            }
                            for i in 0..rewards.read().len() {
                                {
                                    let idx = i;
                                    let place_value = rewards
                                        .read()
                                        .get(idx)
                                        .map(|reward| reward.place.to_string())
                                        .unwrap_or_else(|| "1".into());
                                    let summary_value = rewards
                                        .read()
                                        .get(idx)
                                        .map(|reward| reward.summary.clone())
                                        .unwrap_or_default();
                                    rsx! {
                                        div { key: "reward-{idx}", class: "board-reward-row",
                                            input {
                                                r#type: "number",
                                                min: "1",
                                                class: "ui-input ui-squircle h-10 w-full max-w-[5.5rem] px-3 text-sm outline-none",
                                                value: "{place_value}",
                                                oninput: move |evt: FormEvent| {
                                                    let parsed = evt.value().parse::<u32>().unwrap_or(1);
                                                    if let Some(row) = rewards.write().get_mut(idx) {
                                                        row.place = parsed.max(1);
                                                    }
                                                },
                                            }
                                            input {
                                                r#type: "text",
                                                class: "ui-input ui-squircle h-10 w-full px-4 text-sm outline-none",
                                                value: "{summary_value}",
                                                placeholder: "1× Legendary crate key",
                                                oninput: move |evt: FormEvent| {
                                                    if let Some(row) = rewards.write().get_mut(idx) {
                                                        row.summary = evt.value();
                                                    }
                                                },
                                            }
                                            button {
                                                r#type: "button",
                                                class: "board-console-link is-danger board-reward-remove",
                                                onclick: move |_| {
                                                    if idx < rewards.read().len() {
                                                        rewards.write().remove(idx);
                                                    }
                                                },
                                                "Remove"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "mt-2 flex flex-wrap items-center gap-2",
                    Button {
                        onclick: move |_| {
                            let resolved_name = {
                                let v = name().trim().to_string();
                                if v.is_empty() { "Untitled board".into() } else { v }
                            };
                            let resolved_stat = parse_board_stat(&stat());
                            let resolved_reset = parse_board_reset(&reset());
                            let resolved_source = parse_board_source(&source());
                            let resolved_rewards = rewards();
                            let resolved_accent = {
                                let v = accent().trim().to_string();
                                if v.is_empty() { BOARD_DEFAULT_ACCENT.to_string() } else { v }
                            };
                            if let Some(id) = board_id {
                                boards
                                    .with_mut(|list| {
                                        if let Some(board) = list.iter_mut().find(|b| b.id == id) {
                                            board.name = resolved_name;
                                            board.stat = resolved_stat;
                                            board.reset = resolved_reset;
                                            board.source = resolved_source;
                                            board.accent = resolved_accent;
                                            board.rewards = resolved_rewards;
                                        }
                                    });
                            } else {
                                let next_id = boards.read().iter().map(|b| b.id).max().unwrap_or(0) + 1;
                                boards
                                    .write()
                                    .push(LeaderboardBoard {
                                        id: next_id,
                                        name: resolved_name,
                                        stat: resolved_stat,
                                        reset: resolved_reset,
                                        source: resolved_source,
                                        accent: resolved_accent,
                                        entries: vec![],
                                        rewards: resolved_rewards,
                                    });
                            }
                            navigator.push(Route::CommunityLeaderboards {});
                        },
                        "Save board"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            navigator.push(Route::CommunityLeaderboards {});
                        },
                        "Cancel"
                    }
                    if is_edit {
                        if let Some(id) = board_id {
                            Button {
                                variant: ButtonVariant::Danger,
                                onclick: move |_| {
                                    boards.write().retain(|b| b.id != id);
                                    navigator.push(Route::CommunityLeaderboards {});
                                },
                                "Delete board"
                            }
                        }
                    }
                }
            }

            aside { class: "board-editor-aside",
                div { class: "board-editor-summary",
                    p { class: "pl-eyebrow", "Board summary" }
                    p { class: "mt-2 text-lg font-semibold text-text",
                        if name().trim().is_empty() {
                            "Untitled board"
                        } else {
                            "{name}"
                        }
                    }
                    dl { class: "board-summary-dl",
                        div {
                            dt { "Ranks" }
                            dd { "{stat}" }
                        }
                        div {
                            dt { "Resets" }
                            dd { "{reset}" }
                        }
                        div {
                            dt { "Source" }
                            dd { "{source}" }
                        }
                        div {
                            dt { "Rank rewards" }
                            dd { "{rewards.read().len()}" }
                        }
                    }
                    p { class: "board-editor-aside-note",
                        "Standings are pulled from the data source; rank rewards pay out on reset."
                    }
                }
            }
        }
    }
}

fn parse_board_stat(label: &str) -> BoardStat {
    match label {
        "Kills" => BoardStat::Kills,
        "Votes" => BoardStat::Votes,
        "Balance" => BoardStat::Balance,
        "Blocks placed" => BoardStat::Blocks,
        _ => BoardStat::Playtime,
    }
}

fn parse_board_reset(label: &str) -> BoardReset {
    match label {
        "Daily" => BoardReset::Daily,
        "Weekly" => BoardReset::Weekly,
        "Monthly" => BoardReset::Monthly,
        "Seasonal" => BoardReset::Seasonal,
        _ => BoardReset::Never,
    }
}

fn parse_board_source(label: &str) -> BoardSource {
    match label {
        "Manual" => BoardSource::Manual,
        "API" => BoardSource::Api,
        _ => BoardSource::Plugin,
    }
}

fn board_stat_options() -> Vec<SelectOption> {
    ["Playtime", "Kills", "Votes", "Balance", "Blocks placed"]
        .iter()
        .map(|s| SelectOption::new(*s, *s))
        .collect()
}

fn board_reset_options() -> Vec<SelectOption> {
    ["Never", "Daily", "Weekly", "Monthly", "Seasonal"]
        .iter()
        .map(|s| SelectOption::new(*s, *s))
        .collect()
}

fn board_source_options() -> Vec<SelectOption> {
    ["In-game plugin", "Manual", "API"]
        .iter()
        .map(|s| SelectOption::new(*s, *s))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VoteSiteStatus {
    Live,
    Degraded,
    Offline,
}

pub(crate) fn vote_site_status_label(status: VoteSiteStatus) -> &'static str {
    match status {
        VoteSiteStatus::Live => "Live",
        VoteSiteStatus::Degraded => "Degraded",
        VoteSiteStatus::Offline => "Offline",
    }
}

pub(crate) fn vote_site_status_class(status: VoteSiteStatus) -> &'static str {
    match status {
        VoteSiteStatus::Live => "vote-status is-live",
        VoteSiteStatus::Degraded => "vote-status is-degraded",
        VoteSiteStatus::Offline => "vote-status is-offline",
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct VoteSite {
    pub id: &'static str,
    pub name: &'static str,
    pub status: VoteSiteStatus,
    pub votes_today: u32,
    pub cooldown: &'static str,
    pub last_callback: &'static str,
}

pub(crate) fn placeholder_vote_sites() -> Vec<VoteSite> {
    vec![
        VoteSite {
            id: "minecraft-mp",
            name: "Minecraft-MP",
            status: VoteSiteStatus::Live,
            votes_today: 142,
            cooldown: "24h",
            last_callback: "2m ago",
        },
        VoteSite {
            id: "topg",
            name: "TopG",
            status: VoteSiteStatus::Live,
            votes_today: 89,
            cooldown: "24h",
            last_callback: "8m ago",
        },
        VoteSite {
            id: "planet-minecraft",
            name: "Planet Minecraft",
            status: VoteSiteStatus::Degraded,
            votes_today: 31,
            cooldown: "12h",
            last_callback: "41m ago",
        },
        VoteSite {
            id: "serverlist",
            name: "ServerList",
            status: VoteSiteStatus::Offline,
            votes_today: 0,
            cooldown: "24h",
            last_callback: "6h ago",
        },
    ]
}

#[derive(Clone, PartialEq)]
pub struct VoteReward {
    pub id: u64,
    pub name: String,
    pub trigger_kind: String,
    pub trigger_detail: String,
    pub reward_summary: String,
    pub command: String,
    pub site_scope: String,
    pub cooldown: String,
    pub accent: String,
    pub active: bool,
    pub claims: u32,
    pub note: String,
}

impl VoteReward {
    pub fn blank() -> Self {
        Self {
            id: 0,
            name: String::new(),
            trigger_kind: String::from("Every vote"),
            trigger_detail: String::new(),
            reward_summary: String::new(),
            command: String::new(),
            site_scope: String::from("All connected sites"),
            cooldown: String::from("24h"),
            accent: String::from(VOTES_ACCENT),
            active: true,
            claims: 0,
            note: String::new(),
        }
    }

    pub fn meta_line(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push(self.trigger_kind.clone());
        if !self.reward_summary.trim().is_empty() {
            parts.push(self.reward_summary.clone());
        }
        if !self.site_scope.trim().is_empty() {
            parts.push(self.site_scope.clone());
        }
        if !self.cooldown.trim().is_empty() {
            parts.push(format!("cooldown {}", self.cooldown));
        }
        parts.join(" · ")
    }
}

pub fn placeholder_vote_rewards() -> Vec<VoteReward> {
    vec![
        VoteReward {
            id: 1,
            name: String::from("Daily Vote Crate"),
            trigger_kind: String::from("Every vote"),
            trigger_detail: String::from("Each successful vote"),
            reward_summary: String::from("1x Vote Key + 250 coins"),
            command: String::from("crate give {player} vote 1"),
            site_scope: String::from("All connected sites"),
            cooldown: String::from("24h"),
            accent: String::from(VOTES_ACCENT),
            active: true,
            claims: 1842,
            note: String::from("Most claimed reward"),
        },
        VoteReward {
            id: 2,
            name: String::from("7-Day Streak Chest"),
            trigger_kind: String::from("Streak"),
            trigger_detail: String::from("Vote 7 days in a row"),
            reward_summary: String::from("Legendary crate + title"),
            command: String::from("crate give {player} streak7 1"),
            site_scope: String::from("All connected sites"),
            cooldown: String::from("Weekly"),
            accent: String::from(VOTES_ACCENT),
            active: true,
            claims: 214,
            note: String::from("Resets if a day is missed"),
        },
        VoteReward {
            id: 3,
            name: String::from("100 Vote Milestone"),
            trigger_kind: String::from("Milestone"),
            trigger_detail: String::from("100 lifetime votes"),
            reward_summary: String::from("Voter rank + 5,000 coins"),
            command: String::from("lp user {player} parent add voter"),
            site_scope: String::from("All connected sites"),
            cooldown: String::from("Once"),
            accent: String::from(VOTES_ACCENT),
            active: true,
            claims: 58,
            note: String::new(),
        },
        VoteReward {
            id: 4,
            name: String::from("Weekend Double Coins"),
            trigger_kind: String::from("Every vote"),
            trigger_detail: String::from("Fri–Sun only"),
            reward_summary: String::from("2x coin reward"),
            command: String::from("eco multiply {player} 2"),
            site_scope: String::from("Minecraft-MP, TopG"),
            cooldown: String::from("24h"),
            accent: String::from(VOTES_ACCENT),
            active: false,
            claims: 96,
            note: String::from("Paused — event ended"),
        },
        VoteReward {
            id: 5,
            name: String::from("Monthly Top Voter Draw"),
            trigger_kind: String::from("Monthly draw"),
            trigger_detail: String::from("Top 3 voters"),
            reward_summary: String::from("Custom rank + store credit"),
            command: String::from("Manual — staff delivered"),
            site_scope: String::from("All connected sites"),
            cooldown: String::from("Monthly"),
            accent: String::from(VOTES_ACCENT),
            active: true,
            claims: 3,
            note: String::from("Drawn on the 1st"),
        },
    ]
}

#[component]
pub fn CommunityVotes() -> Element {
    let navigator = use_navigator();
    let rewards = use_context::<Signal<Vec<VoteReward>>>();
    let list = rewards();
    let enabled = list.iter().filter(|r| r.active).count();
    let sites = placeholder_vote_sites();
    let live = sites
        .iter()
        .filter(|s| s.status == VoteSiteStatus::Live)
        .count();

    rsx! {
        VotesApplicationsStyles {}
        div { class: "vote-console-page",
            PageHeader {
                title: "Rewards",
                subtitle: "What players receive after a verified vote. Listing sites and callbacks live on Overview and Settings.",
                action: rsx! {
                    Button {
                        onclick: move |_| {
                            navigator.push(Route::VoteRewardNew {});
                        },
                        IconPlus {}
                        "New reward"
                    }
                },
            }

            div { class: "vote-summary-bar",
                span { "{live} of {sites.len()} listing sites live" }
                span { class: "vote-summary-sep", "·" }
                span { "{enabled} of {list.len()} rewards enabled" }
            }

            if list.is_empty() {
                div { class: "vote-empty",
                    p { class: "text-sm font-medium text-text", "No rewards yet" }
                    p { class: "mt-1 text-sm text-text-muted",
                        "Create a reward rule, then map it to the listing sites that should pay it out."
                    }
                    Button {
                        class: "mt-4",
                        onclick: move |_| {
                            navigator.push(Route::VoteRewardNew {});
                        },
                        IconPlus {}
                        "New reward"
                    }
                }
            } else {
                div {
                    class: "motion-cascade motion-cascade-tight vote-reward-list",
                    role: "list",
                    for reward in list {
                        {
                            let reward_id = reward.id;
                            let meta = reward.meta_line();
                            rsx! {
                                div { key: "{reward.id}", class: "vote-reward-row", role: "listitem",
                                    div { class: "vote-reward-main",
                                        div { class: "vote-reward-top",
                                            p { class: "vote-reward-name", "{reward.name}" }
                                            span { class: if reward.active { "vote-pill is-on" } else { "vote-pill is-off" },
                                                if reward.active {
                                                    "Enabled"
                                                } else {
                                                    "Paused"
                                                }
                                            }
                                        }
                                        p { class: "vote-reward-meta", "{meta}" }
                                        if !reward.note.trim().is_empty() {
                                            p { class: "vote-reward-note", "{reward.note}" }
                                        }
                                    }
                                    span { class: "vote-reward-claims", "{reward.claims} claims" }
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            navigator
                                                .push(Route::VoteRewardEdit {
                                                    id: reward_id,
                                                });
                                        },
                                        "Edit"
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
pub fn VoteRewardNew() -> Element {
    rsx! {
        RewardEditor { reward_id: None }
    }
}

#[component]
pub fn VoteRewardEdit(id: u64) -> Element {
    rsx! {
        RewardEditor { reward_id: Some(id) }
    }
}

fn vote_site_scope_options() -> Vec<SelectOption> {
    [
        "All connected sites",
        "Minecraft-MP",
        "TopG",
        "Planet Minecraft",
        "ServerList",
        "Minecraft-MP, TopG",
    ]
    .iter()
    .map(|s| SelectOption::new(*s, *s))
    .collect()
}

fn vote_trigger_options() -> Vec<SelectOption> {
    ["Every vote", "Streak", "Milestone", "Monthly draw"]
        .iter()
        .map(|s| SelectOption::new(*s, *s))
        .collect()
}

fn vote_cooldown_options() -> Vec<SelectOption> {
    ["None", "12h", "24h", "Weekly", "Monthly", "Once"]
        .iter()
        .map(|s| SelectOption::new(*s, *s))
        .collect()
}

#[component]
fn RewardEditor(reward_id: Option<u64>) -> Element {
    let navigator = use_navigator();
    let mut rewards = use_context::<Signal<Vec<VoteReward>>>();
    let existing = reward_id.and_then(|id| rewards.read().iter().find(|r| r.id == id).cloned());
    let missing = reward_id.is_some() && existing.is_none();
    let seed = existing.clone().unwrap_or_else(VoteReward::blank);
    let claims = seed.claims;

    let mut name = use_signal(|| seed.name.clone());
    let trigger_kind = use_signal(|| seed.trigger_kind.clone());
    let mut trigger_detail = use_signal(|| seed.trigger_detail.clone());
    let mut reward_summary = use_signal(|| seed.reward_summary.clone());
    let mut command = use_signal(|| seed.command.clone());
    let site_scope = use_signal(|| seed.site_scope.clone());
    let cooldown = use_signal(|| seed.cooldown.clone());
    let mut active = use_signal(|| seed.active);

    let heading = if reward_id.is_some() {
        "Edit reward"
    } else {
        "New reward"
    };

    if missing {
        return rsx! {
            VotesApplicationsStyles {}
            button {
                r#type: "button",
                class: "pl-back",
                onclick: move |_| {
                    navigator.push(Route::CommunityVotes {});
                },
                "← Rewards"
            }
            h1 { class: "mt-4 text-2xl font-semibold tracking-tight", "Reward not found" }
            p { class: "mt-2 text-sm text-text-muted", "This reward may have been deleted." }
        };
    }

    let save = move |_| {
        let trimmed = name().trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        let draft = VoteReward {
            id: reward_id.unwrap_or(0),
            name: trimmed,
            trigger_kind: trigger_kind(),
            trigger_detail: trigger_detail().trim().to_string(),
            reward_summary: reward_summary().trim().to_string(),
            command: command().trim().to_string(),
            site_scope: site_scope(),
            cooldown: cooldown(),
            accent: String::from(VOTES_ACCENT),
            active: active(),
            claims,
            note: String::new(),
        };
        match reward_id {
            Some(id) => {
                rewards.with_mut(|list| {
                    if let Some(slot) = list.iter_mut().find(|r| r.id == id) {
                        *slot = draft;
                    }
                });
            }
            None => {
                rewards.with_mut(|list| {
                    let next_id = list.iter().map(|r| r.id).max().unwrap_or(0) + 1;
                    let mut draft = draft;
                    draft.id = next_id;
                    list.push(draft);
                });
            }
        }
        navigator.push(Route::CommunityVotes {});
    };

    rsx! {
        VotesApplicationsStyles {}
        div { class: "vote-console-page",
            button {
                r#type: "button",
                class: "pl-back",
                onclick: move |_| {
                    navigator.push(Route::CommunityVotes {});
                },
                "← Rewards"
            }

            div { class: "mb-8 mt-4",
                p { class: "pl-eyebrow", "Votes" }
                h1 { class: "mt-1 text-2xl font-semibold tracking-tight text-text",
                    "{heading}"
                }
                p { class: "mt-1.5 max-w-xl text-sm text-text-muted",
                    "Define what is granted after a verified vote callback from a listing site."
                }
            }

            div { class: "vote-editor",
                div { class: "vote-editor-main",
                    section { class: "vote-editor-section",
                        h2 { class: "vote-editor-heading", "Reward rule" }
                        p { class: "vote-editor-lede",
                            "The name players see and what triggers the payout."
                        }
                        LabeledField { label: "Reward name",
                            SignalInput {
                                value: name,
                                placeholder: "Vote crate key".to_string(),
                            }
                        }
                        LabeledField { label: "Trigger",
                            SignalSelect {
                                value: trigger_kind,
                                options: vote_trigger_options(),
                            }
                        }
                        LabeledField { label: "Trigger detail",
                            SignalInput {
                                value: trigger_detail,
                                placeholder: "Each successful vote".to_string(),
                            }
                        }
                    }
                    section { class: "vote-editor-section",
                        h2 { class: "vote-editor-heading", "Payout" }
                        p { class: "vote-editor-lede",
                            "What is granted and how it reaches the player."
                        }
                        LabeledField { label: "Reward summary",
                            SignalInput {
                                value: reward_summary,
                                placeholder: "1x Vote Key + 250 coins".to_string(),
                            }
                        }
                        LabeledField { label: "Command",
                            SignalInput {
                                value: command,
                                placeholder: "crate give {player} vote 1".to_string(),
                            }
                        }
                    }
                    section { class: "vote-editor-section",
                        h2 { class: "vote-editor-heading", "Listing sites & limits" }
                        p { class: "vote-editor-lede",
                            "Which connected sites pay this out, and how often."
                        }
                        LabeledField { label: "Listing sites",
                            SignalSelect {
                                value: site_scope,
                                options: vote_site_scope_options(),
                            }
                        }
                        LabeledField { label: "Claim cooldown",
                            SignalSelect {
                                value: cooldown,
                                options: vote_cooldown_options(),
                            }
                        }
                        label { class: "vote-editor-check",
                            input {
                                r#type: "checkbox",
                                checked: active(),
                                onchange: move |e| active.set(e.checked()),
                            }
                            span { "Enabled — grant when a matching site reports a vote" }
                        }
                    }
                    div { class: "mt-6 flex flex-wrap gap-2",
                        Button { onclick: save, "Save reward" }
                        Button {
                            variant: ButtonVariant::Ghost,
                            onclick: move |_| {
                                navigator.push(Route::CommunityVotes {});
                            },
                            "Cancel"
                        }
                    }
                }

                aside { class: "vote-editor-aside",
                    div { class: "vote-editor-summary",
                        p { class: "pl-eyebrow", "How it pays out" }
                        dl { class: "vote-payout-dl",
                            div {
                                dt { "Trigger" }
                                dd { "{trigger_kind}" }
                            }
                            div {
                                dt { "Grant" }
                                dd {
                                    if reward_summary().trim().is_empty() {
                                        "—"
                                    } else {
                                        "{reward_summary}"
                                    }
                                }
                            }
                            div {
                                dt { "Sites" }
                                dd { "{site_scope}" }
                            }
                            div {
                                dt { "Cooldown" }
                                dd { "{cooldown}" }
                            }
                            div {
                                dt { "Status" }
                                dd {
                                    if active() {
                                        "Enabled"
                                    } else {
                                        "Paused"
                                    }
                                }
                            }
                        }
                        p { class: "vote-editor-aside-note",
                            "Callbacks are configured per listing site under Votes → Settings."
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStatus {
    Submitted,
    Reviewing,
    Accepted,
    Denied,
}

impl ApplicationStatus {
    pub fn label(self) -> &'static str {
        match self {
            ApplicationStatus::Submitted => "Submitted",
            ApplicationStatus::Reviewing => "Reviewing",
            ApplicationStatus::Accepted => "Accepted",
            ApplicationStatus::Denied => "Denied",
        }
    }

    pub fn tone(self) -> &'static str {
        match self {
            ApplicationStatus::Submitted => "#fb7185",
            ApplicationStatus::Reviewing => "#fbbf24",
            ApplicationStatus::Accepted => "#3ecf8e",
            ApplicationStatus::Denied => "#8a8f98",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Application {
    pub id: u64,
    pub role: String,
    pub applicant: String,
    pub email: String,
    pub submitted: String,
    pub status: ApplicationStatus,
    pub summary: String,
    pub answers: Vec<(String, String)>,
    pub votes_yes: u32,
    pub votes_no: u32,
}

pub fn placeholder_applications() -> Vec<Application> {
    vec![
        Application {
            id: 1,
            role: "Moderator".into(),
            applicant: "AetherFox".into(),
            email: "aetherfox@players.serverspot.app".into(),
            submitted: "2h ago".into(),
            status: ApplicationStatus::Submitted,
            summary: "Active voter, plays daily, wants to help with chat moderation.".into(),
            answers: vec![
                (
                    "Why do you want to be a moderator?".into(),
                    "I'm online every evening and want to keep chat friendly.".into(),
                ),
                (
                    "How much time can you commit?".into(),
                    "About 2–3 hours a night, more on weekends.".into(),
                ),
                (
                    "Any past experience?".into(),
                    "Helper on a small SMP for a year.".into(),
                ),
            ],
            votes_yes: 3,
            votes_no: 1,
        },
        Application {
            id: 2,
            role: "Builder".into(),
            applicant: "SkyBuilder".into(),
            email: "skybuilder@players.serverspot.app".into(),
            submitted: "Yesterday".into(),
            status: ApplicationStatus::Reviewing,
            summary: "Redstone and terraforming portfolio attached.".into(),
            answers: vec![
                (
                    "Show us a build you're proud of.".into(),
                    "Full working guardian farm + hub island.".into(),
                ),
                (
                    "Preferred build style?".into(),
                    "Medieval fantasy with heavy detailing.".into(),
                ),
            ],
            votes_yes: 4,
            votes_no: 0,
        },
        Application {
            id: 3,
            role: "Moderator".into(),
            applicant: "PixelPaws".into(),
            email: "pixelpaws@players.serverspot.app".into(),
            submitted: "3 days ago".into(),
            status: ApplicationStatus::Accepted,
            summary: "Promoted after a strong trial week.".into(),
            answers: vec![(
                "Why moderator?".into(),
                "I already report rule-breakers and want to do more.".into(),
            )],
            votes_yes: 5,
            votes_no: 0,
        },
        Application {
            id: 4,
            role: "Event Host".into(),
            applicant: "GlacierGuy".into(),
            email: "glacierguy@players.serverspot.app".into(),
            submitted: "5 days ago".into(),
            status: ApplicationStatus::Denied,
            summary: "Not enough playtime yet — encouraged to reapply.".into(),
            answers: vec![(
                "What events would you run?".into(),
                "Build battles and parkour races.".into(),
            )],
            votes_yes: 1,
            votes_no: 4,
        },
    ]
}

const APPLICATION_FILTERS: &[(&str, Option<ApplicationStatus>)] = &[
    ("All", None),
    ("Submitted", Some(ApplicationStatus::Submitted)),
    ("Reviewing", Some(ApplicationStatus::Reviewing)),
    ("Accepted", Some(ApplicationStatus::Accepted)),
    ("Denied", Some(ApplicationStatus::Denied)),
];

#[component]
pub fn CommunityApplications() -> Element {
    let navigator = use_navigator();
    let applications = use_context::<Signal<Vec<Application>>>();
    let list = applications();
    let filter = use_signal(|| 0usize);
    let query = use_signal(String::new);

    let needle = query().to_lowercase();
    let active_filter = APPLICATION_FILTERS[filter()].1;
    let visible: Vec<Application> = list
        .iter()
        .filter(|a| active_filter.map(|s| s == a.status).unwrap_or(true))
        .filter(|a| {
            needle.is_empty()
                || a.applicant.to_lowercase().contains(&needle)
                || a.role.to_lowercase().contains(&needle)
        })
        .cloned()
        .collect();
    rsx! {
        VotesApplicationsStyles {}
        PageHeader {
            title: "Inbox",
            subtitle: "Every submitted application with its status, staff vote tally, and review link.",
            action: rsx! {
                Button {
                    onclick: move |_| {
                        navigator.push(Route::ApplicationFormNew {});
                    },
                    IconPlus {}
                    "New form"
                }
            },
        }

        div { class: "app-toolbar",
            div { class: "app-toolbar-search",
                SearchInput {
                    value: query,
                    placeholder: "Search applicants or roles…".to_string(),
                }
            }
            div { class: "app-toolbar-chips",
                for (index, (label, _)) in APPLICATION_FILTERS.iter().enumerate() {
                    {
                        let mut filter = filter;
                        let is_active = filter() == index;
                        rsx! {
                            button {
                                key: "{label}",
                                r#type: "button",
                                class: if is_active { "app-chip is-active" } else { "app-chip" },
                                onclick: move |_| filter.set(index),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }

        div { class: "app-console-panel",
            div { class: "app-console-panel-head",
                h2 { class: "text-sm font-semibold text-text", "Applications" }
                span { class: "text-xs text-text-muted", "{visible.len()} of {list.len()} shown" }
            }
            if visible.is_empty() {
                p { class: "px-4 py-6 text-sm text-text-muted",
                    "No applications match those filters."
                }
            } else {
                div { class: "motion-cascade motion-cascade-tight app-console-table",
                    div { class: "app-console-row is-head",
                        span { "Applicant" }
                        span { "Role" }
                        span { "Status" }
                        span { "Votes" }
                        span { "Submitted" }
                        span { "" }
                    }
                    for app in visible {
                        {
                            let app_id = app.id;
                            rsx! {
                                button {
                                    key: "{app.id}",
                                    r#type: "button",
                                    class: "app-console-row is-clickable",
                                    onclick: move |_| {
                                        navigator
                                            .push(Route::ApplicationReview {
                                                id: app_id,
                                            });
                                    },
                                    span { class: "app-console-name", "{app.applicant}" }
                                    span { "{app.role}" }
                                    span {
                                        StatusChip { label: app.status.label(), tone: app.status.tone() }
                                    }
                                    span { class: "tabular-nums", "{app.votes_yes} / {app.votes_no}" }
                                    span { class: "text-text-muted", "{app.submitted}" }
                                    span { class: "app-console-link", "Review" }
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
pub fn ApplicationReview(id: u64) -> Element {
    let navigator = use_navigator();
    let applications = use_context::<Signal<Vec<Application>>>();
    let app = applications.read().iter().find(|a| a.id == id).cloned();
    let note = use_signal(String::new);

    let Some(app) = app else {
        return rsx! {
            VotesApplicationsStyles {}
            button {
                r#type: "button",
                class: "pl-back",
                onclick: move |_| {
                    navigator.push(Route::CommunityApplications {});
                },
                "← Inbox"
            }
            h1 { class: "mt-4 text-2xl font-semibold tracking-tight", "Application not found" }
        };
    };

    rsx! {
        VotesApplicationsStyles {}
        button {
            r#type: "button",
            class: "pl-back",
            onclick: move |_| {
                navigator.push(Route::CommunityApplications {});
            },
            "← Inbox"
        }

        div { class: "mb-6 mt-4",
            p { class: "app-console-eyebrow", "Review · {app.role}" }
            h1 { class: "mt-1 text-2xl font-semibold tracking-tight text-text", "{app.applicant}" }
        }

        div { class: "app-editor-layout",
            div { class: "motion-cascade app-editor-main",
                section { class: "app-editor-section",
                    h2 { class: "app-editor-heading", "Answers" }
                    p { class: "app-editor-lede", "Submitted {app.submitted}." }
                    div { class: "motion-cascade motion-cascade-tight app-qa-list",
                        for (q, a) in app.answers.iter().cloned() {
                            div { class: "app-qa-item",
                                p { class: "app-qa-question", "{q}" }
                                p { class: "app-qa-answer", "{a}" }
                            }
                        }
                    }
                }
                section { class: "app-editor-section",
                    h2 { class: "app-editor-heading", "Reviewer notes" }
                    p { class: "app-editor-lede", "Internal notes are only visible to staff." }
                    SignalTextarea {
                        value: note,
                        placeholder: "Add a note for other reviewers…".to_string(),
                    }
                    div { class: "mt-3 flex flex-wrap gap-2",
                        Button { variant: ButtonVariant::Primary, "Accept" }
                        Button { variant: ButtonVariant::Secondary, "Keep reviewing" }
                        Button { variant: ButtonVariant::Danger, "Deny" }
                    }
                }
            }
            aside { class: "app-editor-aside",
                div { class: "app-editor-summary",
                    p { class: "pl-eyebrow", "Application summary" }
                    p { class: "mt-2 text-sm text-text-secondary", "{app.summary}" }
                    dl { class: "app-summary-dl",
                        div {
                            dt { "Role" }
                            dd { "{app.role}" }
                        }
                        div {
                            dt { "Status" }
                            dd { "{app.status.label()}" }
                        }
                        div {
                            dt { "Staff votes" }
                            dd { "{app.votes_yes} for · {app.votes_no} against" }
                        }
                        div {
                            dt { "Submitted" }
                            dd { "{app.submitted}" }
                        }
                    }
                    p { class: "app-editor-aside-note",
                        "Decisions email the applicant and update their account role."
                    }
                }
            }
        }
    }
}

#[component]
pub fn ApplicationFormNew() -> Element {
    let navigator = use_navigator();
    let role = use_signal(String::new);
    let description = use_signal(String::new);
    let questions = use_signal(|| {
        vec![
            String::from("Why do you want this role?"),
            String::from("How much time can you commit?"),
        ]
    });

    rsx! {
        VotesApplicationsStyles {}
        button {
            r#type: "button",
            class: "pl-back",
            onclick: move |_| {
                navigator.push(Route::CommunityApplications {});
            },
            "← Inbox"
        }

        div { class: "mb-6 mt-4",
            p { class: "app-console-eyebrow", "Applications" }
            h1 { class: "mt-1 text-2xl font-semibold tracking-tight text-text", "New form" }
        }

        div { class: "app-editor-layout",
            div { class: "motion-cascade app-editor-main",
                section { class: "app-editor-section",
                    h2 { class: "app-editor-heading", "Role" }
                    p { class: "app-editor-lede",
                        "Create a role and the questions applicants answer."
                    }
                    LabeledField { label: "Role name",
                        SignalInput { value: role, placeholder: "Moderator".to_string() }
                    }
                    LabeledField { label: "Description",
                        SignalTextarea {
                            value: description,
                            placeholder: "Who this role is for and what they do…".to_string(),
                        }
                    }
                }
                section { class: "app-editor-section",
                    h2 { class: "app-editor-heading", "Questions" }
                    p { class: "app-editor-lede", "Applicants answer these in order." }
                    div { class: "motion-cascade motion-cascade-tight app-question-list",
                        for (i, q) in questions().into_iter().enumerate() {
                            {
                                let mut questions = questions;
                                rsx! {
                                    div { key: "{i}", class: "app-question-row",
                                        span { class: "app-question-index", "{i + 1}" }
                                        span { class: "flex-1 text-sm text-text", "{q}" }
                                        button {
                                            r#type: "button",
                                            class: "app-console-link is-danger",
                                            onclick: move |_| {
                                                questions.with_mut(|list| {
                                                    if i < list.len() {
                                                        list.remove(i);
                                                    }
                                                });
                                            },
                                            "Remove"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Button {
                        class: "mt-3",
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            let mut questions = questions;
                            questions.with_mut(|list| list.push(String::from("New question")));
                        },
                        IconPlus {}
                        "Add question"
                    }
                }
                div { class: "mt-6 flex flex-wrap gap-2",
                    Button { "Save form" }
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            navigator.push(Route::CommunityApplications {});
                        },
                        "Cancel"
                    }
                }
            }
            aside { class: "app-editor-aside",
                div { class: "app-editor-summary",
                    p { class: "pl-eyebrow", "Form summary" }
                    p { class: "mt-2 text-lg font-semibold text-text",
                        if role().trim().is_empty() {
                            "Untitled role"
                        } else {
                            "{role}"
                        }
                    }
                    dl { class: "app-summary-dl",
                        div {
                            dt { "Questions" }
                            dd { "{questions().len()}" }
                        }
                        div {
                            dt { "Status" }
                            dd { "Draft" }
                        }
                    }
                    p { class: "app-editor-aside-note",
                        "Published forms appear on your website for players to fill in."
                    }
                }
            }
        }
    }
}

#[component]
fn LabeledField(label: &'static str, children: Element) -> Element {
    rsx! {
        label { class: "mb-4 block",
            span { class: "mb-1.5 block text-xs font-medium text-text-secondary", "{label}" }
            {children}
        }
    }
}
