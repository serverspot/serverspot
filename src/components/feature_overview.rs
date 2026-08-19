use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::community::{
    placeholder_players, placeholder_vote_sites, vote_site_status_class, vote_site_status_label,
    Application, ApplicationStatus, BoardPodium, BoardReset, BoardStandings, LeaderboardBoard,
    Player, PlayerStatus, PlayersLeaderboardsStyles, VoteReward, VotesApplicationsStyles,
    VoteSiteStatus,
};
use crate::components::page::{DataPanel, PageHeader, RowItem, StatPill};
use crate::components::ui::*;
use crate::i18n::t_key;
use crate::router::Route;

#[component]
pub fn PlayersOverview() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let players = use_hook(placeholder_players);
    let online = players
        .iter()
        .filter(|player| player.status == PlayerStatus::Online)
        .count();
    let spotlight = players.iter().copied().max_by_key(|player| player.level);
    let recent: Vec<Player> = use_hook(|| {
        let mut list = placeholder_players();
        list.sort_by(|a, b| b.level.cmp(&a.level));
        list.into_iter().take(4).collect()
    });

    rsx! {
        PlayersLeaderboardsStyles {}
        PageHeader {
            title: t_key("feature-overview-players-title"),
            subtitle: t_key("community-players-subtitle"),
            action: rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        navigator.push(Route::CommunityPlayers {});
                    },
                    { t!("feature-overview-open-roster") }
                }
            },
        }

        section { class: "motion-cascade stat-strip mb-8",
            StatPill { label: t_key("feature-overview-total-players"), value: "1,842", accent: "#69bdf2" }
            StatPill { label: t_key("feature-overview-linked-accounts"), value: "1,204", accent: "#3ecf8e" }
            StatPill {
                label: t_key("feature-overview-online-now"),
                value: online.to_string(),
                accent: "#5b9dff",
            }
            StatPill { label: t_key("feature-overview-servers"), value: "7", accent: "#87d1fe" }
        }

        if let Some(player) = spotlight {
            div { class: "roster-spotlight",
                span { class: "roster-spotlight-ribbon", { t!("feature-overview-spotlight") } }
                Avatar { email: player.email, size: 64, alt: player.username }
                div { class: "min-w-0",
                    div { class: "flex flex-wrap items-center gap-2",
                        p { class: "text-lg font-semibold tracking-tight text-text",
                            "{player.username}"
                        }
                        span {
                            class: "roster-rank-badge",
                            style: "--badge-color: {player.rank.tone()};",
                            "{player.rank.label()}"
                        }
                    }
                    p { class: "mt-1 max-w-md text-sm text-text-muted", "{player.bio}" }
                    Button {
                        class: "mt-3",
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator
                                .push(Route::PlayersProfileDetail {
                                    id: player.id,
                                });
                        },
                        { t!("feature-overview-open-file") }
                    }
                }
                div { class: "roster-spotlight-stats",
                    div { class: "roster-spotlight-stat",
                        p { class: "roster-spotlight-stat-value", "{player.level}" }
                        p { class: "roster-spotlight-stat-label", { t!("community-label-level") } }
                    }
                    div { class: "roster-spotlight-stat",
                        p { class: "roster-spotlight-stat-value", "{player.playtime_hours}h" }
                        p { class: "roster-spotlight-stat-label", { t!("community-label-playtime") } }
                    }
                    div { class: "roster-spotlight-stat",
                        p { class: "roster-spotlight-stat-value", "{player.votes}" }
                        p { class: "roster-spotlight-stat-label", { t!("community-label-votes") } }
                    }
                }
            }
        }

        section { class: "mb-2 flex items-baseline justify-between gap-3",
            h2 { class: "text-sm font-semibold text-text", { t!("feature-overview-recently-active") } }
            button {
                class: "text-xs font-semibold transition-colors",
                style: "color: #69bdf2;",
                r#type: "button",
                onclick: move |_| {
                    navigator.push(Route::CommunityPlayers {});
                },
                { t!("feature-overview-full-roster") }
            }
        }
        div { class: "motion-cascade motion-cascade-tight roster-recent",
            for player in recent {
                {
                    let player_id = player.id;
                    rsx! {
                        button {
                            key: "{player.id}",
                            class: "roster-recent-row",
                            r#type: "button",
                            onclick: move |_| {
                                navigator
                                    .push(Route::PlayersProfileDetail {
                                        id: player_id,
                                    });
                            },
                            Avatar { email: player.email, size: 36, alt: player.username }
                            div { class: "min-w-0 flex-1",
                                p { class: "truncate text-sm font-medium text-text", "{player.username}" }
                                p { class: "mt-0.5 truncate text-xs text-text-muted",
                                    { t!("community-level-rank-line", rank: player.rank.label(), level: player.level) }
                                }
                            }
                            span { class: "text-xs text-text-muted", "{player.last_seen}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LeaderboardsOverview() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let boards = use_context::<Signal<Vec<LeaderboardBoard>>>();
    let board_count = use_memo(move || boards.read().len());
    let with_rewards = use_memo(move || {
        boards
            .read()
            .iter()
            .filter(|b| !b.rewards.is_empty())
            .count()
    });
    let next_reset = use_memo(move || {
        boards
            .read()
            .iter()
            .filter(|b| b.reset != BoardReset::Never)
            .map(|b| b.reset.label())
            .next()
            .unwrap_or_else(|| t_key("community-board-reset-none"))
    });
    let ranked_players =
        use_memo(move || boards.read().iter().map(|b| b.entries.len()).sum::<usize>());
    let boards_now = use_memo(move || boards());

    rsx! {
        PlayersLeaderboardsStyles {}

        div { class: "board-console-masthead",
            div { class: "min-w-0",
                p { class: "board-console-eyebrow", { t!("community-leaderboard-admin") } }
                h1 { class: "board-console-title", { t!("feature-overview-boards-live", count: board_count()) } }
                p { class: "board-console-sub",
                    { t!("feature-overview-boards-sub") }
                }
            }
            div { class: "flex flex-wrap items-center gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::CommunityLeaderboards {});
                    },
                    { t!("feature-overview-all-boards") }
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::LeaderboardsBoardNew {});
                    },
                    IconPlus {}
                    { t!("community-new-board") }
                }
            }
        }

        section { class: "motion-cascade board-stat-strip",
            div { class: "board-stat-tile",
                span { class: "board-stat-tile-label", { t!("feature-overview-stat-boards") } }
                span { class: "board-stat-tile-value", "{board_count}" }
                span { class: "board-stat-tile-meta", { t!("feature-overview-stat-boards-meta") } }
            }
            div { class: "board-stat-tile",
                span { class: "board-stat-tile-label", { t!("feature-overview-stat-with-rewards") } }
                span { class: "board-stat-tile-value", "{with_rewards}" }
                span { class: "board-stat-tile-meta", { t!("feature-overview-stat-with-rewards-meta") } }
            }
            div { class: "board-stat-tile",
                span { class: "board-stat-tile-label", { t!("feature-overview-stat-ranked-seats") } }
                span { class: "board-stat-tile-value", "{ranked_players}" }
                span { class: "board-stat-tile-meta", { t!("feature-overview-stat-ranked-seats-meta") } }
            }
            div { class: "board-stat-tile",
                span { class: "board-stat-tile-label", { t!("feature-overview-stat-next-reset") } }
                span { class: "board-stat-tile-value", "{next_reset}" }
                span { class: "board-stat-tile-meta", { t!("feature-overview-stat-next-reset-meta") } }
            }
        }

        if boards_now().is_empty() {
            p { class: "text-sm text-text-muted",
                { t!("community-boards-empty") }
            }
        } else {
            div { class: "motion-cascade board-card-grid",
                for board in boards_now() {
                    {
                        let board_id = board.id;
                        let top: Vec<_> = board.entries.iter().copied().take(3).collect();
                        let rest: Vec<_> = board.entries.iter().copied().skip(3).take(3).collect();
                        let reward_line = board
                            .rewards
                            .first()
                            .map(|r| {
                                format!("#{place} · {summary}", place = r.place, summary = r.summary)
                            })
                            .unwrap_or_else(|| t_key("community-no-rank-rewards"));
                        rsx! {
                            article {
                                key: "{board.id}",
                                class: "board-card",
                                style: "--board-accent: {board.accent}",
                                div { class: "board-card-head",
                                    div { class: "min-w-0",
                                        h2 { class: "board-card-title", "{board.name}" }
                                        p { class: "board-card-meta",
                                            { t!("community-board-ranked-meta", count: board.entries.len(), source: board.source.label()) }
                                        }
                                    }
                                    div { class: "board-card-chips",
                                        span { class: "board-chip is-accent", "{board.stat.label()}" }
                                        span { class: "board-chip", "{board.reset.label()}" }
                                    }
                                }

                                BoardPodium { entries: top }
                                BoardStandings { entries: rest, skip_top: 0 }

                                div { class: "board-card-foot",
                                    div { class: "board-card-reward",
                                        strong { "{board.rewards.len()}" }
                                        span { "{reward_line}" }
                                    }
                                    div { class: "board-card-actions",
                                        button {
                                            r#type: "button",
                                            class: "board-card-btn is-primary",
                                            onclick: move |_| {
                                                navigator
                                                    .push(Route::LeaderboardsBoardEdit {
                                                        id: board_id,
                                                    });
                                            },
                                            { t!("feature-overview-open-board") }
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
pub fn VotesOverview() -> Element {
    let _lang = i18n();
    let rewards = use_context::<Signal<Vec<VoteReward>>>();
    let navigator = use_navigator();
    let list = use_memo(move || rewards());
    let sites = use_hook(placeholder_vote_sites);
    let votes_today: u32 = sites.iter().map(|s| s.votes_today).sum();
    let live = sites
        .iter()
        .filter(|s| s.status == VoteSiteStatus::Live)
        .count();
    let enabled_rewards = list().iter().filter(|r| r.active).count();
    let top_rewards = use_memo(move || {
        let mut top = list();
        top.sort_by(|a, b| b.claims.cmp(&a.claims));
        top.into_iter().take(3).collect::<Vec<_>>()
    });

    rsx! {
        VotesApplicationsStyles {}
        div { class: "vote-console-masthead",
            div { class: "min-w-0",
                p { class: "vote-console-eyebrow", { t!("feature-overview-vote-console") } }
                h1 { class: "vote-console-title",
                    { t!("feature-overview-vote-sites-live", live: live, total: sites.len()) }
                }
                p { class: "vote-console-sub",
                    { t!("feature-overview-vote-summary", votes: votes_today, rewards: enabled_rewards) }
                }
            }
            div { class: "flex flex-wrap items-center gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::VotesSiteSettings {});
                    },
                    { t!("feature-overview-sites-callbacks") }
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::CommunityVotes {});
                    },
                    { t!("nav-sub-rewards") }
                }
            }
        }

        section { class: "vote-site-panel",
            div { class: "vote-site-panel-head",
                h2 { class: "text-sm font-semibold text-text", { t!("feature-overview-connected-sites") } }
                span { class: "text-xs text-text-muted", { t!("feature-overview-callback-refresh") } }
            }
            div { class: "vote-site-table",
                div { class: "vote-site-row is-head",
                    span { { t!("feature-overview-col-site") } }
                    span { { t!("feature-overview-col-status") } }
                    span { { t!("feature-overview-col-votes-today") } }
                    span { { t!("feature-overview-col-cooldown") } }
                    span { { t!("feature-overview-col-last-callback") } }
                }
                for site in sites {
                    div { key: "{site.id}", class: "vote-site-row",
                        span { class: "vote-site-name", "{site.name}" }
                        span {
                            span { class: vote_site_status_class(site.status),
                                "{vote_site_status_label(site.status)}"
                            }
                        }
                        span { class: "tabular-nums", "{site.votes_today}" }
                        span { "{site.cooldown}" }
                        span { class: "text-text-muted", "{site.last_callback}" }
                    }
                }
            }
        }

        div { class: "motion-cascade grid gap-4 lg:grid-cols-2",
            DataPanel { title: t_key("feature-overview-top-rewards"),
                for reward in top_rewards() {
                    RowItem {
                        title: reward.name.clone(),
                        meta: format!("{} · {}", reward.trigger_kind, reward.reward_summary),
                        trailing: t!("community-claims-count", count: reward.claims),
                    }
                }
            }
            DataPanel { title: t_key("feature-overview-quick-status"),
                RowItem {
                    title: t_key("feature-overview-public-claim-path"),
                    meta: String::from("www.example.com/vote"),
                    trailing: t_key("feature-overview-on-website"),
                }
                RowItem {
                    title: t_key("feature-overview-callback-timeout"),
                    meta: t_key("feature-overview-callback-timeout-meta"),
                    trailing: t_key("feature-overview-status-ok"),
                }
                RowItem {
                    title: t_key("feature-overview-auto-claim"),
                    meta: t_key("feature-overview-auto-claim-meta"),
                    trailing: t_key("common-on"),
                }
            }
        }
    }
}

#[component]
pub fn ApplicationsOverview() -> Element {
    let _lang = i18n();
    let applications = use_context::<Signal<Vec<Application>>>();
    let navigator = use_navigator();
    let list = applications();
    let submitted = list
        .iter()
        .filter(|a| a.status == ApplicationStatus::Submitted)
        .count();
    let reviewing = list
        .iter()
        .filter(|a| a.status == ApplicationStatus::Reviewing)
        .count();
    let accepted = list
        .iter()
        .filter(|a| a.status == ApplicationStatus::Accepted)
        .count();
    let denied = list
        .iter()
        .filter(|a| a.status == ApplicationStatus::Denied)
        .count();

    let roles: Vec<String> = list
        .iter()
        .map(|a| a.role.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    let recent: Vec<Application> = {
        let mut items = list.clone();
        items.sort_by(|a, b| b.id.cmp(&a.id));
        items.into_iter().take(3).collect()
    };

    let open_count = submitted + reviewing;

    rsx! {
        VotesApplicationsStyles {}

        div { class: "app-console-masthead",
            div { class: "min-w-0",
                p { class: "app-console-eyebrow", { t!("feature-overview-applications") } }
                h1 { class: "app-console-title",
                    { t!("feature-overview-applications-need-decision", count: open_count) }
                }
                p { class: "app-console-sub",
                    { t!("feature-overview-applications-summary", submitted: submitted, reviewing: reviewing, roles: roles.len()) }
                }
            }
            div { class: "flex flex-wrap items-center gap-2",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::ApplicationsSiteSettings {});
                    },
                    { t!("feature-overview-form-settings") }
                }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::CommunityApplications {});
                    },
                    { t!("feature-overview-open-inbox") }
                }
            }
        }

        section { class: "app-console-panel",
            div { class: "app-console-panel-head",
                h2 { class: "text-sm font-semibold text-text", { t!("feature-overview-recent-submissions") } }
                span { class: "text-xs text-text-muted", { t!("feature-overview-newest-first") } }
            }
            if recent.is_empty() {
                p { class: "px-4 py-6 text-sm text-text-muted",
                    { t!("feature-overview-applications-empty") }
                }
            } else {
                div { class: "app-console-table is-compact",
                    div { class: "app-console-row is-head",
                        span { { t!("feature-overview-col-applicant") } }
                        span { { t!("feature-overview-col-role") } }
                        span { { t!("feature-overview-col-status") } }
                        span { { t!("feature-overview-col-votes") } }
                        span { { t!("feature-overview-col-submitted") } }
                    }
                    for app in recent {
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
                                    span { "{app.status.label()}" }
                                    span { class: "tabular-nums", "{app.votes_yes} / {app.votes_no}" }
                                    span { class: "text-text-muted", "{app.submitted}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        div { class: "mt-4 grid gap-4 lg:grid-cols-2",
            DataPanel { title: t_key("feature-overview-pipeline"),
                RowItem {
                    title: t_key("community-app-status-submitted"),
                    meta: t_key("feature-overview-pipeline-submitted-meta"),
                    trailing: format!("{submitted}"),
                }
                RowItem {
                    title: t_key("community-app-status-reviewing"),
                    meta: t_key("feature-overview-pipeline-reviewing-meta"),
                    trailing: format!("{reviewing}"),
                }
                RowItem {
                    title: t_key("community-app-status-accepted"),
                    meta: t_key("feature-overview-pipeline-accepted-meta"),
                    trailing: format!("{accepted}"),
                }
                RowItem {
                    title: t_key("community-app-status-denied"),
                    meta: t_key("feature-overview-pipeline-denied-meta"),
                    trailing: format!("{denied}"),
                }
            }
            DataPanel { title: t_key("feature-overview-open-roles"),
                if roles.is_empty() {
                    RowItem {
                        title: t_key("feature-overview-no-roles"),
                        meta: t_key("feature-overview-no-roles-meta"),
                        trailing: String::from("—"),
                    }
                } else {
                    for name in roles {
                        RowItem {
                            title: name.clone(),
                            meta: t_key("feature-overview-accepting-applications"),
                            trailing: t_key("feature-overview-role-open"),
                        }
                    }
                }
            }
        }
    }
}
