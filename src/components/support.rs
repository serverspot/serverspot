use dioxus::prelude::*;

use crate::components::page::{
    DataPanel, FeatureSettingsChrome, SettingRow, SettingsControl, SettingsField, StatusChip,
};
use crate::components::ui::*;
use crate::router::Route;
use crate::user::CurrentUser;

const TICKET_PAGE_SIZE: usize = 6;
const SUPPORT_ACCENT: &str = "#f0a35e";
const HELP_CATEGORIES: &[&str] = &["Guides", "Rules", "Tutorials", "Billing"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum TicketStatus {
    All,
    Open,
    Pending,
    Resolved,
    Closed,
}

impl TicketStatus {
    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Open => "Open",
            Self::Pending => "Pending",
            Self::Resolved => "Resolved",
            Self::Closed => "Closed",
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
    let navigator = use_navigator();
    let tickets = use_signal(placeholder_tickets);
    let mut query = use_signal(String::new);
    let mut status = use_signal(|| TicketStatus::All);
    let mut department = use_signal(|| String::from("All departments"));
    let mut assignee = use_signal(|| String::from("All assignees"));
    let mut priority = use_signal(|| String::from("All priorities"));
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
                ticket.assignee == "Unassigned"
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
        std::iter::once(String::from("All departments"))
            .chain(
                tickets
                    .read()
                    .iter()
                    .map(|ticket| ticket.department.clone())
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter(),
            )
            .collect::<Vec<_>>()
    });
    let assignee_options = use_memo(move || {
        std::iter::once(String::from("All assignees"))
            .chain(
                tickets
                    .read()
                    .iter()
                    .map(|ticket| ticket.assignee.clone())
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter(),
            )
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
                let department_ok =
                    department_now == "All departments" || ticket.department == department_now;
                let assignee_ok =
                    assignee_now == "All assignees" || ticket.assignee == assignee_now;
                let priority_ok =
                    priority_now == "All priorities" || ticket.priority == priority_now;
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
        || department() != "All departments"
        || assignee() != "All assignees"
        || priority() != "All priorities"
        || !query().trim().is_empty();

    let selected_ticket = {
        let filtered_now = filtered();
        selected()
            .and_then(|id| filtered_now.iter().find(|ticket| ticket.id == id).cloned())
            .or_else(|| filtered_now.first().cloned())
    };

    rsx! {
        div { class: "mb-2 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Desk" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "Inbox"
                }
            }
            if filters_active {
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        query.set(String::new());
                        status.set(TicketStatus::All);
                        department.set(String::from("All departments"));
                        assignee.set(String::from("All assignees"));
                        priority.set(String::from("All priorities"));
                        selected.set(None);
                        visible.set(TICKET_PAGE_SIZE);
                    },
                    "Clear filters"
                }
            }
        }

        p { class: "mb-10 text-sm text-text-muted",
            "{open_count()} open · {pending_count()} awaiting reply · {unassigned_count()} unassigned"
        }

        if let Some(case) = attention() {
            section { class: "mb-10",
                p { class: "mb-4 text-xs font-medium uppercase tracking-wide text-text-muted",
                    "Needs you"
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
                        span { class: "support-attention-wait", "Waiting {case.when}" }
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
                            p { class: "truncate text-xs text-text-muted",
                                "#{case.id} · {case.department} · {case.priority} priority"
                            }
                        }
                    }
                }
            }
        }

        div { class: "support-filters mb-6",
            SearchInput {
                class: "support-filters-search",
                value: query,
                placeholder: "Search player, subject, or ticket #…",
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
                    options: department_options()
                        .into_iter()
                        .map(|name| SelectOption::new(name.clone(), name))
                        .collect(),
                }
                SignalSelect {
                    value: assignee,
                    options: assignee_options()
                        .into_iter()
                        .map(|name| SelectOption::new(name.clone(), name))
                        .collect(),
                }
                SignalSelect {
                    value: priority,
                    options: ["All priorities", "High", "Normal", "Low"]
                        .into_iter()
                        .map(|name| SelectOption::new(name, name))
                        .collect(),
                }
            }
        }

        div { class: "support-desk",
            div { class: "motion-cascade motion-cascade-tight support-stack",
                if matched == 0 {
                    div { class: "support-stack-empty",
                        p { class: "text-sm font-medium text-text", "Inbox is quiet" }
                        p { class: "mt-1 text-sm text-text-muted", "Nothing matches these filters." }
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
                            "Show older · {remaining} left"
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
                                    p { class: "support-reading-kicker", "Case #{ticket.id}" }
                                    h2 { class: "support-reading-title", "{ticket.subject}" }
                                }
                                StatusChip { label: ticket.status.label(), tone: ticket.status.tone() }
                            }

                            div { class: "support-reading-tags",
                                span { class: "support-tag", style: "--tag-accent: {tone};", "{ticket.priority}" }
                                span { class: "support-tag", "{ticket.department}" }
                                span { class: "support-tag", "{ticket.assignee}" }
                                span { class: "support-tag is-muted", "Opened {ticket.when} ago" }
                            }

                            div { class: "mt-5 flex items-center gap-3",
                                Avatar { email: ticket.email.clone(), size: 36, alt: ticket.player.clone() }
                                div { class: "min-w-0",
                                    p { class: "truncate text-sm font-medium text-text", "{ticket.player}" }
                                    p { class: "truncate text-xs text-text-muted", "{ticket.email}" }
                                }
                            }

                            div { class: "mt-5",
                                p { class: "text-xs text-text-muted", "Latest note" }
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
                                    "Open ticket"
                                }
                            }
                        }
                    }
                } else {
                    div { class: "support-reading-empty",
                        p { class: "text-sm font-medium text-text", "Pick a case" }
                        p { class: "mt-1 max-w-xs text-sm text-text-muted",
                            "Select a ticket from the stack to preview details, then open it."
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SupportTicket(id: u64) -> Element {
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
                p { class: "text-sm text-text-muted", "Desk" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight", "Ticket not found" }
                p { class: "mt-2 text-sm text-text-secondary", "That case is no longer in the inbox." }
                div { class: "mt-6",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            navigator.push(Route::SupportTickets {});
                        },
                        "Back to inbox"
                    }
                }
            }
        };
    };

    let tone = priority_tone(&ticket.priority);
    let reply_placeholder = format!("Write a reply to {}…", ticket.player);

    rsx! {
        div { class: "mb-2 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Desk · Case #{ticket.id}" }
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
                    "Back to inbox"
                }
            }
        }

        p { class: "mb-8 text-sm text-text-muted",
            "Opened {ticket.when} ago · {ticket.department} · {ticket.priority} priority · {ticket.assignee}"
        }

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
                            p { class: "truncate text-xs text-text-muted", "{ticket.email}" }
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
                            "Send reply"
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            "Assign"
                        }
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Ghost,
                            "Internal note"
                        }
                    }
                }
            }

            aside { class: "support-ticket-side",
                p { class: "text-xs font-medium uppercase tracking-wide text-text-muted",
                    "Case details"
                }
                div { class: "support-reading-tags mt-4",
                    span { class: "support-tag", style: "--tag-accent: {tone};", "{ticket.priority}" }
                    span { class: "support-tag", "{ticket.department}" }
                    span { class: "support-tag", "{ticket.assignee}" }
                    span { class: "support-tag is-muted", "Opened {ticket.when} ago" }
                }
                div { class: "mt-6 space-y-4",
                    div {
                        p { class: "text-xs text-text-muted", "Player" }
                        p { class: "mt-1 text-sm font-medium text-text", "{ticket.player}" }
                    }
                    div {
                        p { class: "text-xs text-text-muted", "Email" }
                        p { class: "mt-1 text-sm font-medium text-text break-all",
                            "{ticket.email}"
                        }
                    }
                    div {
                        p { class: "text-xs text-text-muted", "Status" }
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

    fn label(self) -> &'static str {
        match self {
            Self::Online => "Online",
            Self::Away => "Away",
            Self::Offline => "Offline",
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
            ticket.assignee == "Unassigned"
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

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Live desk" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "Support"
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary",
                    "Open cases, staff coverage, and where automation is doing the work."
                }
            }
            Button { variant: ButtonVariant::Secondary, "View on website" }
        }

        div { class: "motion-cascade ops-meter",
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", "Open now" }
                p { class: "ops-meter-value is-accent", "{open_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", "Pending reply" }
                p { class: "ops-meter-value", "{pending_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", "Unassigned" }
                p { class: "ops-meter-value", "{unassigned_count}" }
            }
            span { class: "ops-meter-divider" }
            div { class: "ops-meter-item",
                p { class: "ops-meter-label", "Avg. first reply" }
                p { class: "ops-meter-value", "14m" }
            }
        }

        section { class: "mb-9",
            div { class: "ops-section-head",
                h2 { class: "ops-section-title", "Open right now" }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        navigator.push(Route::SupportTickets {});
                    },
                    "View inbox →"
                }
            }
            if open_now.is_empty() {
                p { class: "ops-empty", "Nothing waiting on staff right now." }
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
                h2 { class: "ops-section-title", "Staff on the desk" }
                p { class: "ops-section-sub", "{online_count} of {staff_len} online" }
            }
            div { class: "motion-cascade motion-cascade-tight ops-roster",
                for member in staff {
                    {
                        let load_pct = if member.capacity == 0 {
                            0
                        } else {
                            (member.active_tickets * 100 / member.capacity).min(100)
                        };
                        rsx! {
                            div { class: "ops-roster-row", key: "{member.email}",
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
                                span { class: "ops-roster-load-label", "{member.active_tickets}/{member.capacity} tickets" }
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
                        p { class: "ops-pulse-title", "Help centre" }
                        p { class: "ops-pulse-meta",
                            "{article_count} articles · {helpful_avg}% helpful"
                        }
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
                        p { class: "ops-pulse-title", "Automation" }
                        p { class: "ops-pulse-meta", "{live_rules} rules live" }
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
    fn label(self) -> &'static str {
        match self {
            Self::Published => "Published",
            Self::Featured => "Featured",
            Self::Draft => "Draft",
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
    let articles = use_context::<Signal<Vec<Article>>>();
    let navigator = use_navigator();
    let all = articles();

    let total = all.len();
    let helpful_avg = if total == 0 {
        0
    } else {
        all.iter().map(|a| a.helpful_pct).sum::<u32>() / total as u32
    };

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Knowledge base" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "Help centre"
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary",
                    "{total} articles across {HELP_CATEGORIES.len()} shelves · {helpful_avg}% marked helpful."
                }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::SupportHelpNew {});
                },
                IconPlus {}
                "New article"
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
                                span { class: "shelf-count", "{items.len()} articles" }
                            }
                            div { class: "shelf-rail",
                                for article in items {
                                    {
                                        let article_id = article.id;
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
                                                    span { class: "shelf-card-views", "{article.views} views" }
                                                }
                                                h3 { class: "shelf-card-title", "{article.title}" }
                                                p { class: "shelf-card-snippet", "{article.snippet}" }
                                                div { class: "shelf-card-foot", "Updated {article.updated} · {article.helpful_pct}% helpful" }
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
    rsx! {
        ArticleEditor { article_id: None }
    }
}

#[component]
pub fn SupportHelpEdit(id: u64) -> Element {
    rsx! {
        ArticleEditor { article_id: Some(id) }
    }
}

#[component]
fn ArticleEditor(article_id: Option<u64>) -> Element {
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
                    "← Help centre"
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight", "Article not found" }
            p { class: "mt-2 text-sm text-text-muted", "This article may have been deleted." }
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
        String::from("Article title")
    } else {
        title_now.trim().to_string()
    };
    let preview_snippet = if snippet_now.trim().is_empty() {
        String::from("A short summary shown on the shelf card.")
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
                    "← Help centre"
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
                            "Delete"
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            navigator.push(Route::SupportHelpCentre {});
                        },
                        "Cancel"
                    }
                    Button {
                        size: ButtonSize::Sm,
                        disabled: !can_save,
                        onclick: save,
                        if is_new {
                            "Publish article"
                        } else {
                            "Save changes"
                        }
                    }
                }
            }

            div { class: "mb-8",
                p { class: "text-sm text-text-muted", "Knowledge base" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight",
                    if is_new {
                        "New article"
                    } else {
                        "Edit article"
                    }
                }
                p { class: "mt-2 max-w-2xl text-sm text-text-muted",
                    "Title, shelf, body, and whether it's ready for players to see."
                }
            }

            div { class: "motion-cascade forum-editor-layout",
                div { class: "forum-editor-main space-y-8",
                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Basics" }
                        p { class: "forum-editor-lede", "Title and the shelf this article lives on." }
                        div { class: "mt-4 space-y-4",
                            Field { label: "Title",
                                SignalInput {
                                    value: title,
                                    placeholder: "How to claim vote rewards",
                                }
                            }
                            Field { label: "Shelf",
                                SignalSelect {
                                    value: category,
                                    options: HELP_CATEGORIES
                                                                            .iter()
                                                                            .map(|c| SelectOption::new(*c, *c))
                                                                            .collect(),
                                }
                            }
                            Field {
                                label: "Snippet",
                                hint: "Shown on the shelf card. Leave blank to use the start of the body.",
                                SignalTextarea {
                                    value: snippet,
                                    placeholder: "One or two sentences…",
                                    class: "min-h-[4.5rem]",
                                }
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Content" }
                        p { class: "forum-editor-lede", "The full article body. Markdown supported." }
                        div { class: "mt-4",
                            SignalTextarea {
                                value: body,
                                placeholder: "Write the article…",
                                class: "min-h-[14rem]",
                            }
                        }
                    }

                    section { class: "forum-editor-section",
                        h2 { class: "forum-editor-heading", "Status" }
                        p { class: "forum-editor-lede",
                            "Drafts sit in the proof queue until you publish them."
                        }
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
                            "Shelf preview"
                        }
                        div {
                            class: "mt-4 shelf-card",
                            style: "cursor: default; width: 100%;",
                            div { class: "shelf-card-top",
                                StatusChip {
                                    label: status_now.label(),
                                    tone: status_now.tone(),
                                }
                                span { class: "shelf-card-views", "0 views" }
                            }
                            h3 { class: "shelf-card-title", "{preview_title}" }
                            p { class: "shelf-card-snippet", "{preview_snippet}" }
                            div { class: "shelf-card-foot", "{category_now} · just now" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct AutomationRuleMeta {
    name: &'static str,
    description: &'static str,
    category: &'static str,
    tone: &'static str,
    trigger: &'static str,
}

const AUTOMATION_RULES: &[AutomationRuleMeta] = &[
    AutomationRuleMeta {
        name: "AI first reply",
        description: "Draft a reply from help centre articles before a human responds.",
        category: "First reply",
        tone: "#f0a35e",
        trigger: "When a ticket opens with no staff reply yet",
    },
    AutomationRuleMeta {
        name: "Discord alert",
        description: "Post new tickets to #support-alerts as soon as they land.",
        category: "Notifications",
        tone: "#5b9dff",
        trigger: "When any ticket status changes to Open",
    },
    AutomationRuleMeta {
        name: "SLA breach ping",
        description: "Ping the on-call staff member if a high priority ticket goes unassigned.",
        category: "Escalation",
        tone: "#f87171",
        trigger: "When a High priority ticket waits 15 minutes",
    },
    AutomationRuleMeta {
        name: "CSAT follow-up",
        description: "Send a one-question satisfaction survey after a ticket resolves.",
        category: "Notifications",
        tone: "#5b9dff",
        trigger: "When a ticket status changes to Resolved",
    },
    AutomationRuleMeta {
        name: "Auto-close idle tickets",
        description: "Close tickets automatically after a week without a reply.",
        category: "Cleanup",
        tone: "#858899",
        trigger: "When a ticket sits idle for 7 days",
    },
    AutomationRuleMeta {
        name: "Duplicate merge suggestion",
        description: "Flag likely duplicate tickets opened by the same player minutes apart.",
        category: "Cleanup",
        tone: "#858899",
        trigger: "When a player opens a second ticket within 10 minutes",
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
    let mut enabled = use_signal(|| vec![true, true, true, true, false, false]);
    let enabled_now = enabled();
    let live_count = enabled_now.iter().filter(|on| **on).count();

    rsx! {
        div { class: "mb-8 flex flex-wrap items-end justify-between gap-4",
            div { class: "min-w-0",
                p { class: "text-sm text-text-muted", "Playbook" }
                h1 { class: "mt-1 text-3xl font-semibold tracking-tight sm:text-4xl",
                    "Automation"
                }
                p { class: "mt-2 max-w-xl text-sm text-text-secondary",
                    "{live_count} of {AUTOMATION_RULES.len()} rules live. Flip a track on or off — the AI preview reacts."
                }
            }
            Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary,
                IconAi {}
                "Configure AI"
            }
        }

        div { class: "auto-layout",
            div { class: "motion-cascade auto-rails",
                for (index, rule) in AUTOMATION_RULES.iter().enumerate() {
                    {
                        let on = enabled_now[index];
                        rsx! {
                            div {
                                class: "auto-rail",
                                key: "{rule.name}",
                                style: "--rail-tone: {rule.tone};",
                                span { class: "auto-rail-bar" }
                                div { class: "auto-rail-body",
                                    div { class: "auto-rail-copy",
                                        div { class: "auto-rail-top",
                                            p { class: "auto-rail-name", "{rule.name}" }
                                            span { class: "auto-rail-tag", "{rule.category}" }
                                        }
                                        p { class: "auto-rail-desc", "{rule.description}" }
                                        p { class: "auto-rail-trigger", "{rule.trigger}" }
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
                p { class: "auto-preview-kicker", "AI reply preview" }
                div { class: "auto-preview-ticket",
                    p { class: "auto-preview-ticket-label", "Incoming · NovaCraft" }
                    p { class: "auto-preview-ticket-text",
                        "“I paid for VIP through PayPal but the rank never showed up in-game. Can someone check?”"
                    }
                }
                div { class: "auto-preview-reply",
                    span { class: "auto-preview-badge",
                        IconAi { class: "h-3.5 w-3.5" }
                        "AI drafted"
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
                        span { class: "auto-preview-meter-label", "82% confidence" }
                    }
                }
                div { class: "auto-preview-actions",
                    Button { size: ButtonSize::Sm, "Use reply" }
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Secondary,
                        "Regenerate"
                    }
                }
            }
        }
    }
}

#[component]
pub fn SupportSiteSettings() -> Element {
    let public_path = use_signal(|| String::from("/support"));
    let page_title = use_signal(|| String::from("Support"));
    let department = use_signal(|| String::from("Store"));
    let priority = use_signal(|| String::from("Normal"));

    rsx! {
        FeatureSettingsChrome { subtitle: "Path, ticket defaults, and how the desk hands off to the help centre and automation.",
            DataPanel { title: "Support path",
                SettingsControl { label: "Public path",
                    SignalInput {
                        value: public_path,
                        placeholder: "/support".to_string(),
                    }
                }
                SettingsField {
                    label: "Full URL",
                    value: format!("www.example.com{}", public_path()),
                }
                SettingsControl { label: "Page title",
                    SignalInput { value: page_title, placeholder: "Support".to_string() }
                }
                p { class: "pt-3 text-xs text-text-muted",
                    "Domain and HTTPS are managed in Settings → General."
                }
            }
            DataPanel { title: "Ticket defaults",
                SettingsControl { label: "Default department",
                    SignalSelect {
                        value: department,
                        options: ["Store", "Gameplay", "Moderation", "Account"]
                            .into_iter()
                            .map(|name| SelectOption::new(name, name))
                            .collect(),
                    }
                }
                SettingsControl { label: "Default priority",
                    SignalSelect {
                        value: priority,
                        options: ["Low", "Normal", "High"]
                            .into_iter()
                            .map(|name| SelectOption::new(name, name))
                            .collect(),
                    }
                }
                SettingRow {
                    title: "Business hours only",
                    description: "Show players an away message outside your staffed hours.",
                    enabled: false,
                }
            }
            DataPanel { title: "Help centre",
                SettingRow {
                    title: "Public search",
                    description: "Let players search articles without logging in.",
                    enabled: true,
                }
                SettingRow {
                    title: "Ask for feedback",
                    description: "Show a helpful / not helpful prompt at the end of articles.",
                    enabled: true,
                }
            }
            DataPanel { title: "Automation defaults",
                SettingRow {
                    title: "AI first reply",
                    description: "Draft a reply from help centre articles before a human responds.",
                    enabled: true,
                }
                SettingRow {
                    title: "Discord alerts",
                    description: "Post new tickets to your staff Discord channel.",
                    enabled: true,
                }
            }
        }
    }
}
