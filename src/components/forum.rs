use dioxus::prelude::*;
use crate::components::page::{
    DataPanel, FeatureBullet, FeatureBullets, PageHeader, RowItem, SettingRow, StatPill,
};
use crate::components::ui::*;
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
            title: "Categories",
            subtitle: "Organize discussion boards, roles, and visibility for your forum.",
            Button {
                IconPlus {}
                "New category"
            }
        }
        section { class: "mb-6 grid grid-cols-2 gap-2 sm:mb-8 sm:gap-3 md:grid-cols-4",
            StatPill { label: "Threads", value: "1,204", accent: "#5b9dff" }
            StatPill { label: "Posts today", value: "52", accent: "#f071a5" }
            StatPill { label: "Members", value: "3,481", accent: "#3ecf8e" }
            StatPill { label: "Reports", value: "3", accent: "#f0a35e" }
        }
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Categories",
                RowItem {
                    title: "Announcements",
                    meta: "Pinned · 18 threads",
                    trailing: "Staff",
                }
                RowItem {
                    title: "Survival",
                    meta: "General talk · 412 threads",
                    trailing: "Public",
                }
                RowItem {
                    title: "Suggestions",
                    meta: "Player ideas · 96 threads",
                    trailing: "Public",
                }
                RowItem {
                    title: "Off-topic",
                    meta: "Community lounge · 220 threads",
                    trailing: "Public",
                }
            }
            DataPanel { title: "Forum features",
                FeatureBullets {
                    FeatureBullet { text: "Categories, posts, and comments" }
                    FeatureBullet { text: "Markdown support" }
                    FeatureBullet { text: "Reactions and user mentions" }
                    FeatureBullet { text: "Attachments and tags" }
                    FeatureBullet { text: "Pinning and locking" }
                    FeatureBullet { text: "Moderation tools and reports" }
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
        PageHeader {
            title: "Posts",
            subtitle: "Browse threads, replies, and media across every forum category.",
            Button {
                IconPlus {}
                "New thread"
            }
        }
        DataPanel { title: "Recent threads",
            RowItem {
                title: "Season 4 spawn redesign",
                meta: "Survival · 24 replies · 8 reactions",
                trailing: "2m",
            }
            RowItem {
                title: "Rank perks feedback",
                meta: "Suggestions · 11 replies · tagged: economy",
                trailing: "18m",
            }
            RowItem {
                title: "Patch notes 1.21.4",
                meta: "Announcements · Pinned · Locked comments",
                trailing: "1h",
            }
            RowItem {
                title: "Looking for builders",
                meta: "Off-topic · Mentions @SkyBuilder",
                trailing: "3h",
            }
            RowItem {
                title: "Screenshot dump",
                meta: "Survival · 3 attachments",
                trailing: "5h",
            }
        }
    }
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
        div { class: "grid gap-4 lg:grid-cols-2",
            DataPanel { title: "Admin features",
                FeatureBullets {
                    FeatureBullet { text: "Delete posts" }
                    FeatureBullet { text: "Delete comments" }
                    FeatureBullet { text: "Ban posting" }
                    FeatureBullet { text: "Manage categories" }
                    FeatureBullet { text: "Control permissions" }
                }
            }
            DataPanel { title: "Open reports",
                RowItem {
                    title: "Spam reply in Suggestions",
                    meta: "Reported by NovaCraft",
                    trailing: "Review",
                }
                RowItem {
                    title: "Toxic thread title",
                    meta: "Reported by QuietLeaf",
                    trailing: "Review",
                }
                RowItem {
                    title: "Duplicate announcement",
                    meta: "Reported by staff",
                    trailing: "Dismiss",
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
