use dioxus::prelude::*;

use crate::components::ui::*;
use crate::nav::{crumb_for, section_for, Section};
use crate::router::Route;

#[component]
pub fn AppShell() -> Element {
    let mut search_open = use_signal(|| false);
    let mut search = use_signal(|| String::new());
    let route = use_route::<Route>();
    let navigator = use_navigator();
    let section = section_for(&route);
    let crumb = crumb_for(&route);
    let nav_home = navigator.clone();
    let nav_grid = navigator.clone();
    let nav_settings = navigator.clone();

    rsx! {
        div {
            class: "flex min-h-dvh bg-bg text-text",

            aside {
                class: "sticky top-0 hidden h-dvh w-[68px] shrink-0 flex-col items-center self-start overflow-y-auto border-r border-border-subtle py-5 md:flex",
                div {
                    class: "mb-8 flex h-9 w-9 items-center justify-center",
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Icon,
                        class: "h-9 w-9 text-sm",
                        onclick: move |_| {
                            nav_home.push(Route::Dashboard {});
                        },
                        "S"
                    }
                }
                nav {
                    class: "flex flex-1 flex-col items-center gap-1",
                    for main in Section::all().iter().copied() {
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
                    div {
                        class: "h-8 w-8 rounded-full",
                        style: "background: linear-gradient(135deg, #60a5fa, #a78bfa, #f472b6);",
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
                class: "flex min-w-0 flex-1 flex-col",

                header {
                    class: "flex items-center justify-between gap-4 px-6 py-4 md:px-8",
                    div {
                        class: "flex items-center gap-2 text-sm text-text-muted",
                        Link {
                            to: section.home(),
                            class: "opacity-70 hover:opacity-100 transition-opacity text-text-muted",
                            "{section.label()}"
                        }
                        span { class: "opacity-40", "/" }
                        span { class: "text-text-secondary", "{crumb}" }
                    }

                    div {
                        class: "flex items-center gap-2",
                        if search_open() {
                            SearchInput {
                                class: "w-44 sm:w-56",
                                value: search(),
                                placeholder: "Search…",
                                oninput: move |evt: FormEvent| search.set(evt.value()),
                            }
                        }
                        div {
                            class: "flex items-center gap-1.5",
                            IconButton {
                                onclick: move |_| search_open.set(!search_open()),
                                IconSearch {}
                            }
                            IconButton {
                                onclick: move |_| {
                                    nav_grid.push(Route::Dashboard {});
                                },
                                IconGrid {}
                            }
                            IconButton { IconBell {} }
                            IconButton {
                                onclick: move |_| {
                                    nav_settings.push(Route::SettingsIntegrations {});
                                },
                                IconGlobe {}
                            }
                        }
                        Button {
                            IconPlus {}
                            "New"
                        }
                    }
                }

                main {
                    class: "flex-1 px-6 pb-12 md:px-8",
                    Outlet::<Route> {}
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
            size: ButtonSize::Icon,
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
