use dioxus::prelude::*;

use crate::components::page::StatPill;
use crate::components::settings::{IconKey, SectionIntro, ToggleField};
use crate::components::ui::*;
use crate::router::Route;

const AUTH_ACCENT: &str = "#5b9dff";
const STAFF_ACCENT: &str = "#69bdf2";
const ROLES_ACCENT: &str = "#fb923c";

struct AuthLane {
    name: &'static str,
    desc: &'static str,
    usage_pct: u8,
    enabled: bool,
    accent: &'static str,
}

const AUTH_LANES: &[AuthLane] = &[
    AuthLane {
        name: "Email & password",
        desc: "Default sign-in method for all new accounts.",
        usage_pct: 100,
        enabled: true,
        accent: "#5b9dff",
    },
    AuthLane {
        name: "Two-factor authentication",
        desc: "Authenticator app required for staff roles.",
        usage_pct: 62,
        enabled: true,
        accent: "#3ecf8e",
    },
    AuthLane {
        name: "Magic link",
        desc: "Passwordless sign-in via emailed one-time link.",
        usage_pct: 24,
        enabled: true,
        accent: "#87d1fe",
    },
    AuthLane {
        name: "Single sign-on (SAML)",
        desc: "Enterprise SSO for staff-only login.",
        usage_pct: 4,
        enabled: false,
        accent: "#c4b5fd",
    },
];

#[component]
pub fn AccountsAuth() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {AUTH_ACCENT};",
            SectionIntro {
                eyebrow: "Identity",
                title: "Authentication",
                description: "Sign-in methods, OAuth providers, and linking website accounts to in-game identities.",
                accent: AUTH_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        "Invite user"
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: "Accounts", value: "3,481", accent: AUTH_ACCENT }
                StatPill { label: "2FA enabled", value: "62%", accent: AUTH_ACCENT }
                StatPill {
                    label: "OAuth logins",
                    value: "842",
                    accent: AUTH_ACCENT,
                }
                StatPill {
                    label: "Linked players",
                    value: "1,204",
                    accent: AUTH_ACCENT,
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Sign-in methods" }
                        p { class: "stg-site-panel-sub",
                            "How players and staff authenticate on the website."
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
                                    p { class: "acct-lane-title", "{lane.name}" }
                                    p { class: "acct-lane-desc", "{lane.desc}" }
                                }
                                div { class: "acct-lane-usage",
                                    div { class: "acct-lane-usage-track",
                                        div {
                                            class: "acct-lane-usage-fill",
                                            style: "width: {lane.usage_pct}%;",
                                        }
                                    }
                                    p { class: "acct-lane-usage-label",
                                        "{lane.usage_pct}% of accounts"
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
                        h2 { class: "stg-site-panel-title", "Connections" }
                        p { class: "stg-site-panel-sub",
                            "External OAuth providers for login and account sync."
                        }
                    }
                    Button { size: ButtonSize::Sm,
                        IconPlus {}
                        "Add provider"
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
                        h2 { class: "stg-site-panel-title", "Account linking" }
                        p { class: "stg-site-panel-sub",
                            "Connect website accounts with in-game identities across servers."
                        }
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        "Generate codes"
                    }
                }
                div { class: "stg-site-panel-body",
                    div { class: "motion-cascade acct-flow mb-5",
                        AcctFlowStep {
                            num: "1",
                            title: "Generate a code",
                            desc: "Player requests a one-time link code from your website.",
                            connector: true,
                        }
                        AcctFlowStep {
                            num: "2",
                            title: "Enter it in-game",
                            desc: "Player types /link CODE on any connected server.",
                            connector: true,
                        }
                        AcctFlowStep {
                            num: "3",
                            title: "Confirmed",
                            desc: "Accounts are linked instantly and stay in sync.",
                            connector: false,
                        }
                    }
                    div { class: "mb-2 flex items-center justify-between gap-3",
                        h3 { class: "text-sm font-semibold text-text", "Recent links" }
                        span { class: "text-xs text-text-muted", "1,204 linked · 18 pending" }
                    }
                    div { class: "motion-cascade motion-cascade-tight acct-link-list",
                        AcctLinkRow {
                            badge: "MC",
                            title: "NovaCraft · Survival",
                            meta: "Minecraft · Verified with code N7K2",
                            status: "Linked",
                        }
                        AcctLinkRow {
                            badge: "MC",
                            title: "SkyBuilder · Skyblock",
                            meta: "Minecraft · Verified with code Q1M9",
                            status: "Linked",
                        }
                        AcctLinkRow {
                            badge: "MC",
                            title: "AetherFox · Creative",
                            meta: "Awaiting in-game confirmation",
                            status: "Pending",
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
    title: &'static str,
    desc: &'static str,
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
    status: &'static str,
) -> Element {
    rsx! {
        div { class: "acct-link-row",
            span { class: "acct-link-badge", "{badge}" }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-sm font-medium text-text", "{title}" }
                p { class: "mt-0.5 text-xs text-text-muted", "{meta}" }
            }
            span { class: "shrink-0 text-xs text-text-secondary", "{status}" }
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
    email: &'static str,
    role: &'static str,
    role_accent: &'static str,
    last_active: &'static str,
    status: &'static str,
}

const STAFF_MEMBERS: &[StaffMember] = &[
    StaffMember {
        initials: "AC",
        name: "Alex Chen",
        email: "alex@novacraft.gg",
        role: "Owner",
        role_accent: "#fb7185",
        last_active: "Just now",
        status: "Online",
    },
    StaffMember {
        initials: "JR",
        name: "Jordan Reyes",
        email: "jordan@novacraft.gg",
        role: "Admin",
        role_accent: "#fb923c",
        last_active: "12 min ago",
        status: "Online",
    },
    StaffMember {
        initials: "MK",
        name: "Maya Khan",
        email: "maya@novacraft.gg",
        role: "Admin",
        role_accent: "#fb923c",
        last_active: "2 hours ago",
        status: "Away",
    },
    StaffMember {
        initials: "TS",
        name: "Theo Santos",
        email: "theo@novacraft.gg",
        role: "Moderator",
        role_accent: "#5b9dff",
        last_active: "Yesterday",
        status: "Offline",
    },
    StaffMember {
        initials: "LP",
        name: "Lena Park",
        email: "lena@novacraft.gg",
        role: "Moderator",
        role_accent: "#5b9dff",
        last_active: "3 days ago",
        status: "Offline",
    },
];

struct StaffInvite {
    email: &'static str,
    role: &'static str,
    sent: &'static str,
}

const STAFF_INVITES: &[StaffInvite] = &[
    StaffInvite {
        email: "sam@novacraft.gg",
        role: "Moderator",
        sent: "2 hours ago",
    },
    StaffInvite {
        email: "ria@studio.gg",
        role: "Admin",
        sent: "Yesterday",
    },
];

#[component]
pub fn AccountsStaff() -> Element {
    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {STAFF_ACCENT};",
            SectionIntro {
                eyebrow: "Team",
                title: "Staff",
                description: "People with access to this admin panel, and outstanding invites.",
                accent: STAFF_ACCENT,
                action: rsx! {
                    Button {
                        IconPlus {}
                        "Invite staff"
                    }
                },
            }

            section { class: "motion-cascade stat-strip mb-6",
                StatPill { label: "Active staff", value: "5", accent: STAFF_ACCENT }
                StatPill {
                    label: "Pending invites",
                    value: "2",
                    accent: STAFF_ACCENT,
                }
                StatPill { label: "Admins", value: "3", accent: STAFF_ACCENT }
                StatPill {
                    label: "2FA coverage",
                    value: "100%",
                    accent: STAFF_ACCENT,
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Team roster" }
                        p { class: "stg-site-panel-sub",
                            "Staff accounts that can sign in to the admin panel."
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
                                    p { class: "acct-staff-email", "{member.email}" }
                                }
                                span {
                                    class: "acct-staff-role",
                                    style: "--staff-accent: {member.role_accent};",
                                    "{member.role}"
                                }
                                span { class: "acct-staff-meta", "{member.last_active}" }
                                span { class: if member.status == "Online" { "acct-staff-status is-online" } else if member.status == "Away" { "acct-staff-status is-away" } else { "acct-staff-status" },
                                    "{member.status}"
                                }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Pending invites" }
                        p { class: "stg-site-panel-sub",
                            "Invites expire after 7 days if they are not accepted."
                        }
                    }
                }
                div { class: "stg-site-panel-body stg-site-panel-body-flush",
                    div { class: "motion-cascade motion-cascade-tight acct-staff-list",
                        for invite in STAFF_INVITES {
                            div { class: "acct-staff-row",
                                span { class: "acct-staff-avatar is-invite", "··" }
                                div { class: "min-w-0 flex-1",
                                    p { class: "acct-staff-name", "{invite.email}" }
                                    p { class: "acct-staff-email", "Invited as {invite.role}" }
                                }
                                span { class: "acct-staff-meta", "Sent {invite.sent}" }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Sm,
                                    "Resend"
                                }
                            }
                        }
                    }
                }
            }

            section { class: "stg-site-panel",
                div { class: "stg-site-panel-head",
                    div { class: "min-w-0",
                        h2 { class: "stg-site-panel-title", "Access rules" }
                        p { class: "stg-site-panel-sub",
                            "Baseline requirements for anyone with a staff role."
                        }
                    }
                }
                div { class: "stg-site-panel-body",
                    ToggleField {
                        label: "Require two-factor authentication",
                        hint: "Staff must enable an authenticator app before accessing the panel.",
                        enabled: true,
                    }
                    ToggleField {
                        label: "Restrict staff login to known IPs",
                        hint: "Block admin sign-in from addresses outside your allowlist.",
                        enabled: false,
                    }
                    ToggleField {
                        label: "Notify owners on new staff invites",
                        hint: "Send an alert when anyone invites a new team member.",
                        enabled: true,
                    }
                }
            }
        }
    }
}

struct RoleRung {
    name: &'static str,
    meta: &'static str,
    level: u8,
    members: &'static str,
    accent: &'static str,
}

const ROLE_LADDER: &[RoleRung] = &[
    RoleRung {
        name: "Owner",
        meta: "Full access",
        level: 100,
        members: "1 member",
        accent: "#fb7185",
    },
    RoleRung {
        name: "Admin",
        meta: "Manage staff & settings",
        level: 80,
        members: "4 members",
        accent: "#fb923c",
    },
    RoleRung {
        name: "Moderator",
        meta: "Forums & tickets",
        level: 40,
        members: "11 members",
        accent: "#f5c14a",
    },
    RoleRung {
        name: "Helper",
        meta: "Limited support tools",
        level: 20,
        members: "18 members",
        accent: "#5b9dff",
    },
    RoleRung {
        name: "Member",
        meta: "Default community access",
        level: 1,
        members: "3,427 members",
        accent: "#87d1fe",
    },
];

const PERMISSIONS: &[&str] = &[
    "forums.create",
    "forums.delete",
    "tickets.manage",
    "news.publish",
    "applications.review",
    "analytics.view",
];

const GRANTS: &[[bool; 5]] = &[
    [true, true, true, false, true],
    [true, true, true, false, false],
    [true, true, true, true, false],
    [true, true, false, false, false],
    [true, true, false, false, false],
    [true, true, true, false, false],
];

#[component]
pub fn AccountsRoles() -> Element {
    let navigator = use_navigator();
    let max_level = 100f32;

    rsx! {
        div {
            class: "motion-cascade contents",
            style: "--stg-accent: {ROLES_ACCENT};",
            SectionIntro {
                eyebrow: "Access",
                title: "Permissions & roles",
                description: "Fully configurable permission system with hierarchy and groups.",
                accent: ROLES_ACCENT,
                action: rsx! {
                    Button {
                        onclick: move |_| {
                            navigator.push(Route::AccountsRoleNew {});
                        },
                        IconPlus {}
                        "Create role"
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
                                        p { class: "acct-ladder-name", "{rung.name}" }
                                        span {
                                            class: "acct-ladder-level",
                                            style: "--rung-accent: {rung.accent};",
                                            "Level {rung.level}"
                                        }
                                    }
                                    p { class: "acct-ladder-meta", "{rung.meta} · {rung.members}" }
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
                            th { "Permission" }
                            for rung in ROLE_LADDER {
                                th { "{rung.name}" }
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
    let navigator = use_navigator();
    let name = use_signal(String::new);
    let color = use_signal(|| ROLES_ACCENT.to_string());
    let mut level = use_signal(|| 10u8);
    let mut perms = use_signal(|| vec![false; PERMISSIONS.len()]);

    let granted_count = perms.read().iter().filter(|granted| **granted).count();
    let preview_name = if name().trim().is_empty() {
        String::from("New role")
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
                "← Roles"
            }
            Button {
                onclick: move |_| {
                    navigator.push(Route::AccountsRoles {});
                },
                "Save role"
            }
        }

        SectionIntro {
            eyebrow: "Access",
            title: "New role",
            description: "Define a name, colour, hierarchy level, and starting permissions.",
            accent: ROLES_ACCENT,
        }

        div { class: "acct-role-form",
            div { class: "flex flex-col gap-4",
                div { class: "ui-card p-4",
                    div { class: "stg-field",
                        label { class: "stg-field-label", "Role name" }
                        SignalInput {
                            value: name,
                            placeholder: "e.g. Community Manager",
                        }
                    }
                    div { class: "stg-field",
                        label { class: "stg-field-label", "Colour" }
                        ColorPicker { value: color }
                    }
                    div { class: "stg-field",
                        label { class: "stg-field-label", "Hierarchy level ({level})" }
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
                    p { class: "mb-3 text-sm font-semibold text-text", "Starting permissions" }
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
                    "Preview"
                }
                span { class: "acct-role-preview-badge",
                    span { class: "acct-role-preview-badge-dot" }
                    "{preview_name}"
                }
                p { class: "acct-role-preview-hint",
                    "Level {level} · {granted_count} permissions granted"
                }
            }
        }
    }
}
