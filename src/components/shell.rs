use dioxus::prelude::*;

use crate::components::brand::{favicon_svg, BrandMark};
use crate::components::page::{PageTransition, PoweredByFooter};
use crate::components::ui::*;
use crate::nav::{crumb_for, is_theme_editor, section_for, subnav_active, Section};
use crate::router::Route;
use crate::user::CurrentUser;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SheetAnim {
    Open,
    Closing,
}

#[component]
pub fn AppShell() -> Element {
    let mut sheet = use_signal(|| Option::<SheetAnim>::None);
    let route = use_route::<Route>();
    let navigator = use_navigator();
    let current_user = use_context::<Signal<CurrentUser>>();
    let account_email = current_user.read().email.clone();
    let section = section_for(&route);
    let crumb = crumb_for(&route);
    let theme_ide = is_theme_editor(&route);
    let mut favicon_accent = use_signal(|| Option::<&'static str>::None);
    let mut side_open = use_signal(|| true);

    use_effect(move || {
        let _route = router().current::<Route>();
        if matches!(*sheet.peek(), Some(SheetAnim::Open)) {
            sheet.set(Some(SheetAnim::Closing));
        }
    });

    use_effect(move || {
        let route = router().current::<Route>();
        let accent = section_for(&route).accent();
        if *favicon_accent.peek() == Some(accent) {
            return;
        }
        favicon_accent.set(Some(accent));

        let svg = favicon_svg(accent);
        let eval = document::eval(
            r#"
            const svg = await dioxus.recv();
            if (window.__ssFaviconUrl) {
              try { URL.revokeObjectURL(window.__ssFaviconUrl); } catch (_) {}
            }
            for (const el of [...document.querySelectorAll("link[rel~='icon']")]) {
              el.remove();
            }
            const url = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml" }));
            window.__ssFaviconUrl = url;
            const link = document.createElement("link");
            link.rel = "icon";
            link.type = "image/svg+xml";
            link.setAttribute("data-ss-favicon", "1");
            link.href = url;
            document.head.appendChild(link);
            "#,
        );
        let _ = eval.send(svg);
    });

    let sheet_anim = sheet();
    let backdrop_class = match sheet_anim {
        Some(SheetAnim::Open) => "nav-sheet-backdrop-enter",
        Some(SheetAnim::Closing) => "nav-sheet-backdrop-exit",
        None => "",
    };
    let panel_class = match sheet_anim {
        Some(SheetAnim::Open) => "nav-sheet-panel-enter",
        Some(SheetAnim::Closing) => "nav-sheet-panel-exit",
        None => "",
    };
    let account_avatar_class = if section == Section::Account {
        "ring-2 ring-accent"
    } else {
        "ring-1 ring-border-subtle"
    };
    let side_open_now = side_open();
    let side_panel_class = if side_open_now {
        "side-panel fixed inset-y-0 left-[68px] z-20 hidden flex-col border-r bg-bg lg:flex"
    } else {
        "side-panel side-panel-collapsed fixed inset-y-0 left-[68px] z-20 hidden flex-col border-r bg-bg lg:flex"
    };
    let content_pad = if side_open_now {
        "shell-main-pad flex h-full min-h-0 min-w-0 flex-col overflow-hidden md:pl-[68px] lg:pl-[calc(68px+14rem)]"
    } else {
        "shell-main-pad flex h-full min-h-0 min-w-0 flex-col overflow-hidden md:pl-[68px]"
    };
    let subnav_class = if side_open_now {
        "flex gap-1 overflow-x-auto border-t border-border-subtle/50 px-3 py-2 sm:px-5 md:px-8 lg:hidden scrollbar-none"
    } else {
        "flex gap-1 overflow-x-auto border-t border-border-subtle/50 px-3 py-2 sm:px-5 md:px-8 scrollbar-none"
    };
    let expand_btn_class = if side_open_now {
        "side-expand-btn side-expand-btn-hidden ui-btn ui-squircle ui-btn-ghost mb-3 hidden h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted lg:inline-flex"
    } else {
        "side-expand-btn ui-btn ui-squircle ui-btn-ghost mb-3 hidden h-9 w-9 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted lg:inline-flex"
    };

    rsx! {
        document::Title { "{section.document_title()}" }

        div {
            class: "relative h-dvh overflow-hidden bg-bg text-text",
            style: section.theme_vars(),

            aside {
                class: "fixed inset-y-0 left-0 z-20 hidden w-[68px] flex-col items-center overflow-y-auto border-r border-border-subtle bg-bg py-5 md:flex",
                button {
                    class: "mb-8 flex h-9 w-9 items-center justify-center rounded-squircle-sm transition-opacity hover:opacity-80",
                    onclick: move |_| {
                        navigator.push(Route::Dashboard {});
                    },
                    BrandMark { class: "h-9 w-9" }
                }
                button {
                    class: expand_btn_class,
                    aria_label: "Show sidebar",
                    title: "Show sidebar",
                    aria_hidden: side_open_now,
                    tabindex: if side_open_now { "-1" } else { "0" },
                    onclick: move |_| side_open.set(true),
                    span { class: "text-lg leading-none", "›" }
                }
                nav {
                    class: "flex flex-1 flex-col items-center gap-1",
                    for main in Section::ALL.iter().copied() {
                        RailNav {
                            key: "{main.label()}",
                            to: main.home(),
                            active: section == main,
                            icon: rail_icon_data(main),
                        }
                    }
                }
                div {
                    class: "mt-auto flex flex-col items-center gap-3 pb-1",
                    RailNav {
                        to: Section::Settings.home(),
                        active: section == Section::Settings,
                        icon: rail_icon_data(Section::Settings),
                    }
                    button {
                        class: "rounded-full transition-opacity hover:opacity-80",
                        aria_label: "Account",
                        onclick: move |_| {
                            navigator.push(Route::Account {});
                        },
                        Avatar {
                            email: account_email,
                            size: 32,
                            alt: "Account",
                            class: account_avatar_class,
                        }
                    }
                }
            }

            aside {
                class: side_panel_class,
                aria_hidden: !side_open_now,
                div {
                    class: "side-panel-inner flex h-full flex-col overflow-y-auto px-4 py-6",
                    div {
                        class: "mb-5 flex items-start justify-between gap-2",
                        div {
                            class: "min-w-0 px-2",
                            p { class: "mb-1 text-xs font-medium uppercase tracking-wide text-text-muted", "Section" }
                            p {
                                class: "text-lg font-semibold tracking-tight text-accent",
                                "{section.label()}"
                            }
                        }
                        button {
                            class: "ui-btn ui-squircle ui-btn-ghost inline-flex h-9 w-9 shrink-0 cursor-pointer items-center justify-center p-0 font-semibold text-text-muted",
                            aria_label: "Hide sidebar",
                            title: "Hide sidebar",
                            tabindex: if side_open_now { "0" } else { "-1" },
                            onclick: move |_| side_open.set(false),
                            span { class: "text-lg leading-none", "‹" }
                        }
                    }
                    nav {
                        class: "flex flex-col gap-0.5",
                        for sub in section.subs().iter().copied() {
                            SideNav {
                                to: sub.route,
                                label: sub.label,
                                active: subnav_active(&route, sub.route),
                            }
                        }
                    }
                }
            }

            div {
                class: content_pad,

                header {
                    class: "z-30 shrink-0 border-b border-border-subtle/60 bg-bg/90 backdrop-blur-md",
                    ShellHeaderBar {
                        section,
                        crumb,
                        sheet,
                    }

                    nav {
                        class: subnav_class,
                        for sub in section.subs().iter().copied() {
                            SubChip {
                                to: sub.route,
                                label: sub.label,
                                active: subnav_active(&route, sub.route),
                            }
                        }
                    }
                }

                main {
                    class: if theme_ide {
                        "flex min-h-0 flex-1 flex-col overflow-hidden p-0"
                    } else {
                        "flex min-h-0 flex-1 flex-col overflow-y-auto px-4 pt-4 sm:px-6 sm:pt-6 md:px-8"
                    },
                    div {
                        class: if theme_ide {
                            "flex min-h-0 flex-1 flex-col"
                        } else {
                            "flex-1"
                        },
                        PageTransition {}
                    }
                    if !theme_ide {
                        PoweredByFooter {}
                    }
                }
            }

            if sheet_anim.is_some() {
                MobileNavSheet {
                    section,
                    route,
                    sheet,
                    backdrop_class,
                    panel_class,
                }
            }
        }
    }
}

#[component]
fn MobileNavSheet(
    section: Section,
    route: Route,
    mut sheet: Signal<Option<SheetAnim>>,
    backdrop_class: &'static str,
    panel_class: &'static str,
) -> Element {
    let backdrop = match backdrop_class {
        "nav-sheet-backdrop-enter" => "absolute inset-0 bg-black/50 nav-sheet-backdrop-enter",
        "nav-sheet-backdrop-exit" => "absolute inset-0 bg-black/50 nav-sheet-backdrop-exit",
        _ => "absolute inset-0 bg-black/50",
    };
    let panel = match panel_class {
        "nav-sheet-panel-enter" => {
            "absolute inset-y-0 left-0 flex w-[min(100%,18.5rem)] flex-col border-r border-border-subtle bg-bg shadow-xl nav-sheet-panel-enter"
        }
        "nav-sheet-panel-exit" => {
            "absolute inset-y-0 left-0 flex w-[min(100%,18.5rem)] flex-col border-r border-border-subtle bg-bg shadow-xl nav-sheet-panel-exit"
        }
        _ => {
            "absolute inset-y-0 left-0 flex w-[min(100%,18.5rem)] flex-col border-r border-border-subtle bg-bg shadow-xl"
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-40 md:hidden",
            button {
                class: backdrop,
                aria_label: "Close menu",
                onclick: move |_| {
                    if matches!(*sheet.peek(), Some(SheetAnim::Open)) {
                        sheet.set(Some(SheetAnim::Closing));
                    }
                },
            }
            div {
                class: panel,
                onanimationend: move |_| {
                    if matches!(*sheet.peek(), Some(SheetAnim::Closing)) {
                        sheet.set(None);
                    }
                },
                div {
                    class: "flex h-14 items-center justify-between gap-3 border-b border-border-subtle px-4",
                    div {
                        class: "flex items-center gap-3",
                        BrandMark { class: "h-7 w-7" }
                        span { class: "text-base font-semibold tracking-tight", "ServerSpot" }
                    }
                    IconButton {
                        onclick: move |_| {
                            if matches!(*sheet.peek(), Some(SheetAnim::Open)) {
                                sheet.set(Some(SheetAnim::Closing));
                            }
                        },
                        IconClose {}
                    }
                }

                div {
                    class: "flex-1 overflow-y-auto px-3 py-4",
                    p { class: "mb-2 px-2 text-xs font-medium uppercase tracking-wide text-text-muted", "Sections" }
                    nav {
                        class: "mb-6 flex flex-col gap-0.5",
                        for main in Section::ALL.iter().copied().chain(std::iter::once(Section::Settings)) {
                            MobileSectionNav {
                                section: main,
                                active: section == main,
                            }
                        }
                    }

                    p {
                        class: "mb-2 px-2 text-xs font-medium uppercase tracking-wide text-text-muted",
                        "{section.label()}"
                    }
                    nav {
                        class: "flex flex-col gap-0.5",
                        for sub in section.subs().iter().copied() {
                            SideNav {
                                to: sub.route,
                                label: sub.label,
                                active: subnav_active(&route, sub.route),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ShellHeaderBar(
    section: Section,
    crumb: &'static str,
    mut sheet: Signal<Option<SheetAnim>>,
) -> Element {
    let mut search_open = use_signal(|| false);
    let mut search = use_signal(String::new);
    let navigator = use_navigator();
    let route = use_route::<Route>();

    use_effect(move || {
        let _ = route;
        if *search_open.peek() {
            search_open.set(false);
            search.write().clear();
        }
    });

    rsx! {
        div {
            class: "flex h-14 items-center gap-2 px-3 sm:gap-3 sm:px-5 md:px-8",

            if search_open() {
                IconButton {
                    class: "shrink-0",
                    onclick: move |_| {
                        search_open.set(false);
                        search.write().clear();
                    },
                    IconClose {}
                }
                SearchInput {
                    class: "min-w-0 flex-1",
                    value: search,
                    placeholder: "Search…",
                }
            } else {
                IconButton {
                    class: "shrink-0 md:hidden",
                    onclick: move |_| sheet.set(Some(SheetAnim::Open)),
                    IconMenu {}
                }

                button {
                    class: "flex h-8 w-8 shrink-0 items-center justify-center md:hidden",
                    onclick: move |_| {
                        navigator.push(Route::Dashboard {});
                    },
                    BrandMark { class: "h-7 w-7" }
                }

                div {
                    class: "min-w-0 flex-1",
                    p {
                        class: "truncate text-sm font-medium text-text md:hidden",
                        "{section.label()}"
                        span { class: "font-normal text-text-muted", " / {crumb}" }
                    }
                    div {
                        class: "hidden min-w-0 items-center gap-2 text-sm text-text-muted md:flex",
                        Link {
                            to: section.home(),
                            class: "truncate opacity-70 transition-opacity hover:opacity-100 text-text-muted",
                            "{section.label()}"
                        }
                        span { class: "opacity-40", "/" }
                        span { class: "truncate text-text-secondary", "{crumb}" }
                    }
                }

                div {
                    class: "flex shrink-0 items-center gap-0.5 sm:gap-1",
                    IconButton {
                        onclick: move |_| search_open.set(true),
                        IconSearch {}
                    }
                    IconButton {
                        class: "max-lg:hidden",
                        IconBell {}
                    }
                    IconButton {
                        class: "max-lg:hidden",
                        onclick: move |_| {
                            navigator.push(Route::SettingsIntegrations {});
                        },
                        IconGlobe {}
                    }
                    Button {
                        class: "ml-1 max-sm:hidden",
                        size: ButtonSize::Sm,
                        IconPlus {}
                        "New"
                    }
                    IconButton {
                        class: "sm:hidden",
                        IconPlus {}
                    }
                }
            }
        }
    }
}

fn rail_icon_data(section: Section) -> HugeIconData {
    use crate::components::ui::hugeicon::*;
    match section {
        Section::Dashboard => DASHBOARD_SQUARE_01,
        Section::Store => STORE_01,
        Section::Forum => MESSAGE_01,
        Section::Support => TICKET_01,
        Section::Content => NEWS,
        Section::Players => USER_GROUP,
        Section::Leaderboards => CHART_LINE_DATA_01,
        Section::Votes => GIFT,
        Section::Applications => CUSTOMER_SERVICE_01,
        Section::Analytics => ANALYTICS_UP,
        Section::Settings => SETTINGS_01,
        Section::Account => USER_GROUP,
    }
}

#[component]
fn RailNav(to: Route, active: bool, icon: HugeIconData) -> Element {
    let navigator = use_navigator();
    let dest = to;
    let class = if active {
        "ui-btn-rail-accent"
    } else {
        "text-text-muted"
    };

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::IconSm,
            class,
            onclick: move |_| {
                navigator.push(dest);
            },
            HugeIcon { icon }
        }
    }
}

#[component]
fn SideNav(to: Route, label: &'static str, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = to;
    let class = if active {
        "ui-btn-feature-color justify-start"
    } else {
        "justify-start text-text-muted"
    };

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::Sm,
            full_width: true,
            class,
            onclick: move |_| {
                navigator.push(dest);
            },
            "{label}"
        }
    }
}

#[component]
fn SubChip(to: Route, label: &'static str, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = to;
    let class = if active {
        "ui-btn-feature-color shrink-0"
    } else {
        "shrink-0 text-text-muted"
    };

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::Sm,
            class,
            onclick: move |_| {
                navigator.push(dest);
            },
            "{label}"
        }
    }
}

#[component]
fn MobileSectionNav(section: Section, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = section.home();
    let class = if active {
        "ui-btn-feature-color justify-start gap-2.5"
    } else {
        "justify-start gap-2.5 text-text-muted"
    };

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::Sm,
            full_width: true,
            class,
            onclick: move |_| {
                navigator.push(dest);
            },
            span { class: "opacity-80", HugeIcon { icon: rail_icon_data(section) } }
            span { "{section.label()}" }
        }
    }
}
