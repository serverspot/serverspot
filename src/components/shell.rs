use dioxus::prelude::*;

use crate::components::page::{PageTransition, PoweredByFooter};
use crate::components::ui::*;
use crate::gravatar::CURRENT_USER_EMAIL;
use crate::nav::{crumb_for, section_for, Section};
use crate::router::Route;

const LOGO: Asset = asset!("/assets/logo.svg");

#[derive(Clone, Copy, PartialEq, Eq)]
enum SheetAnim {
    Open,
    Closing,
}

#[component]
pub fn AppShell() -> Element {
    let mut sheet = use_signal(|| Option::<SheetAnim>::None);
    let mut search_open = use_signal(|| false);
    let mut search = use_signal(|| String::new());
    let route = use_route::<Route>();
    let navigator = use_navigator();
    let section = section_for(&route);
    let crumb = crumb_for(&route);
    let nav_home = navigator.clone();
    let nav_home_mobile = navigator.clone();
    let nav_settings = navigator.clone();

    use_effect(move || {
        let _ = route;
        if matches!(*sheet.peek(), Some(SheetAnim::Open)) {
            sheet.set(Some(SheetAnim::Closing));
        }
        if *search_open.peek() {
            search_open.set(false);
            search.set(String::new());
        }
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

    rsx! {
        div {
            class: "flex min-h-dvh bg-bg text-text",

            aside {
                class: "sticky top-0 hidden h-dvh w-[68px] shrink-0 flex-col items-center self-start overflow-y-auto border-r border-border-subtle py-5 md:flex",
                button {
                    class: "mb-8 flex h-9 w-9 items-center justify-center rounded-squircle-sm transition-opacity hover:opacity-80",
                    onclick: move |_| {
                        nav_home.push(Route::Dashboard {});
                    },
                    img {
                        src: LOGO,
                        alt: "ServerSpot",
                        class: "h-9 w-auto",
                    }
                }
                nav {
                    class: "flex flex-1 flex-col items-center gap-1",
                    for main in Section::ALL.iter().copied() {
                        RailNav {
                            to: main.home(),
                            active: section == main,
                            icon: rail_icon(main),
                        }
                    }
                }
                div {
                    class: "mt-auto flex flex-col items-center gap-3 pb-1",
                    RailNav {
                        to: Section::Settings.home(),
                        active: section == Section::Settings,
                        icon: rsx! { IconSettings {} },
                    }
                    Avatar {
                        email: CURRENT_USER_EMAIL,
                        size: 32,
                        alt: "Account",
                        class: "ring-1 ring-border-subtle",
                    }
                }
            }

            aside {
                class: "sticky top-0 hidden h-dvh w-56 shrink-0 flex-col self-start overflow-y-auto border-r border-border-subtle px-4 py-6 lg:flex",
                p { class: "mb-1 px-2 text-xs font-medium uppercase tracking-wide text-text-muted", "Section" }
                p { class: "mb-5 px-2 text-lg font-semibold tracking-tight", "{section.label()}" }
                nav {
                    class: "flex flex-col gap-0.5",
                    for sub in section.subs().iter().copied() {
                        SideNav {
                            to: sub.route,
                            label: sub.label,
                            active: route == sub.route,
                        }
                    }
                }
            }

            div {
                class: "flex min-h-dvh min-w-0 flex-1 flex-col",

                header {
                    class: "sticky top-0 z-30 border-b border-border-subtle/60 bg-bg/90 backdrop-blur-md",
                    div {
                        class: "flex h-14 items-center gap-2 px-3 sm:gap-3 sm:px-5 md:px-8",

                        if search_open() {
                            IconButton {
                                class: "shrink-0",
                                onclick: move |_| {
                                    search_open.set(false);
                                    search.set(String::new());
                                },
                                IconClose {}
                            }
                            SearchInput {
                                class: "min-w-0 flex-1",
                                value: search(),
                                placeholder: "Search…",
                                oninput: move |evt: FormEvent| search.set(evt.value()),
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
                                    nav_home_mobile.push(Route::Dashboard {});
                                },
                                img {
                                    src: LOGO,
                                    alt: "ServerSpot",
                                    class: "h-7 w-auto",
                                }
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
                                        nav_settings.push(Route::SettingsIntegrations {});
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

                    nav {
                        class: "flex gap-1 overflow-x-auto border-t border-border-subtle/50 px-3 py-2 sm:px-5 md:px-8 lg:hidden scrollbar-none",
                        for sub in section.subs().iter().copied() {
                            SubChip {
                                to: sub.route,
                                label: sub.label,
                                active: route == sub.route,
                            }
                        }
                    }
                }

                main {
                    class: "flex flex-1 flex-col px-4 pt-4 sm:px-6 sm:pt-6 md:px-8",
                    div {
                        class: "flex-1",
                        PageTransition {}
                    }
                    PoweredByFooter {}
                }
            }

            if sheet_anim.is_some() {
                div {
                    class: "fixed inset-0 z-40 md:hidden",
                    button {
                        class: "absolute inset-0 bg-black/50 {backdrop_class}",
                        aria_label: "Close menu",
                        onclick: move |_| {
                            if matches!(*sheet.peek(), Some(SheetAnim::Open)) {
                                sheet.set(Some(SheetAnim::Closing));
                            }
                        },
                    }
                    div {
                        class: "absolute inset-y-0 left-0 flex w-[min(100%,18.5rem)] flex-col border-r border-border-subtle bg-bg shadow-xl {panel_class}",
                        onanimationend: move |_| {
                            if matches!(*sheet.peek(), Some(SheetAnim::Closing)) {
                                sheet.set(None);
                            }
                        },
                        div {
                            class: "flex h-14 items-center justify-between gap-3 border-b border-border-subtle px-4",
                            div {
                                class: "flex items-center gap-3",
                                img {
                                    src: LOGO,
                                    alt: "ServerSpot",
                                    class: "h-7 w-auto",
                                }
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

                            p { class: "mb-2 px-2 text-xs font-medium uppercase tracking-wide text-text-muted", "{section.label()}" }
                            nav {
                                class: "flex flex-col gap-0.5",
                                for sub in section.subs().iter().copied() {
                                    SideNav {
                                        to: sub.route,
                                        label: sub.label,
                                        active: route == sub.route,
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

fn rail_icon(section: Section) -> Element {
    match section {
        Section::Dashboard => rsx! { IconGrid {} },
        Section::Store => rsx! { IconStore {} },
        Section::Forum => rsx! { IconForum {} },
        Section::Support => rsx! { IconTicket {} },
        Section::Content => rsx! { IconNews {} },
        Section::Community => rsx! { IconUsers {} },
        Section::Analytics => rsx! { IconAnalytics {} },
        Section::Settings => rsx! { IconSettings {} },
    }
}

#[component]
fn RailNav(to: Route, active: bool, icon: Element) -> Element {
    let navigator = use_navigator();
    let dest = to.clone();

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::IconSm,
            class: if active { "text-accent" } else { "text-text-muted" },
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            {icon}
        }
    }
}

#[component]
fn SideNav(to: Route, label: &'static str, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = to.clone();

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::Sm,
            full_width: true,
            class: if active {
                "justify-start text-accent"
            } else {
                "justify-start text-text-muted"
            },
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            "{label}"
        }
    }
}

#[component]
fn SubChip(to: Route, label: &'static str, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = to.clone();

    rsx! {
        button {
            class: if active {
                "shrink-0 rounded-squircle-sm bg-surface-2 px-3 py-1.5 text-xs font-medium text-accent"
            } else {
                "shrink-0 rounded-squircle-sm px-3 py-1.5 text-xs font-medium text-text-muted transition-colors hover:bg-surface/60 hover:text-text"
            },
            onclick: move |_| {
                navigator.push(dest.clone());
            },
            "{label}"
        }
    }
}

#[component]
fn MobileSectionNav(section: Section, active: bool) -> Element {
    let navigator = use_navigator();
    let dest = section.home();

    rsx! {
        Button {
            variant: if active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
            size: ButtonSize::Sm,
            full_width: true,
            class: if active {
                "justify-start gap-2.5 text-accent"
            } else {
                "justify-start gap-2.5 text-text-muted"
            },
            onclick: move |_| {
                navigator.push(dest);
            },
            span { class: "opacity-80", {rail_icon(section)} }
            span { "{section.label()}" }
        }
    }
}
