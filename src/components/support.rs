use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, SettingRow, SettingsControl, SettingsField, StatusChip,
};
use crate::components::ui::*;
use crate::i18n::t_key;
use crate::router::Route;
use crate::user::CurrentUser;

const TICKET_PAGE_SIZE: usize = 6;
const SUPPORT_ACCENT: &str = "#f0a35e";
const HELP_CATEGORIES: &[&str] = &["Guides", "Rules", "Tutorials", "Billing"];
const FILTER_ALL_DEPARTMENTS: &str = "all";
const FILTER_ALL_ASSIGNEES: &str = "all";
const FILTER_ALL_PRIORITIES: &str = "all";
const UNASSIGNED_ASSIGNEE: &str = "Unassigned";

fn support_help_card_views(count: u32) -> String {
    t!("support-help-card-views", count: count)
}

fn support_help_card_foot(updated: String, helpful: u32) -> String {
    t!("support-help-card-foot", updated: updated, helpful: helpful)
}

fn support_automation_subtitle(live: usize, rule_count: usize) -> String {
    t!("support-automation-subtitle", live: live, total: rule_count)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TicketStatus {
    All,
    Open,
    Pending,
    Resolved,
    Closed,
}

impl TicketStatus {
    fn label(self) -> String {
        match self {
            Self::All => t_key("support-ticket-status-all"),
            Self::Open => t_key("support-ticket-status-open"),
            Self::Pending => t_key("support-ticket-status-pending"),
            Self::Resolved => t_key("support-ticket-status-resolved"),
            Self::Closed => t_key("support-ticket-status-closed"),
        }
    }
    fn tone(self) -> &'static str {
        match self {
            Self::All => SUPPORT_ACCENT,
            Self::Open => "#f0a35e",
            Self::Pending => "#f5c14a",
            Self::Resolved => "#3ecf8e",
            Self::Closed => "#858899",
        }
    }
}

#[derive(Clone, PartialEq)]
struct Ticket {
    id: u64,
    subject: String,
    player: String,
    email: String,
    status: TicketStatus,
    department: String,
    priority: String,
    assignee: String,
    when: String,
    note: String,
}

fn placeholder_tickets() -> Vec<Ticket> {
    vec![
        Ticket {
            id: 1842,
            subject: String::from("Payment not received"),
            player: String::from("NovaCraft"),
            email: String::from("novacraft@players.serverspot.app"),
            status: TicketStatus::Open,
            department: String::from("Store"),
            priority: String::from("High"),
            assignee: String::from("Unassigned"),
            when: String::from("11m"),
            note: String::from("Player paid via PayPal but rank never applied."),
        },
        Ticket {
            id: 1839,
            subject: String::from("Can't join lobby"),
            player: String::from("SkyBuilder"),
            email: String::from("skybuilder@players.serverspot.app"),
            status: TicketStatus::Pending,
            department: String::from("Gameplay"),
            priority: String::from("Normal"),
            assignee: String::from("Mira"),
            when: String::from("34m"),
            note: String::from("Waiting on proxy reconnect logs."),
        },
        Ticket {
            id: 1833,
            subject: String::from("Rank missing perks"),
            player: String::from("AetherFox"),
            email: String::from("aetherfox@players.serverspot.app"),
            status: TicketStatus::Open,
            department: String::from("Store"),
            priority: String::from("Normal"),
            assignee: String::from("Unassigned"),
            when: String::from("1h"),
            note: String::from("Internal note: LP group looks correct — recheck kit."),
        },
        Ticket {
            id: 1828,
            subject: String::from("Ban appeal"),
            player: String::from("RedstoneRex"),
            email: String::from("redstonerex@players.serverspot.app"),
            status: TicketStatus::Pending,
            department: String::from("Moderation"),
            priority: String::from("Low"),
            assignee: String::from("Jordan"),
            when: String::from("3h"),
            note: String::from("History: 2 prior appeals. Review chat logs."),
        },
        Ticket {
            id: 1821,
            subject: String::from("Vote rewards not claiming"),
            player: String::from("QuietLeaf"),
            email: String::from("quietleaf@players.serverspot.app"),
            status: TicketStatus::Resolved,
            department: String::from("Gameplay"),
            priority: String::from("Normal"),
            assignee: String::from("Mira"),
            when: String::from("5h"),
            note: String::from("Fixed — voter link was stale."),
        },
        Ticket {
            id: 1814,
            subject: String::from("Wrong package delivered"),
            player: String::from("CopperFox"),
            email: String::from("copperfox@players.serverspot.app"),
            status: TicketStatus::Open,
            department: String::from("Store"),
            priority: String::from("High"),
            assignee: String::from("Unassigned"),
            when: String::from("Yesterday"),
            note: String::from("Bought MVP, received VIP kit."),
        },
        Ticket {
            id: 1807,
            subject: String::from("Account linking failed"),
            player: String::from("MossyBee"),
            email: String::from("mossybee@players.serverspot.app"),
            status: TicketStatus::Closed,
            department: String::from("Account"),
            priority: String::from("Normal"),
            assignee: String::from("Jordan"),
            when: String::from("Yesterday"),
            note: String::from("Player resolved by re-auth."),
        },
        Ticket {
            id: 1799,
            subject: String::from("Report harassment in Survival"),
            player: String::from("PixelNomad"),
            email: String::from("pixelnomad@players.serverspot.app"),
            status: TicketStatus::Pending,
            department: String::from("Moderation"),
            priority: String::from("High"),
            assignee: String::from("Mira"),
            when: String::from("2d"),
            note: String::from("Escalated — awaiting screenshots."),
        },
        Ticket {
            id: 1792,
            subject: String::from("Refund request for crate keys"),
            player: String::from("LunarWisp"),
            email: String::from("lunarwisp@players.serverspot.app"),
            status: TicketStatus::Resolved,
            department: String::from("Store"),
            priority: String::from("Normal"),
            assignee: String::from("Jordan"),
            when: String::from("2d"),
            note: String::from("Partial refund issued."),
        },
        Ticket {
            id: 1785,
            subject: String::from("How do I transfer a gift?"),
            player: String::from("OakSpire"),
            email: String::from("oakspire@players.serverspot.app"),
            status: TicketStatus::Closed,
            department: String::from("Store"),
            priority: String::from("Low"),
            assignee: String::from("Mira"),
            when: String::from("3d"),
            note: String::from("Pointed to help centre article."),
        },
        Ticket {
            id: 1778,
            subject: String::from("Creative plot wipe"),
            player: String::from("GlintFox"),
            email: String::from("glintfox@players.serverspot.app"),
            status: TicketStatus::Open,
            department: String::from("Gameplay"),
            priority: String::from("High"),
            assignee: String::from("Unassigned"),
            when: String::from("3d"),
            note: String::from("Plot ID 441 — check rollback window."),
        },
        Ticket {
            id: 1771,
            subject: String::from("Discord role not syncing"),
            player: String::from("TideRunner"),
            email: String::from("tiderunner@players.serverspot.app"),
            status: TicketStatus::Pending,
            department: String::from("Account"),
            priority: String::from("Normal"),
            assignee: String::from("Jordan"),
            when: String::from("4d"),
            note: String::from("Bot reconnect scheduled."),
        },
    ]
}

fn priority_tone(priority: &str) -> &'static str {
    match priority {
        "High" => "#f07167",
        "Low" => "#858899",
        _ => SUPPORT_ACCENT,
    }
}

#[component]
pub fn SupportTickets() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let tickets = use_signal(placeholder_tickets);
    let mut query = use_signal(String::new);
    let mut status = use_signal(|| TicketStatus::All);
    let mut department = use_signal(|| String::from(FILTER_ALL_DEPARTMENTS));
    let mut assignee = use_signal(|| String::from(FILTER_ALL_ASSIGNEES));
    let mut priority = use_signal(|| String::from(FILTER_ALL_PRIORITIES));
    let mut selected = use_signal(|| Option::<u64>::None);
    let mut visible = use_signal(|| TICKET_PAGE_SIZE);

    use_effect(move || {
        let _ = (query(), status(), department(), assignee(), priority());
        visible.set(TICKET_PAGE_SIZE);
    });

    let open_count = use_memo(move || {
        tickets
            .read()
            .iter()
            .filter(|ticket| ticket.status == TicketStatus::Open)
            .count()
    });
    let pending_count = use_memo(move || {
        tickets
            .read()
            .iter()
            .filter(|ticket| ticket.status == TicketStatus::Pending)
            .count()
    });
    let unassigned_count = use_memo(move || {
        tickets
            .read()
            .iter()
            .filter(|ticket| {
                ticket.assignee == UNASSIGNED_ASSIGNEE
                    && matches!(ticket.status, TicketStatus::Open | TicketStatus::Pending)
            })
            .count()
    });

    let attention = use_memo(move || {
        tickets
            .read()
            .iter()
            .find(|ticket| {
                ticket.priority == "High"
                    && matches!(ticket.status, TicketStatus::Open | TicketStatus::Pending)
            })
            .cloned()
    });
    let attention_id = attention().as_ref().map(|ticket| ticket.id);

    let department_options = use_memo(move || {
        std::iter::once(String::from(FILTER_ALL_DEPARTMENTS))
            .chain(
                tickets
                    .read()
                    .iter()
                    .map(|ticket| ticket.department.clone())
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter(),
            )
            .map(|value| {
                let label = if value == FILTER_ALL_DEPARTMENTS {
                    t_key("support-filter-all-departments")
                } else {
                    value.clone()
                };
                SelectOption::new(value, label)
            })
            .collect::<Vec<_>>()
    });
    let assignee_options = use_memo(move || {
        std::iter::once(String::from(FILTER_ALL_ASSIGNEES))
            .chain(
                tickets
                    .read()
                    .iter()
                    .map(|ticket| ticket.assignee.clone())
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter(),
            )
            .map(|value| {
                let label = if value == FILTER_ALL_ASSIGNEES {
                    t_key("support-filter-all-assignees")
                } else {
                    value.clone()
                };
                SelectOption::new(value, label)
            })
            .collect::<Vec<_>>()
    });

    let filtered = use_memo(move || {
        let q = query().trim().to_lowercase();
        let status_now = status();
        let department_now = department();
        let assignee_now = assignee();
        let priority_now = priority();
        tickets
            .read()
            .iter()
            .filter(|ticket| {
                let status_ok = status_now == TicketStatus::All || ticket.status == status_now;
                let department_ok = department_now == FILTER_ALL_DEPARTMENTS
                    || ticket.department == department_now;
                let assignee_ok = assignee_now == FILTER_ALL_ASSIGNEES
                    || ticket.assignee == assignee_now;
                let priority_ok = priority_now == FILTER_ALL_PRIORITIES
                    || ticket.priority == priority_now;
                let search_ok = q.is_empty()
                    || ticket.subject.to_lowercase().contains(&q)
                    || ticket.player.to_lowercase().contains(&q)
                    || ticket.email.to_lowercase().contains(&q)
                    || ticket.department.to_lowercase().contains(&q)
                    || ticket.assignee.to_lowercase().contains(&q)
                    || ticket.id.to_string().contains(&q);
                status_ok && department_ok && assignee_ok && priority_ok && search_ok
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let matched = filtered().len();
    let limit = visible().min(matched);
    let remaining = matched.saturating_sub(limit);
    let can_load_more = remaining > 0;
    let filters_active = status() != TicketStatus::All
        || department() != FILTER_ALL_DEPARTMENTS
        || assignee() != FILTER_ALL_ASSIGNEES
        || priority() != FILTER_ALL_PRIORITIES
        || !query().trim().is_empty();

    let selected_ticket = {
        let filtered_now = filtered();
        selected()
            .and_then(|id| filtered_now.iter().find(|ticket| ticket.id == id).cloned())
            .or_else(|| filtered_now.first().cloned())
    };

    let summary_open = open_count();
    let summary_pending = pending_count();
    let summary_unassigned = unassigned_count();
    let summary_text = t!(
        "support-tickets-summary",
        open: summary_open,
        pending: summary_pending,
        unassigned: summary_unassigned
    );
    let search_placeholder = t!("support-tickets-search-placeholder");
    let attention_block = attention().map(|case| {
        let wait = t!("support-tickets-waiting", when: case.when.clone());
        let meta = t!(
            "support-tickets-attention-meta",
            id: case.id,
            department: case.department.clone(),
            priority: case.priority.clone()
        );
        (case, wait, meta)
    });

    rsx! {
        div { class: "mb-2 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("support-tickets-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    { t!("support-tickets-title") }
                }
            }
            if filters_active {
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        query.set(String::new());
                        status.set(TicketStatus::All);
                        department.set(String::from(FILTER_ALL_DEPARTMENTS));
                        assignee.set(String::from(FILTER_ALL_ASSIGNEES));
                        priority.set(String::from(FILTER_ALL_PRIORITIES));
                        selected.set(None);
                        visible.set(TICKET_PAGE_SIZE);
                    },
                    { t!("support-tickets-clear-filters") }
                }
            }
        }

        p { class: "mb-10 text-sm text-text-muted", "{summary_text}" }

        if let Some((case, attention_wait, attention_meta)) = attention_block {
            section { class: "mb-10",
                p { class: "mb-4 text-xs font-medium uppercase tracking-wide text-text-muted",
                    { t!("support-tickets-needs-you") }
                }
                button {
                    r#type: "button",
                    class: "support-attention",
                    onclick: move |_| {
                        if let Some(id) = attention_id {
                            navigator.push(Route::SupportTicket { id });
                        }
                    },
                    div { class: "support-attention-top",
                        StatusChip {
                            label: case.status.label(),
                            tone: case.status.tone(),
                        }
                        span { class: "support-attention-wait", "{attention_wait}" }
                    }
                    h2 { class: "support-attention-title", "{case.subject}" }
                    p { class: "support-attention-note", "{case.note}" }
                    div { class: "support-attention-meta",
                        Avatar {
                            email: case.email.clone(),
                            size: 32,
                            alt: case.player.clone(),
                        }
                        div { class: "min-w-0",
                            p { class: "truncate text-sm font-medium text-text", "{case.player}" }
                            p { class: "truncate text-xs text-text-muted", "{attention_meta}" }
                        }
                    }
                }
            }
        }

        div { class: "support-filters mb-6",
            SearchInput {
                class: "support-filters-search",
                value: query,
                placeholder: "{search_placeholder}",
            }
            div { class: "support-filters-chips",
                for chip in [
                    TicketStatus::All,
                    TicketStatus::Open,
                    TicketStatus::Pending,
                    TicketStatus::Resolved,
                    TicketStatus::Closed,
                ]
                {
                    {
                        let active = status() == chip;
                        rsx! {
                            button {
                                class: if active { "support-chip is-active" } else { "support-chip" },
                                style: if active { format!("--chip-accent: {}", chip.tone()) } else { String::new() },
                                onclick: move |_| {
                                    status.set(chip);
                                    selected.set(None);
                                },
                                "{chip.label()}"
                            }
                        }
                    }
                }
            }
            div { class: "support-filters-selects",
                SignalSelect {
                    value: department,
                    options: department_options(),
                }
                SignalSelect {
                    value: assignee,
                    options: assignee_options(),
                }
                SignalSelect {
                    value: priority,
                    options: [
                        (FILTER_ALL_PRIORITIES, t_key("support-filter-all-priorities")),
                        ("High", t_key("support-priority-high")),
                        ("Normal", t_key("support-priority-normal")),
                        ("Low", t_key("support-priority-low")),
                    ]
                    .into_iter()
                    .map(|(value, label)| SelectOption::new(value, label))
                    .collect(),
                }
            }
        }

        div { class: "support-desk",
            div { class: "motion-cascade motion-cascade-tight support-stack",
                if matched == 0 {
                    div { class: "support-stack-empty",
                        p { class: "text-sm font-medium text-text", { t!("support-tickets-inbox-quiet") } }
                        p { class: "mt-1 text-sm text-text-muted", { t!("support-tickets-no-matches") } }
                    }
                } else {
                    for ticket in filtered().into_iter().take(limit) {
                        {
                            let ticket_id = ticket.id;
                            let active =
                                selected_ticket.as_ref().is_some_and(|item| item.id == ticket_id);
                            let tone = priority_tone(&ticket.priority);
                            rsx! {
                                button {
                                    key: "{ticket_id}",
                                    r#type: "button",
                                    class: if active { "support-case is-active" } else { "support-case" },
                                    style: "--case-accent: {tone};",
                                    onclick: move |_| selected.set(Some(ticket_id)),
                                    span { class: "support-case-rail" }
                                    div { class: "support-case-body",
                                        div { class: "support-case-head",
                                            p { class: "support-case-subject", "{ticket.subject}" }
                                        }
                                        div { class: "support-case-foot",
                                            span { "{ticket.player}" }
                                            span { "·" }
                                            span { "{ticket.department}" }
                                        }
                                    }
                                    span { class: "support-case-wait", "{ticket.when}" }
                                }
                            }
                        }
                    }
                    if can_load_more {
                        button {
                            class: "support-stack-more",
                            onclick: move |_| {
                                let next = *visible.peek() + TICKET_PAGE_SIZE;
                                visible.set(next.min(matched));
                            },
                            { t!("support-tickets-show-older", remaining: remaining) }
                        }
                    }
                }
            }

            aside { class: "support-reading",
                if let Some(ticket) = selected_ticket {
                    {
                        let tone = priority_tone(&ticket.priority);
                        let ticket_id = ticket.id;
                        rsx! {
                            div { class: "support-reading-head",
                                div { class: "min-w-0",
                                    p { class: "support-reading-kicker",
                                        { t!("support-tickets-case-kicker", id: ticket.id) }
                                    }
                                    h2 { class: "support-reading-title", "{ticket.subject}" }
                                }
                                StatusChip { label: ticket.status.label(), tone: ticket.status.tone() }
                            }

                            div { class: "support-reading-tags",
                                span { class: "support-tag", style: "--tag-accent: {tone};", "{ticket.priority}" }
                                span { class: "support-tag", "{ticket.department}" }
                                span { class: "support-tag", "{ticket.assignee}" }
                                span { class: "support-tag is-muted",
                                    { t!("support-tickets-opened-ago", when: ticket.when.clone()) }
                                }
                            }

                            div { class: "mt-5 flex items-center gap-3",
                                Avatar { email: ticket.email.clone(), size: 36, alt: ticket.player.clone() }
                                div { class: "min-w-0",
                                    p { class: "truncate text-sm font-medium text-text", "{ticket.player}" }
                                }
                            }

                            div { class: "mt-5",
                                p { class: "text-xs text-text-muted", { t!("support-tickets-latest-note") } }
                                p { class: "mt-1.5 text-sm leading-relaxed text-text-secondary", "{ticket.note}" }
                            }

                            div { class: "mt-8",
                                Button {
                                    onclick: move |_| {
                                        navigator
                                            .push(Route::SupportTicket {
                                                id: ticket_id,
                                            });
                                    },
                                    { t!("support-tickets-open-ticket") }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "support-reading-empty",
                        p { class: "text-sm font-medium text-text", { t!("support-tickets-pick-case") } }
                        p { class: "mt-1 max-w-xs text-sm text-text-muted",
                            { t!("support-tickets-pick-case-hint") }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SupportTicket(id: u64) -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut reply = use_signal(String::new);
    let ticket = use_hook(|| {
        placeholder_tickets()
            .into_iter()
            .find(|ticket| ticket.id == id)
    });

    let Some(ticket) = ticket else {
        return rsx! {
            div { class: "mb-6",
                p { class: "text-sm text-text-muted", { t!("support-tickets-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", { t!("support-ticket-not-found-title") } }
                p { class: "mt-2 text-sm text-text-secondary", { t!("support-ticket-not-found-desc") } }
                div { class: "mt-6",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            navigator.push(Route::SupportTickets {});
                        },
                        { t!("support-ticket-back-inbox") }
                    }
                }
            }
        };
    };

    let tone = priority_tone(&ticket.priority);
    let reply_placeholder = t!("support-ticket-reply-placeholder", player: ticket.player.clone());
    let meta_when = ticket.when.clone();
    let meta_department = ticket.department.clone();
    let meta_priority = ticket.priority.clone();
    let meta_assignee = ticket.assignee.clone();
    let header_eyebrow = t!("support-ticket-header-eyebrow", id: ticket.id);
    let meta_text = t!(
        "support-ticket-meta",
        when: meta_when,
        department: meta_department,
        priority: meta_priority,
        assignee: meta_assignee
    );

    rsx! {
        div { class: "mb-2 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "{header_eyebrow}" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "{ticket.subject}"
                }
            }
            div { class: "flex flex-wrap items-center gap-2",
                StatusChip { label: ticket.status.label(), tone: ticket.status.tone() }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SupportTickets {});
                    },
                    { t!("support-ticket-back-inbox") }
                }
            }
        }

        p { class: "mb-8 text-sm text-text-muted", "{meta_text}" }

        div { class: "support-ticket-page",
            section { class: "support-ticket-main",
                div { class: "support-letter",
                    div { class: "support-letter-from",
                        Avatar {
                            email: ticket.email.clone(),
                            size: 40,
                            alt: ticket.player.clone(),
                        }
                        div { class: "min-w-0",
                            p { class: "truncate text-sm font-medium text-text", "{ticket.player}" }
                        }
                    }
                    p { class: "support-letter-body", "{ticket.note}" }
                }

                div { class: "support-composer mt-5",
                    SignalTextarea {
                        value: reply,
                        placeholder: reply_placeholder,
                        class: "support-composer-input min-h-[7rem]",
                    }
                    div { class: "support-composer-actions",
                        Button {
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                reply.write().clear();
                            },
                            { t!("support-ticket-send-reply") }
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            { t!("support-ticket-assign") }
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Ghost,
                            { t!("support-ticket-internal-note") }
                        }
                    }
                }
            }

            aside { class: "support-ticket-side",
                p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                    { t!("support-ticket-case-details") }
                }
                div { class: "support-reading-tags mt-4",
                    span { class: "support-tag", style: "--tag-accent: {tone};", "{ticket.priority}" }
                    span { class: "support-tag", "{ticket.department}" }
                    span { class: "support-tag", "{ticket.assignee}" }
                    span { class: "support-tag is-muted",
                        { t!("support-tickets-opened-ago", when: ticket.when.clone()) }
                    }
                }
                div { class: "mt-6 space-y-4",
                    div {
                        p { class: "text-xs text-text-muted", { t!("support-ticket-field-player") } }
                        p { class: "mt-1 text-sm font-medium text-text", "{ticket.player}" }
                    }
                    div {
                        p { class: "text-xs text-text-muted", { t!("support-ticket-field-status") } }
                        p { class: "mt-1 text-sm font-medium text-text",
                            "{ticket.status.label()}"
                        }
                    }
                    div {
                        p { class: "text-xs text-text-muted", { t!("support-ticket-field-status-chip") } }
                        div { class: "mt-1.5",
                            StatusChip {
                                label: ticket.status.label(),
                                tone: ticket.status.tone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Field(
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Presence {
    Online,
    Away,
    Offline,
}

impl Presence {
    fn tone(self) -> &'static str {
        match self {
            Self::Online => "#3ecf8e",
            Self::Away => "#f5c14a",
            Self::Offline => "#858899",
        }
    }

    fn label(self) -> String {
        match self {
            Self::Online => t_key("support-presence-online"),
            Self::Away => t_key("support-presence-away"),
            Self::Offline => t_key("support-presence-offline"),
        }
    }
}

#[derive(Clone, PartialEq)]
struct StaffMember {
    name: String,
    email: String,
    role: String,
    presence: Presence,
    active_tickets: u32,
    capacity: u32,
}

fn placeholder_staff() -> Vec<StaffMember> {
    vec![
        StaffMember {
            name: String::from("Mira"),
            email: String::from("mira@serverspot.app"),
            role: String::from("Support lead"),
            presence: Presence::Online,
            active_tickets: 5,
            capacity: 8,
        },
        StaffMember {
            name: String::from("Jordan"),
            email: String::from("jordan@serverspot.app"),
            role: String::from("Moderation & appeals"),
            presence: Presence::Away,
            active_tickets: 3,
            capacity: 6,
        },
        StaffMember {
            name: String::from("Charlie Admin"),
            email: String::from("admin@serverspot.app"),
            role: String::from("Owner"),
            presence: Presence::Online,
            active_tickets: 2,
            capacity: 10,
        },
        StaffMember {
            name: String::from("Sana"),
            email: String::from("sana@serverspot.app"),
            role: String::from("Store & billing"),
            presence: Presence::Offline,
            active_tickets: 0,
            capacity: 6,
        },
    ]
}

#[component]
pub fn SupportOverview() -> Element {
    let _lang = i18n();
    let articles = use_context::<Signal<Vec<Article>>>();
    let navigator = use_navigator();
    let tickets = use_hook(placeholder_tickets);
    let staff = use_hook(placeholder_staff);
    let staff_len = staff.len();

    let open_count = tickets
        .iter()
        .filter(|ticket| ticket.status == TicketStatus::Open)
        .count();
    let pending_count = tickets
        .iter()
        .filter(|ticket| ticket.status == TicketStatus::Pending)
        .count();
    let unassigned_count = tickets
        .iter()
        .filter(|ticket| {
            ticket.assignee == UNASSIGNED_ASSIGNEE
                && matches!(ticket.status, TicketStatus::Open | TicketStatus::Pending)
        })
        .count();

    let mut open_now: Vec<Ticket> = tickets
        .iter()
        .filter(|ticket| matches!(ticket.status, TicketStatus::Open | TicketStatus::Pending))
        .cloned()
        .collect();
    open_now.sort_by_key(|ticket| match ticket.priority.as_str() {
        "High" => 0,
        "Normal" => 1,
        _ => 2,
    });
    open_now.truncate(6);

    let online_count = staff
        .iter()
        .filter(|member| member.presence == Presence::Online)
        .count();

    let article_list = articles();
    let article_count = article_list.len();
    let helpful_avg = if article_list.is_empty() {
        0
    } else {
        article_list.iter().map(|a| a.helpful_pct).sum::<u32>() / article_list.len() as u32
    };
    let live_rules = AUTOMATION_RULES_DEFAULT_LIVE;
    let staff_online_text = t!("support-overview-staff-online", online: online_count, total: staff_len);
    let help_centre_meta = t!(
        "support-overview-help-centre-meta",
        count: article_count,
        helpful: helpful_avg
    );
    let automation_meta = t!("support-overview-automation-meta", count: live_rules);

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("support-overview-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    { t!("support-overview-title") }
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary",
                    { t!("support-overview-subtitle") }
                }
            }
            Button { variant: ButtonVariant::Secondary, { t!("support-overview-view-site") } }
        }

        div { class: "motion-cascade ops-meter",
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", { t!("support-overview-open-now") } }
                p { class: "ops-meter-value is-accent", "{open_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", { t!("support-overview-pending-reply") } }
                p { class: "ops-meter-value", "{pending_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", { t!("support-overview-unassigned") } }
                p { class: "ops-meter-value", "{unassigned_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", { t!("support-overview-avg-first-reply") } }
                p { class: "ops-meter-value", "14m" }
            }
        }

        section { class: "mb-9",
            div { class: "ops-section-head",
                h2 { class: "ops-section-title", { t!("support-overview-open-right-now") } }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SupportTickets {});
                    },
                    { t!("support-overview-view-inbox") }
                }
            }
            if open_now.is_empty() {
                p { class: "ops-empty", { t!("support-overview-nothing-waiting") } }
            } else {
                div { class: "motion-cascade motion-cascade-tight ops-strip",
                    for ticket in open_now {
                        {
                            let tone = priority_tone(&ticket.priority);
                            rsx! {
                                button {
                                    r#type: "button",
                                    key: "{ticket.id}",
                                    class: "ops-ticket",
                                    style: "--ops-accent: {tone};",
                                    onclick: move |_| {
                                        navigator.push(Route::SupportTickets {});
                                    },
                                    span { class: "ops-ticket-rail" }
                                    div { class: "ops-ticket-body",
                                        div { class: "ops-ticket-top",
                                            StatusChip { label: ticket.status.label(), tone: ticket.status.tone() }
                                            span { class: "ops-ticket-wait", "{ticket.when}" }
                                        }
                                        p { class: "ops-ticket-subject", "{ticket.subject}" }
                                        p { class: "ops-ticket-meta", "{ticket.player} · {ticket.department}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        section { class: "mb-9",
            div { class: "ops-section-head",
                h2 { class: "ops-section-title", { t!("support-overview-staff-desk") } }
                p { class: "ops-section-sub", "{staff_online_text}" }
            }
            div { class: "motion-cascade motion-cascade-tight ops-roster",
                for member in staff {
                    {
                        let load_pct = if member.capacity == 0 {
                            0
                        } else {
                            (member.active_tickets * 100 / member.capacity).min(100)
                        };
                        let roster_active = member.active_tickets;
                        let roster_capacity = member.capacity;
                        let roster_load = t!(
                            "support-overview-roster-load",
                            active: roster_active,
                            capacity: roster_capacity
                        );
                        rsx! {
                            div { class: "ops-roster-row", key: "{member.name}",
                                div { class: "ops-roster-avatar-wrap",
                                    Avatar { email: member.email.clone(), size: 36, alt: member.name.clone() }
                                    span {
                                        class: "ops-roster-dot",
                                        style: "--presence-color: {member.presence.tone()};",
                                    }
                                }
                                div { class: "ops-roster-info",
                                    p { class: "ops-roster-name", "{member.name}" }
                                    p { class: "ops-roster-role", "{member.role} · {member.presence.label()}" }
                                }
                                div { class: "ops-roster-load",
                                    div { class: "ops-roster-load-fill", style: "width: {load_pct}%;" }
                                }
                                span { class: "ops-roster-load-label", "{roster_load}" }
                            }
                        }
                    }
                }
            }
        }

        section {
            div { class: "motion-cascade ops-pulse",
                button {
                    r#type: "button",
                    class: "ops-pulse-link",
                    onclick: move |_| {
                        navigator.push(Route::SupportHelpCentre {});
                    },
                    div {
                        p { class: "ops-pulse-title", { t!("support-overview-help-centre") } }
                        p { class: "ops-pulse-meta", "{help_centre_meta}" }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
                button {
                    r#type: "button",
                    class: "ops-pulse-link",
                    onclick: move |_| {
                        navigator.push(Route::SupportAutomation {});
                    },
                    div {
                        p { class: "ops-pulse-title", { t!("support-overview-automation") } }
                        p { class: "ops-pulse-meta", "{automation_meta}" }
                    }
                    span { class: "ops-pulse-arrow", "→" }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArticleStatus {
    Published,
    Featured,
    Draft,
}

impl ArticleStatus {
    fn label(self) -> String {
        match self {
            Self::Published => t_key("support-article-status-published"),
            Self::Featured => t_key("support-article-status-featured"),
            Self::Draft => t_key("support-article-status-draft"),
        }
    }

    fn tone(self) -> &'static str {
        match self {
            Self::Published => "#3ecf8e",
            Self::Featured => "#f5c14a",
            Self::Draft => "#858899",
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Article {
    id: u64,
    title: String,
    category: String,
    snippet: String,
    body: String,
    status: ArticleStatus,
    views: u32,
    helpful_pct: u32,
    updated: String,
    author: String,
}

pub(crate) fn placeholder_articles() -> Vec<Article> {
    vec![
        Article {
            id: 1,
            title: String::from("How to claim vote rewards"),
            category: String::from("Guides"),
            snippet: String::from(
                "Vote on all five listing sites, then run /vote claim in-game to collect streak rewards.",
            ),
            body: String::from(
                "Vote on all five listing sites within a 24 hour window, then run /vote claim in-game. Streaks reset if you miss a day — check your streak with /vote streak.",
            ),
            status: ArticleStatus::Featured,
            views: 2140,
            helpful_pct: 94,
            updated: String::from("2d"),
            author: String::from("Mira"),
        },
        Article {
            id: 2,
            title: String::from("Server rules"),
            category: String::from("Rules"),
            snippet: String::from(
                "The full rulebook covering griefing, chat conduct, and ban appeals.",
            ),
            body: String::from(
                "1. No griefing outside of designated PvP zones.\n2. Keep chat civil — no harassment or hate speech.\n3. Advertising other servers results in an instant ban.",
            ),
            status: ArticleStatus::Published,
            views: 1420,
            helpful_pct: 88,
            updated: String::from("6d"),
            author: String::from("Charlie Admin"),
        },
        Article {
            id: 3,
            title: String::from("Connecting with Bedrock"),
            category: String::from("Tutorials"),
            snippet: String::from(
                "Cross-play setup for console and mobile players using the Bedrock gateway.",
            ),
            body: String::from(
                "Bedrock players can join on port 19132 using the same address as Java. Skins and some blocks render differently — see the compatibility table below.",
            ),
            status: ArticleStatus::Published,
            views: 980,
            helpful_pct: 81,
            updated: String::from("1w"),
            author: String::from("Jordan"),
        },
        Article {
            id: 4,
            title: String::from("Store refund policy"),
            category: String::from("Billing"),
            snippet: String::from(
                "What qualifies for a refund and how long it takes to process.",
            ),
            body: String::from(
                "Refunds are available within 14 days of purchase if the item hasn't been used. Submit a ticket under the Store department with your order ID.",
            ),
            status: ArticleStatus::Published,
            views: 640,
            helpful_pct: 76,
            updated: String::from("2w"),
            author: String::from("Sana"),
        },
        Article {
            id: 5,
            title: String::from("Setting up two-factor authentication"),
            category: String::from("Guides"),
            snippet: String::from(
                "Secure your account with an authenticator app in under two minutes.",
            ),
            body: String::from(
                "Head to Account → Security and scan the QR code with an authenticator app. Save your backup codes somewhere safe — support can't restore 2FA without them.",
            ),
            status: ArticleStatus::Draft,
            views: 0,
            helpful_pct: 0,
            updated: String::from("3h"),
            author: String::from("Mira"),
        },
        Article {
            id: 6,
            title: String::from("Appealing a ban"),
            category: String::from("Rules"),
            snippet: String::from(
                "How to submit an appeal and what staff look for when reviewing it.",
            ),
            body: String::from(
                "Open a ticket under the Moderation department with your username and the ban reason. Appeals are reviewed within 48 hours — duplicate tickets will be closed.",
            ),
            status: ArticleStatus::Published,
            views: 512,
            helpful_pct: 69,
            updated: String::from("3w"),
            author: String::from("Jordan"),
        },
        Article {
            id: 7,
            title: String::from("Transferring a gifted rank"),
            category: String::from("Billing"),
            snippet: String::from(
                "Gifted ranks can be reassigned once within 30 days of purchase.",
            ),
            body: String::from(
                "Contact the original purchaser to confirm, then open a Store ticket with both usernames. Transfers outside the 30 day window need owner approval.",
            ),
            status: ArticleStatus::Draft,
            views: 0,
            helpful_pct: 0,
            updated: String::from("1d"),
            author: String::from("Sana"),
        },
        Article {
            id: 8,
            title: String::from("Building permissions in Creative"),
            category: String::from("Tutorials"),
            snippet: String::from(
                "Plot trust levels, WorldEdit access, and how to request a plot reset.",
            ),
            body: String::from(
                "Trust a co-builder with /plot trust <name>. WorldEdit is available to Builder rank and above. Plot resets can be requested once every 30 days.",
            ),
            status: ArticleStatus::Published,
            views: 355,
            helpful_pct: 90,
            updated: String::from("4d"),
            author: String::from("Mira"),
        },
    ]
}

fn next_article_id(articles: &[Article]) -> u64 {
    articles.iter().map(|article| article.id).max().unwrap_or(0) + 1
}

#[component]
pub fn SupportHelpCentre() -> Element {
    let _lang = i18n();
    let articles = use_context::<Signal<Vec<Article>>>();
    let navigator = use_navigator();
    let all = articles();

    let total = all.len();
    let helpful_avg = if total == 0 {
        0
    } else {
        all.iter().map(|a| a.helpful_pct).sum::<u32>() / total as u32
    };
    let shelf_count = HELP_CATEGORIES.len();
    let help_subtitle = t!(
        "support-help-subtitle",
        total: total,
        shelves: shelf_count,
        helpful: helpful_avg
    );

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("support-help-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    { t!("support-help-title") }
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary", "{help_subtitle}" }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::SupportHelpNew {});
                },
                IconPlus {}
                { t!("support-help-new-article") }
            }
        }

        div { class: "shelf-index",
            for category in HELP_CATEGORIES.iter().copied() {
                {
                    let count = all.iter().filter(|a| a.category == category).count();
                    let anchor = format!("#shelf-{category}");
                    rsx! {
                        a { key: "{category}", href: "{anchor}", class: "shelf-index-pill",
                            "{category}"
                            span { class: "shelf-index-count", " · {count}" }
                        }
                    }
                }
            }
        }

        for category in HELP_CATEGORIES.iter().copied() {
            {
                let items: Vec<Article> = all
                    .iter()
                    .filter(|a| a.category == category && a.status != ArticleStatus::Draft)
                    .cloned()
                    .collect();
                rsx! {
                    if !items.is_empty() {
                        section { key: "{category}", id: "shelf-{category}", class: "shelf",
                            div { class: "shelf-head",
                                h2 { class: "shelf-title", "{category}" }
                                span { class: "shelf-count",
                                    { t!("support-help-shelf-count", count: items.len()) }
                                }
                            }
                            div { class: "shelf-rail",
                                for article in items {
                                    {
                                        let article_id = article.id;
                                        let card_views = support_help_card_views(article.views);
                                        let card_foot =
                                            support_help_card_foot(article.updated.clone(), article.helpful_pct);
                                        rsx! {
                                            button {
                                                r#type: "button",
                                                key: "{article.id}",
                                                class: "shelf-card",
                                                onclick: move |_| {
                                                    navigator
                                                        .push(Route::SupportHelpEdit {
                                                            id: article_id,
                                                        });
                                                },
                                                div { class: "shelf-card-top",
                                                    StatusChip { label: article.status.label(), tone: article.status.tone() }
                                                    span { class: "shelf-card-views", "{card_views}" }
                                                }
                                                h3 { class: "shelf-card-title", "{article.title}" }
                                                p { class: "shelf-card-snippet", "{article.snippet}" }
                                                div { class: "shelf-card-foot", "{card_foot}" }
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
}

#[component]
pub fn SupportHelpNew() -> Element {
    let _lang = i18n();
    rsx! {
        ArticleEditor { article_id: None }
    }
}

#[component]
pub fn SupportHelpEdit(id: u64) -> Element {
    let _lang = i18n();
    rsx! {
        ArticleEditor { article_id: Some(id) }
    }
}

#[component]
fn ArticleEditor(article_id: Option<u64>) -> Element {
    let _lang = i18n();
    let mut articles = use_context::<Signal<Vec<Article>>>();
    let current_user = use_context::<Signal<CurrentUser>>();
    let navigator = use_navigator();
    let is_new = article_id.is_none();

    let (seed, missing) = use_hook(|| {
        let existing = article_id.and_then(|id| {
            articles
                .peek()
                .iter()
                .find(|article| article.id == id)
                .cloned()
        });
        let missing = !is_new && existing.is_none();
        let seed = existing.unwrap_or_else(|| Article {
            id: 0,
            title: String::new(),
            category: String::from(HELP_CATEGORIES[0]),
            snippet: String::new(),
            body: String::new(),
            status: ArticleStatus::Draft,
            views: 0,
            helpful_pct: 0,
            updated: String::from("just now"),
            author: current_user.peek().name.clone(),
        });
        (seed, missing)
    });

    let title = use_signal(|| seed.title.clone());
    let category = use_signal(|| seed.category.clone());
    let snippet = use_signal(|| seed.snippet.clone());
    let body = use_signal(|| seed.body.clone());
    let mut status = use_signal(|| seed.status);

    let title_now = title();
    let category_now = category();
    let snippet_now = snippet();
    let status_now = status();
    let can_save = !title_now.trim().is_empty();

    if missing {
        return rsx! {
            div { class: "mb-8 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SupportHelpCentre {});
                    },
                    { t!("support-article-back") }
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", { t!("support-article-not-found-title") } }
            p { class: "mt-2 text-sm text-text-muted", { t!("support-article-not-found-desc") } }
        };
    }

    let save = move |_| {
        let title_value = title().trim().to_string();
        if title_value.is_empty() {
            return;
        }
        let category_value = category();
        let body_value = body();
        let snippet_value = {
            let value = snippet().trim().to_string();
            if value.is_empty() {
                body_value.trim().chars().take(120).collect::<String>()
            } else {
                value
            }
        };
        let status_value = status();

        articles.with_mut(|list| {
            if let Some(id) = article_id {
                if let Some(article) = list.iter_mut().find(|article| article.id == id) {
                    article.title = title_value;
                    article.category = category_value;
                    article.snippet = snippet_value;
                    article.body = body_value;
                    article.status = status_value;
                    article.updated = String::from("just now");
                }
            } else {
                let id = next_article_id(list);
                list.push(Article {
                    id,
                    title: title_value,
                    category: category_value,
                    snippet: snippet_value,
                    body: body_value,
                    status: status_value,
                    views: 0,
                    helpful_pct: 0,
                    updated: String::from("just now"),
                    author: current_user().name,
                });
            }
        });

        navigator.push(Route::SupportHelpCentre {});
    };

    let preview_title = if title_now.trim().is_empty() {
        t_key("support-article-preview-title")
    } else {
        title_now.trim().to_string()
    };
    let preview_snippet = if snippet_now.trim().is_empty() {
        t_key("support-article-preview-snippet")
    } else {
        snippet_now.trim().to_string()
    };

    rsx! {
        div { class: "motion-cascade forum-editor",
            div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SupportHelpCentre {});
                    },
                    { t!("support-article-back") }
                }
                div { class: "flex flex-wrap items-center gap-2",
                    if let Some(id) = article_id {
                        Button {
                            variant: ButtonVariant::Danger,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                articles.with_mut(|list| list.retain(|article| article.id != id));
                                navigator.push(Route::SupportHelpCentre {});
                            },
                            { t!("support-article-delete") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::SupportHelpCentre {});
                        },
                        { t!("support-article-cancel") }
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: save,
                        if is_new {
                            { t!("support-article-publish") }
                        } else {
                            { t!("support-article-save") }
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", { t!("support-help-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        { t!("support-article-new-title") }
                    } else {
                        { t!("support-article-edit-title") }
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted", { t!("support-article-lede") } }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("support-article-section-basics") } }
                        p { class: "forum-editor-lede", { t!("support-article-section-basics-lede") } }
                        div { class: "mt-4 space-y-4",
                            Field { label: t_key("support-article-field-title"),
                                SignalInput {
                                    value: title,
                                    placeholder: t_key("support-article-placeholder-title"),
                                }
                            }
                            Field { label: t_key("support-article-field-shelf"),
                                SignalSelect {
                                    value: category,
                                    options: HELP_CATEGORIES
                                        .iter()
                                        .map(|c| SelectOption::new(*c, *c))
                                        .collect(),
                                }
                            }
                            Field {
                                label: t_key("support-article-field-snippet"),
                                hint: Some(t_key("support-article-field-snippet-hint")),
                                SignalTextarea {
                                    value: snippet,
                                    placeholder: "{t!(\"support-article-placeholder-snippet\")}",
                                    class: "min-h-[4.5rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("support-article-section-content") } }
                        p { class: "forum-editor-lede", { t!("support-article-section-content-lede") } }
                        div { class: "mt-4",
                            SignalTextarea {
                                value: body,
                                placeholder: "{t!(\"support-article-placeholder-body\")}",
                                class: "min-h-[14rem]",
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", { t!("support-article-section-status") } }
                        p { class: "forum-editor-lede", { t!("support-article-section-status-lede") } }
                        div { class: "mt-4 flex flex-wrap gap-2",
                            for option in [ArticleStatus::Draft, ArticleStatus::Published, ArticleStatus::Featured] {
                                button {
                                    r#type: "button",
                                    key: "{option.label()}",
                                    class: if status_now == option { "shelf-status-option is-active" } else { "shelf-status-option" },
                                    style: "--status-accent: {option.tone()};",
                                    onclick: move |_| status.set(option),
                                    "{option.label()}"
                                }
                            }
                        }
                    }
                }

                aside { class: "forum-editor-aside",
                    div { class: "forum-editor-preview",
                        p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                            { t!("support-article-shelf-preview") }
                        }
                        div {
                            class: "mt-4 shelf-card",
                            style: "cursor: default; width: 100%;",
                            div { class: "shelf-card-top",
                                StatusChip {
                                    label: status_now.label(),
                                    tone: status_now.tone(),
                                }
                                span { class: "shelf-card-views", { t!("support-article-preview-views") } }
                            }
                            h3 { class: "shelf-card-title", "{preview_title}" }
                            p { class: "shelf-card-snippet", "{preview_snippet}" }
                            div { class: "shelf-card-foot",
                                { t!("support-article-preview-foot", category: category_now.clone()) }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct AutomationRuleMeta {
    name_key: &'static str,
    description_key: &'static str,
    category_key: &'static str,
    tone: &'static str,
    trigger_key: &'static str,
}

const AUTOMATION_RULES: &[AutomationRuleMeta] = &[
    AutomationRuleMeta {
        name_key: "support-auto-rule-ai-first-reply-name",
        description_key: "support-auto-rule-ai-first-reply-desc",
        category_key: "support-auto-category-first-reply",
        tone: "#f0a35e",
        trigger_key: "support-auto-rule-ai-first-reply-trigger",
    },
    AutomationRuleMeta {
        name_key: "support-auto-rule-discord-alert-name",
        description_key: "support-auto-rule-discord-alert-desc",
        category_key: "support-auto-category-notifications",
        tone: "#5b9dff",
        trigger_key: "support-auto-rule-discord-alert-trigger",
    },
    AutomationRuleMeta {
        name_key: "support-auto-rule-sla-breach-name",
        description_key: "support-auto-rule-sla-breach-desc",
        category_key: "support-auto-category-escalation",
        tone: "#f87171",
        trigger_key: "support-auto-rule-sla-breach-trigger",
    },
    AutomationRuleMeta {
        name_key: "support-auto-rule-csat-name",
        description_key: "support-auto-rule-csat-desc",
        category_key: "support-auto-category-notifications",
        tone: "#5b9dff",
        trigger_key: "support-auto-rule-csat-trigger",
    },
    AutomationRuleMeta {
        name_key: "support-auto-rule-auto-close-name",
        description_key: "support-auto-rule-auto-close-desc",
        category_key: "support-auto-category-cleanup",
        tone: "#858899",
        trigger_key: "support-auto-rule-auto-close-trigger",
    },
    AutomationRuleMeta {
        name_key: "support-auto-rule-duplicate-name",
        description_key: "support-auto-rule-duplicate-desc",
        category_key: "support-auto-category-cleanup",
        tone: "#858899",
        trigger_key: "support-auto-rule-duplicate-trigger",
    },
];

const AUTOMATION_RULES_DEFAULT_LIVE: usize = 4;

#[component]
fn RuleSwitch(checked: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: if checked { "auto-switch is-on" } else { "auto-switch" },
            role: "switch",
            "aria-checked": if checked { "true" } else { "false" },
            onclick: move |evt| onclick.call(evt),
            span { class: "auto-switch-knob" }
        }
    }
}

#[component]
pub fn SupportAutomation() -> Element {
    let _lang = i18n();
    let mut enabled = use_signal(|| vec![true, true, true, true, false, false]);
    let enabled_now = enabled();
    let live_count = enabled_now.iter().filter(|on| **on).count();
    let rules_total = AUTOMATION_RULES.len();
    let automation_subtitle = support_automation_subtitle(live_count, rules_total);

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", { t!("support-automation-eyebrow") } }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    { t!("support-automation-title") }
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary", "{automation_subtitle}" }
            }
            Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary,
                IconAi {}
                { t!("support-automation-configure-ai") }
            }
        }

        div { class: "auto-layout",
            div { class: "motion-cascade auto-rails",
                for (index, rule) in AUTOMATION_RULES.iter().enumerate() {
                    {
                        let on = enabled_now[index];
                        let name = t_key(rule.name_key);
                        let category = t_key(rule.category_key);
                        let description = t_key(rule.description_key);
                        let trigger = t_key(rule.trigger_key);
                        rsx! {
                            div {
                                class: "auto-rail",
                                key: "{rule.name_key}",
                                style: "--rail-tone: {rule.tone};",
                                span { class: "auto-rail-bar" }
                                div { class: "auto-rail-body",
                                    div { class: "auto-rail-copy",
                                        div { class: "auto-rail-top",
                                            p { class: "auto-rail-name", "{name}" }
                                            span { class: "auto-rail-tag", "{category}" }
                                        }
                                        p { class: "auto-rail-desc", "{description}" }
                                        p { class: "auto-rail-trigger", "{trigger}" }
                                    }
                                    RuleSwitch {
                                        checked: on,
                                        onclick: move |_| {
                                            enabled.with_mut(|list| {
                                                if let Some(value) = list.get_mut(index) {
                                                    *value = !*value;
                                                }
                                            });
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }

            aside { class: "auto-preview",
                p { class: "auto-preview-kicker", { t!("support-automation-preview-kicker") } }
                div { class: "auto-preview-ticket",
                    p { class: "auto-preview-ticket-label", { t!("support-automation-preview-incoming") } }
                    p { class: "auto-preview-ticket-text",
                        "“I paid for VIP through PayPal but the rank never showed up in-game. Can someone check?”"
                    }
                }
                div { class: "auto-preview-reply",
                    span { class: "auto-preview-badge",
                        IconAi { class: "h-3.5 w-3.5" }
                        { t!("support-automation-preview-drafted") }
                    }
                    p { class: "auto-preview-reply-text",
                        "Hi NovaCraft — thanks for flagging this. I can see the payment but the rank grant didn't run. I've queued a manual grant and it should apply within 10 minutes. Sorry for the wait!"
                    }
                    div { class: "auto-preview-meter",
                        div { class: "auto-preview-meter-track",
                            div {
                                class: "auto-preview-meter-fill",
                                style: "width: 82%;",
                            }
                        }
                        span { class: "auto-preview-meter-label", { t!("support-automation-preview-confidence") } }
                    }
                }
                div { class: "auto-preview-actions",
                    Button { size: ButtonSize::Sm, { t!("support-automation-use-reply") } }
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Secondary,
                        { t!("support-automation-regenerate") }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SupportSiteSettings() -> Element {
    let _lang = i18n();
    let public_path = use_signal(|| String::from("/support"));
    let page_title = use_signal(|| String::from("Support"));
    let department = use_signal(|| String::from("Store"));
    let priority = use_signal(|| String::from("Normal"));

    rsx! {
        FeatureSettingsChrome { subtitle: t_key("support-settings-subtitle"),
            DataPanel { title: t_key("support-settings-panel-path"),
                SettingsControl { label: t_key("support-settings-field-public-path"),
                    SignalInput {
                        value: public_path,
                        placeholder: t_key("support-settings-placeholder-path"),
                    }
                }
                SettingsField {
                    label: t_key("support-settings-field-full-url"),
                    value: format!("www.example.com{}", public_path()),
                }
                SettingsControl { label: t_key("support-settings-field-page-title"),
                    SignalInput { value: page_title, placeholder: t_key("support-settings-placeholder-page-title") }
                }
                p { class: "pt-3 text-xs text-text-muted", { t!("support-settings-domain-hint") } }
            }
            DataPanel { title: t_key("support-settings-panel-ticket-defaults"),
                SettingsControl { label: t_key("support-settings-field-default-department"),
                    SignalSelect {
                        value: department,
                        options: ["Store", "Gameplay", "Moderation", "Account"]
                            .into_iter()
                            .map(|name| SelectOption::new(name, name))
                            .collect(),
                    }
                }
                SettingsControl { label: t_key("support-settings-field-default-priority"),
                    SignalSelect {
                        value: priority,
                        options: [
                            ("Low", t_key("support-priority-low")),
                            ("Normal", t_key("support-priority-normal")),
                            ("High", t_key("support-priority-high")),
                        ]
                        .into_iter()
                        .map(|(value, label)| SelectOption::new(value, label))
                        .collect(),
                    }
                }
                SettingRow {
                    title: t_key("support-settings-business-hours-title"),
                    description: t_key("support-settings-business-hours-desc"),
                    enabled: false,
                }
            }
            DataPanel { title: t_key("support-settings-panel-help-centre"),
                SettingRow {
                    title: t_key("support-settings-public-search-title"),
                    description: t_key("support-settings-public-search-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("support-settings-feedback-title"),
                    description: t_key("support-settings-feedback-desc"),
                    enabled: true,
                }
            }
            DataPanel { title: t_key("support-settings-panel-automation"),
                SettingRow {
                    title: t_key("support-auto-rule-ai-first-reply-name"),
                    description: t_key("support-auto-rule-ai-first-reply-desc"),
                    enabled: true,
                }
                SettingRow {
                    title: t_key("support-settings-discord-alerts-title"),
                    description: t_key("support-settings-discord-alerts-desc"),
                    enabled: true,
                }
            }
        }
    }
}
