use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::page::StatPill;
use crate::components::settings::{IconKey, SectionIntro, ToggleField};
use crate::components::ui::*;
use crate::i18n::t_key;
use crate::router::Route;

const AUTH_ACCENT: &str = "#5b9dff";
const STAFF_ACCENT: &str = "#69bdf2";
const ROLES_ACCENT: &str = "#fb923c";

struct AuthLane {
    name_key: &'static str,
    desc_key: &'static str,
    usage_pct: u8,
    enabled: bool,
    accent: &'static str,
}

const AUTH_LANES: &[AuthLane] = &[
    AuthLane {
        name_key: "accounts-auth-lane-password-name",
        desc_key: "accounts-auth-lane-password-desc",
        usage_pct: 100,
        enabled: true,
        accent: "#5b9dff",
    },
    AuthLane {
        name_key: "accounts-auth-lane-2fa-name",
        desc_key: "accounts-auth-lane-2fa-desc",
        usage_pct: 62,
        enabled: true,
        accent: "#3ecf8e",
    },
    AuthLane {
        name_key: "accounts-auth-lane-magic-link-name",
        desc_key: "accounts-auth-lane-magic-link-desc",
        usage_pct: 24,
        enabled: true,
        accent: "#87d1fe",
    },
    AuthLane {
        name_key: "accounts-auth-lane-saml-name",
        desc_key: "accounts-auth-lane-saml-desc",
        usage_pct: 4,
        enabled: false,
        accent: "#c4b5fd",
    },
];

#[component]
pub fn AccountsAuth() -> Element {
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {AUTH_ACCENT};",
            SectionIntro {
                eyebrow: t_key("accounts-auth-eyebrow"),
                title: t_key("accounts-auth-title"),
                description: t_key("accounts-auth-description"),
                accent: AUTH_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        { t!("accounts-auth-invite-user") }
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: t_key("accounts-auth-stat-accounts"), value: "3,481", accent: AUTH_ACCENT }
                StatPill { label: t_key("accounts-auth-stat-2fa-enabled"), value: "62%", accent: AUTH_ACCENT }
                StatPill {
                    label: t_key("accounts-auth-stat-oauth-logins"),
                    value: "842",
                    accent: AUTH_ACCENT,
                }
                StatPill {
                    label: t_key("accounts-auth-stat-linked-players"),
                    value: "1,204",
                    accent: AUTH_ACCENT,
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-auth-signin-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-auth-signin-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "motion-cascade acct-lanes",
                        for lane in AUTH_LANES {
                            div {
                                class: "acct-lane",
                                style: "--lane-accent: {lane.accent};",
                                span { class: "acct-lane-bar" }
                                span { class: "acct-lane-icon", IconKey {} }
                                div { class: "acct-lane-body",
                                    p { class: "acct-lane-title", { t_key(lane.name_key) } }
                                    p { class: "acct-lane-desc", { t_key(lane.desc_key) } }
                                }
                                div { class: "acct-lane-usage",
                                    div { class: "acct-lane-usage-track",
                                        div {
                                            class: "acct-lane-usage-fill",
                                            style: "width: {lane.usage_pct}%;",
                                        }
                                    }
                                    p { class: "acct-lane-usage-label",
                                        { t!("accounts-auth-usage-pct", pct: lane.usage_pct) }
                                    }
                                }
                                ToggleField { enabled: lane.enabled }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-auth-connections-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-auth-connections-sub") }
                        }
                    }
                    Button { size: ButtonSize::Sm,
                        IconPlus {}
                        { t!("accounts-auth-add-provider") }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "motion-cascade acct-oauth-grid",
                        for provider in OAUTH_PROVIDERS {
                            div {
                                class: "acct-oauth-card",
                                style: "--oauth-accent: {provider.accent};",
                                span { class: "acct-oauth-icon",
                                    OauthBrandIcon { id: provider.id }
                                }
                                div { class: "min-w-0",
                                    p { class: "acct-oauth-name", "{provider.name}" }
                                    p { class: "acct-oauth-count", "{provider.count}" }
                                }
                                span { class: if provider.up { "acct-oauth-trend is-up" } else { "acct-oauth-trend" },
                                    "{provider.trend}"
                                }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-auth-linking-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-auth-linking-sub") }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        { t!("accounts-auth-generate-codes") }
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "motion-cascade acct-flow mb-5",
                        AcctFlowStep {
                            num: "1",
                            title: t_key("accounts-auth-flow-step1-title"),
                            desc: t_key("accounts-auth-flow-step1-desc"),
                            connector: true,
                        }
                        AcctFlowStep {
                            num: "2",
                            title: t_key("accounts-auth-flow-step2-title"),
                            desc: t_key("accounts-auth-flow-step2-desc"),
                            connector: true,
                        }
                        AcctFlowStep {
                            num: "3",
                            title: t_key("accounts-auth-flow-step3-title"),
                            desc: t_key("accounts-auth-flow-step3-desc"),
                            connector: false,
                        }
                    }
                    div { class: "mb-2 flex items-center justify-between gap-3",
                        h3 { class: "text-sm font-semibold text-text", { t!("accounts-auth-recent-links") } }
                        span { class: "text-xs text-text-muted", "1,204 linked · 18 pending" }
                    }
                    div { class: "motion-cascade motion-cascade-tight acct-link-list",
                        AcctLinkRow {
                            badge: "MC",
                            title: "NovaCraft · Survival",
                            meta: "Minecraft · Verified with code N7K2",
                            status_key: "accounts-auth-status-linked",
                        }
                        AcctLinkRow {
                            badge: "MC",
                            title: "SkyBuilder · Skyblock",
                            meta: "Minecraft · Verified with code Q1M9",
                            status_key: "accounts-auth-status-linked",
                        }
                        AcctLinkRow {
                            badge: "MC",
                            title: "AetherFox · Creative",
                            meta: t_key("accounts-auth-link-meta-awaiting"),
                            status_key: "accounts-auth-status-pending",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AcctFlowStep(
    num: &'static str,
    #[props(into)] title: String,
    #[props(into)] desc: String,
    #[props(default)] connector: bool,
) -> Element {
    rsx! {
        div { class: "acct-flow-step",
            span { class: "acct-flow-step-num", "{num}" }
            p { class: "acct-flow-step-title", "{title}" }
            p { class: "acct-flow-step-desc", "{desc}" }
            if connector {
                span { class: "acct-flow-connector" }
            }
        }
    }
}

#[component]
fn AcctLinkRow(
    badge: &'static str,
    #[props(into)] title: String,
    #[props(into)] meta: String,
    status_key: &'static str,
) -> Element {
    rsx! {
        div { class: "acct-link-row",
            span { class: "acct-link-badge", "{badge}" }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-sm font-medium text-text", "{title}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
            }
            span { class: "shrink-0 text-xs text-text-secondary", { t_key(status_key) } }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum OauthProviderId {
    Discord,
    Steam,
    Microsoft,
    Google,
    GitHub,
    Custom,
}

struct OauthCard {
    id: OauthProviderId,
    name: &'static str,
    count: &'static str,
    trend: &'static str,
    up: bool,
    accent: &'static str,
}

const OAUTH_PROVIDERS: &[OauthCard] = &[
    OauthCard {
        id: OauthProviderId::Discord,
        name: "Discord",
        count: "842 logins",
        trend: "+12%",
        up: true,
        accent: "#5865F2",
    },
    OauthCard {
        id: OauthProviderId::Steam,
        name: "Steam",
        count: "611 logins",
        trend: "+4%",
        up: true,
        accent: "#66c0f4",
    },
    OauthCard {
        id: OauthProviderId::Microsoft,
        name: "Microsoft",
        count: "203 logins",
        trend: "flat",
        up: false,
        accent: "#00A4EF",
    },
    OauthCard {
        id: OauthProviderId::Google,
        name: "Google",
        count: "188 logins",
        trend: "+2%",
        up: true,
        accent: "#4285F4",
    },
    OauthCard {
        id: OauthProviderId::GitHub,
        name: "GitHub",
        count: "34 logins",
        trend: "flat",
        up: false,
        accent: "#8b949e",
    },
    OauthCard {
        id: OauthProviderId::Custom,
        name: "Custom OAuth",
        count: "2 apps",
        trend: "new",
        up: true,
        accent: "#818cf8",
    },
];

#[component]
fn OauthBrandIcon(id: OauthProviderId) -> Element {
    match id {
        OauthProviderId::Discord => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.09.09 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.09 16.09 0 0 0-4.8 0c-.14-.34-.37-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.02.06.03.09.02c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02zM8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12zm6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12z" }
            }
        },
        OauthProviderId::Steam => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M11.979 0C5.678 0 .511 4.86.022 11.122l6.432 2.658c.545-.371 1.203-.59 1.912-.59.063 0 .125.004.188.006l2.861-4.142V8.91c0-2.495 2.028-4.524 4.524-4.524 2.494 0 4.524 2.031 4.524 4.527s-2.03 4.525-4.524 4.525h-.105l-4.076 2.911c0 .052.004.105.004.159 0 1.875-1.515 3.396-3.39 3.396-1.635 0-3.016-1.173-3.331-2.727L.436 15.27C1.962 20.607 6.534 24 11.979 24c6.624 0 11.999-5.375 11.999-12S18.603 0 11.979 0zM7.54 18.21l-1.473-.61c.262.543.714.999 1.314 1.25 1.297.539 2.793-.076 3.332-1.375.263-.628.264-1.319.005-1.949s-.75-1.121-1.377-1.383c-.624-.26-1.29-.249-1.878-.03l1.523.63c.956.4 1.409 1.5 1.009 2.455-.397.957-1.497 1.41-2.454 1.012H7.54zm11.415-9.303c0-1.662-1.353-3.015-3.015-3.015-1.665 0-3.015 1.353-3.015 3.015 0 1.665 1.35 3.015 3.015 3.015 1.663 0 3.015-1.35 3.015-3.015zm-5.273-.005c0-1.252 1.013-2.266 2.266-2.266 1.249 0 2.266 1.014 2.266 2.266 0 1.251-1.017 2.265-2.266 2.265-1.253 0-2.266-1.014-2.266-2.265z" }
            }
        },
        OauthProviderId::Microsoft => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                rect {
                    x: "1",
                    y: "1",
                    width: "10",
                    height: "10",
                    fill: "#f25022",
                }
                rect {
                    x: "13",
                    y: "1",
                    width: "10",
                    height: "10",
                    fill: "#7fba00",
                }
                rect {
                    x: "1",
                    y: "13",
                    width: "10",
                    height: "10",
                    fill: "#00a4ef",
                }
                rect {
                    x: "13",
                    y: "13",
                    width: "10",
                    height: "10",
                    fill: "#ffb900",
                }
            }
        },
        OauthProviderId::Google => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                path {
                    fill: "#4285F4",
                    d: "M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z",
                }
                path {
                    fill: "#34A853",
                    d: "M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z",
                }
                path {
                    fill: "#FBBC05",
                    d: "M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z",
                }
                path {
                    fill: "#EA4335",
                    d: "M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z",
                }
            }
        },
        OauthProviderId::GitHub => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                fill: "currentColor",
                "aria-hidden": "true",
                path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" }
            }
        },
        OauthProviderId::Custom => rsx! {
            svg {
                class: "acct-oauth-brand",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.75",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                "aria-hidden": "true",
                path { d: "M15.5 7.5l1 1a3.5 3.5 0 0 1 0 5l-1.2 1.2a3.5 3.5 0 0 1-5 0l-.3-.3" }
                path { d: "M8.5 16.5l-1-1a3.5 3.5 0 0 1 0-5l1.2-1.2a3.5 3.5 0 0 1 5 0l.3.3" }
            }
        },
    }
}

struct StaffMember {
    initials: &'static str,
    name: &'static str,
    username: &'static str,
    role_key: &'static str,
    role_accent: &'static str,
    last_active: &'static str,
    status_key: &'static str,
}

const STAFF_MEMBERS: &[StaffMember] = &[
    StaffMember {
        initials: "AC",
        name: "Alex Chen",
        username: "alex",
        role_key: "accounts-staff-role-owner",
        role_accent: "#fb7185",
        last_active: "Just now",
        status_key: "accounts-staff-status-online",
    },
    StaffMember {
        initials: "JR",
        name: "Jordan Reyes",
        username: "jordan",
        role_key: "accounts-staff-role-admin",
        role_accent: "#fb923c",
        last_active: "12 min ago",
        status_key: "accounts-staff-status-online",
    },
    StaffMember {
        initials: "MK",
        name: "Maya Khan",
        username: "maya",
        role_key: "accounts-staff-role-admin",
        role_accent: "#fb923c",
        last_active: "2 hours ago",
        status_key: "accounts-staff-status-away",
    },
    StaffMember {
        initials: "TS",
        name: "Theo Santos",
        username: "theo",
        role_key: "accounts-staff-role-moderator",
        role_accent: "#5b9dff",
        last_active: "Yesterday",
        status_key: "accounts-staff-status-offline",
    },
    StaffMember {
        initials: "LP",
        name: "Lena Park",
        username: "lena",
        role_key: "accounts-staff-role-moderator",
        role_accent: "#5b9dff",
        last_active: "3 days ago",
        status_key: "accounts-staff-status-offline",
    },
];

struct StaffInvite {
    username: &'static str,
    role_key: &'static str,
    sent: &'static str,
}

const STAFF_INVITES: &[StaffInvite] = &[
    StaffInvite {
        username: "sam",
        role_key: "accounts-staff-role-moderator",
        sent: "2 hours ago",
    },
    StaffInvite {
        username: "ria",
        role_key: "accounts-staff-role-admin",
        sent: "Yesterday",
    },
];

#[component]
pub fn AccountsStaff() -> Element {
    let _lang = i18n();

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {STAFF_ACCENT};",
            SectionIntro {
                eyebrow: t_key("accounts-staff-eyebrow"),
                title: t_key("accounts-staff-title"),
                description: t_key("accounts-staff-description"),
                accent: STAFF_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        { t!("accounts-staff-invite-staff") }
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: t_key("accounts-staff-stat-active"), value: "5", accent: STAFF_ACCENT }
                StatPill {
                    label: t_key("accounts-staff-stat-pending-invites"),
                    value: "2",
                    accent: STAFF_ACCENT,
                }
                StatPill { label: t_key("accounts-staff-stat-admins"), value: "3", accent: STAFF_ACCENT }
                StatPill {
                    label: t_key("accounts-staff-stat-2fa-coverage"),
                    value: "100%",
                    accent: STAFF_ACCENT,
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-staff-roster-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-staff-roster-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body stg-site-panel-body-flush",
                    div { class: "motion-cascade motion-cascade-tight acct-staff-list",
                        for member in STAFF_MEMBERS {
                            div { class: "acct-staff-row",
                                span {
                                    class: "acct-staff-avatar",
                                    style: "--staff-accent: {member.role_accent};",
                                    "{member.initials}"
                                }
                                div { class: "min-w-0 flex-1",
                                    p { class: "acct-staff-name", "{member.name}" }
                                    p { class: "acct-staff-email", "@{member.username}" }
                                }
                                span {
                                    class: "acct-staff-role",
                                    style: "--staff-accent: {member.role_accent};",
                                    { t_key(member.role_key) }
                                }
                                span { class: "acct-staff-meta", "{member.last_active}" }
                                span { class: if member.status_key == "accounts-staff-status-online" { "acct-staff-status is-online" } else if member.status_key == "accounts-staff-status-away" { "acct-staff-status is-away" } else { "acct-staff-status" },
                                    { t_key(member.status_key) }
                                }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-staff-pending-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-staff-pending-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body stg-site-panel-body-flush",
                    div { class: "motion-cascade motion-cascade-tight acct-staff-list",
                        for invite in STAFF_INVITES {
                            div { class: "acct-staff-row",
                                span { class: "acct-staff-avatar is-invite", "··" }
                                div { class: "min-w-0 flex-1",
                                    p { class: "acct-staff-name", "@{invite.username}" }
                                    p { class: "acct-staff-email",
                                        { t!("accounts-staff-invited-as", role: t_key(invite.role_key)) }
                                    }
                                }
                                span { class: "acct-staff-meta",
                                    { t!("accounts-staff-sent", when: invite.sent) }
                                }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Sm,
                                    { t!("common-resend") }
                                }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", { t!("accounts-staff-access-title") } }
                        p { class: "stg-site-panel-sub",
                            { t!("accounts-staff-access-sub") }
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    ToggleField {
                        label: t_key("accounts-staff-toggle-2fa-label"),
                        hint: t_key("accounts-staff-toggle-2fa-hint"),
                        enabled: true,
                    }
                    ToggleField {
                        label: t_key("accounts-staff-toggle-ip-label"),
                        hint: t_key("accounts-staff-toggle-ip-hint"),
                        enabled: false,
                    }
                    ToggleField {
                        label: t_key("accounts-staff-toggle-notify-label"),
                        hint: t_key("accounts-staff-toggle-notify-hint"),
                        enabled: true,
                    }
                }
            }
        }
    }
}

struct RoleRung {
    name_key: &'static str,
    meta_key: &'static str,
    level: u8,
    members_key: &'static str,
    accent: &'static str,
}

const ROLE_LADDER: &[RoleRung] = &[
    RoleRung {
        name_key: "accounts-roles-role-owner",
        meta_key: "accounts-roles-meta-owner",
        level: 100,
        members_key: "accounts-roles-members-owner",
        accent: "#fb7185",
    },
    RoleRung {
        name_key: "accounts-roles-role-admin",
        meta_key: "accounts-roles-meta-admin",
        level: 80,
        members_key: "accounts-roles-members-admin",
        accent: "#fb923c",
    },
    RoleRung {
        name_key: "accounts-roles-role-moderator",
        meta_key: "accounts-roles-meta-moderator",
        level: 40,
        members_key: "accounts-roles-members-moderator",
        accent: "#f5c14a",
    },
    RoleRung {
        name_key: "accounts-roles-role-helper",
        meta_key: "accounts-roles-meta-helper",
        level: 20,
        members_key: "accounts-roles-members-helper",
        accent: "#5b9dff",
    },
    RoleRung {
        name_key: "accounts-roles-role-member",
        meta_key: "accounts-roles-meta-member",
        level: 1,
        members_key: "accounts-roles-members-member",
        accent: "#87d1fe",
    },
];

const PERMISSIONS: &[&str] = &[
    // Dashboard
    "dashboard.view",
    // Store
    "store.view",
    "store.products.manage",
    "store.orders.manage",
    "store.orders.refund",
    "store.coupons.manage",
    "store.categories.manage",
    "store.settings",
    // Forums
    "forums.view",
    "forums.create",
    "forums.edit",
    "forums.delete",
    "forums.moderate",
    "forums.boards.manage",
    "forums.automod",
    "forums.settings",
    // Support
    "tickets.view",
    "tickets.reply",
    "tickets.assign",
    "tickets.manage",
    "tickets.close",
    "help.view",
    "help.manage",
    "support.automation",
    "support.settings",
    // Content / news
    "news.view",
    "news.draft",
    "news.publish",
    "news.delete",
    "news.settings",
    // Community — players
    "players.view",
    "players.manage",
    "players.ban",
    "players.settings",
    // Community — leaderboards
    "leaderboards.view",
    "leaderboards.manage",
    "leaderboards.settings",
    // Community — votes
    "votes.view",
    "votes.manage",
    "votes.settings",
    // Community — applications
    "applications.view",
    "applications.review",
    "applications.manage",
    "applications.settings",
    // Analytics
    "analytics.view",
    "analytics.export",
    "analytics.settings",
    // Accounts & access
    "accounts.view",
    "accounts.manage",
    "accounts.invite",
    "roles.view",
    "roles.manage",
    // Settings & theme
    "settings.view",
    "settings.general",
    "settings.localisation",
    "settings.integrations",
    "theme.view",
    "theme.edit",
];

// Columns: Owner, Admin, Moderator, Helper, Member
const GRANTS: &[[bool; 5]] = &[
    // dashboard.view
    [true, true, true, true, false],
    // store.view
    [true, true, true, false, false],
    // store.products.manage
    [true, true, false, false, false],
    // store.orders.manage
    [true, true, true, false, false],
    // store.orders.refund
    [true, true, false, false, false],
    // store.coupons.manage
    [true, true, false, false, false],
    // store.categories.manage
    [true, true, false, false, false],
    // store.settings
    [true, true, false, false, false],
    // forums.view
    [true, true, true, true, true],
    // forums.create
    [true, true, true, true, true],
    // forums.edit
    [true, true, true, false, false],
    // forums.delete
    [true, true, true, false, false],
    // forums.moderate
    [true, true, true, true, false],
    // forums.boards.manage
    [true, true, false, false, false],
    // forums.automod
    [true, true, true, false, false],
    // forums.settings
    [true, true, false, false, false],
    // tickets.view
    [true, true, true, true, false],
    // tickets.reply
    [true, true, true, true, false],
    // tickets.assign
    [true, true, true, false, false],
    // tickets.manage
    [true, true, true, false, false],
    // tickets.close
    [true, true, true, true, false],
    // help.view
    [true, true, true, true, true],
    // help.manage
    [true, true, true, false, false],
    // support.automation
    [true, true, false, false, false],
    // support.settings
    [true, true, false, false, false],
    // news.view
    [true, true, true, true, true],
    // news.draft
    [true, true, true, false, false],
    // news.publish
    [true, true, false, false, false],
    // news.delete
    [true, true, false, false, false],
    // news.settings
    [true, true, false, false, false],
    // players.view
    [true, true, true, true, false],
    // players.manage
    [true, true, true, false, false],
    // players.ban
    [true, true, true, false, false],
    // players.settings
    [true, true, false, false, false],
    // leaderboards.view
    [true, true, true, true, true],
    // leaderboards.manage
    [true, true, false, false, false],
    // leaderboards.settings
    [true, true, false, false, false],
    // votes.view
    [true, true, true, false, false],
    // votes.manage
    [true, true, false, false, false],
    // votes.settings
    [true, true, false, false, false],
    // applications.view
    [true, true, true, true, false],
    // applications.review
    [true, true, true, false, false],
    // applications.manage
    [true, true, false, false, false],
    // applications.settings
    [true, true, false, false, false],
    // analytics.view
    [true, true, true, false, false],
    // analytics.export
    [true, true, false, false, false],
    // analytics.settings
    [true, true, false, false, false],
    // accounts.view
    [true, true, false, false, false],
    // accounts.manage
    [true, true, false, false, false],
    // accounts.invite
    [true, true, false, false, false],
    // roles.view
    [true, true, false, false, false],
    // roles.manage
    [true, false, false, false, false],
    // settings.view
    [true, true, false, false, false],
    // settings.general
    [true, true, false, false, false],
    // settings.localisation
    [true, true, false, false, false],
    // settings.integrations
    [true, true, false, false, false],
    // theme.view
    [true, true, false, false, false],
    // theme.edit
    [true, true, false, false, false],
];

const _: () = assert!(PERMISSIONS.len() == GRANTS.len());

#[component]
pub fn AccountsRoles() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let max_level = 100f32;

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {ROLES_ACCENT};",
            SectionIntro {
                eyebrow: t_key("accounts-roles-eyebrow"),
                title: t_key("accounts-roles-title"),
                description: t_key("accounts-roles-description"),
                accent: ROLES_ACCENT,
                action: rsx! {
                    Button {
                        onclick: move |_| {
                            navigator.push(Route::AccountsRoleNew {});
                        },
                        IconPlus {}
                        { t!("accounts-roles-create-role") }
                    }
                },
            }

            div { class: "motion-cascade motion-cascade-tight acct-ladder mb-6",
                for rung in ROLE_LADDER {
                    {
                        let width = ((rung.level as f32 / max_level) * 100.0).max(6.0);
                        rsx! {
                            div { class: "acct-ladder-rung",
                                span { class: "acct-ladder-dot", style: "--rung-accent: {rung.accent};" }
                                div { class: "acct-ladder-bar-wrap",
                                    div { class: "acct-ladder-top",
                                        p { class: "acct-ladder-name", { t_key(rung.name_key) } }
                                        span {
                                            class: "acct-ladder-level",
                                            style: "--rung-accent: {rung.accent};",
                                            { t!("accounts-roles-level", level: rung.level) }
                                        }
                                    }
                                    p { class: "acct-ladder-meta",
                                        { t!("accounts-roles-rung-meta", meta: t_key(rung.meta_key), members: t_key(rung.members_key)) }
                                    }
                                    div { class: "acct-ladder-bar-track",
                                        div {
                                            class: "acct-ladder-bar-fill",
                                            style: "width: {width}%; --rung-accent: {rung.accent};",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "acct-matrix",
                table { class: "acct-matrix-table",
                    thead {
                        tr {
                            th { { t!("accounts-roles-matrix-permission") } }
                            for rung in ROLE_LADDER {
                                th { { t_key(rung.name_key) } }
                            }
                        }
                    }
                    tbody {
                        for (i, perm) in PERMISSIONS.iter().enumerate() {
                            tr {
                                td { class: "acct-matrix-perm", "{perm}" }
                                for granted in GRANTS[i].iter() {
                                    td {
                                        span { class: if *granted { "acct-matrix-cell is-on" } else { "acct-matrix-cell" },
                                            "✓"
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
pub fn AccountsRoleNew() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let name = use_signal(String::new);
    let color = use_signal(|| ROLES_ACCENT.to_string());
    let mut level = use_signal(|| 10u8);
    let mut perms = use_signal(|| vec![false; PERMISSIONS.len()]);

    let granted_count = perms.read().iter().filter(|granted| **granted).count();
    let preview_name = if name().trim().is_empty() {
        t_key("accounts-roles-new-role-default")
    } else {
        name()
    };

    rsx! {
        div { class: "mb-6 flex flex-wrap items-center justify-between gap-3",
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    navigator.push(Route::AccountsRoles {});
                },
                { t!("accounts-roles-back") }
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::AccountsRoles {});
                },
                { t!("accounts-roles-save-role") }
            }
        }

        SectionIntro {
            eyebrow: t_key("accounts-roles-eyebrow"),
            title: t_key("accounts-roles-new-title"),
            description: t_key("accounts-roles-new-description"),
            accent: ROLES_ACCENT,
        }

        div { class: "acct-role-form",
            div { class: "flex flex-col gap-4",
                div { class: "ui-card p-4",
                    div { class: "stg-field",
                        label { class: "stg-field-label", { t!("accounts-roles-field-name") } }
                        SignalInput {
                            value: name,
                            placeholder: t_key("accounts-roles-placeholder-name"),
                        }
                    }
                    div { class: "stg-field",
                        label { class: "stg-field-label", { t!("accounts-roles-field-colour") } }
                        ColorPicker { value: color }
                    }
                    div { class: "stg-field",
                        label { class: "stg-field-label", { t!("accounts-roles-field-hierarchy", level: level()) } }
                        input {
                            r#type: "range",
                            min: "1",
                            max: "100",
                            value: "{level}",
                            class: "w-full",
                            oninput: move |evt| {
                                if let Ok(next) = evt.value().parse::<u8>() {
                                    level.set(next);
                                }
                            },
                        }
                    }
                }
                div { class: "ui-card p-4",
                    p { class: "mb-3 text-sm font-semibold text-text", { t!("accounts-roles-starting-permissions") } }
                    div { class: "motion-cascade motion-cascade-tight acct-role-perm-list",
                        for (i, perm) in PERMISSIONS.iter().enumerate() {
                            {
                                let checked = perms.read()[i];
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: if checked { "acct-role-perm-item is-on" } else { "acct-role-perm-item" },
                                        onclick: move |_| {
                                            perms.with_mut(|list| list[i] = !list[i]);
                                        },
                                        span { class: "acct-role-perm-check", "✓" }
                                        span { "{perm}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "acct-role-preview", style: "--role-accent: {color};",
                p { class: "mb-3 text-xs font-semibold uppercase tracking-wide text-text-muted",
                    { t!("accounts-roles-preview") }
                }
                span { class: "acct-role-preview-badge",
                    span { class: "acct-role-preview-badge-dot" }
                    "{preview_name}"
                }
                p { class: "acct-role-preview-hint",
                    { t!("accounts-roles-preview-hint", level: level(), count: granted_count) }
                }
            }
        }
    }
}
