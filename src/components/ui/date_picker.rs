use dioxus::prelude::*;

const MONTH_NAMES: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAY_LABELS: &[&str] = &["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

fn next_datepicker_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

fn parse_ymd(value: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<_> = value.trim().split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let day = parts[2].parse::<u32>().ok()?;
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return None;
    }
    Some((year, month, day))
}

fn format_ymd(year: i32, month: u32, day: u32) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

pub fn format_display_date(value: &str) -> String {
    let Some((year, month, day)) = parse_ymd(value) else {
        return value.trim().to_string();
    };
    let month_name = MONTH_NAMES
        .get((month as usize).saturating_sub(1))
        .copied()
        .unwrap_or("?");
    format!("{day} {month_name} {year}")
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn weekday_monday_first(year: i32, month: u32, day: u32) -> u32 {
    const TABLE: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    let sunday_first =
        (y + y / 4 - y / 100 + y / 400 + TABLE[(month as usize) - 1] + day as i32).rem_euclid(7);
    ((sunday_first + 6) % 7) as u32
}

fn shift_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let absolute = (year * 12) + (month as i32 - 1) + delta;
    let y = absolute.div_euclid(12);
    let m = (absolute.rem_euclid(12) + 1) as u32;
    (y, m)
}

fn today_fallback() -> (i32, u32, u32) {
    (2026, 7, 30)
}

fn use_datepicker_dismiss(mut open: Signal<bool>, picker_id: u64) {
    use_effect(move || {
        if !open() {
            return;
        }

        spawn(async move {
            let mut eval = document::eval(
                r#"
                const id = await dioxus.recv();
                await new Promise((resolve) => {
                    const onKey = (event) => {
                        if (event.key === "Escape") {
                            cleanup();
                            resolve(true);
                        }
                    };
                    const onPointer = (event) => {
                        const root = document.querySelector(`[data-ui-datepicker="${id}"]`);
                        if (root && root.contains(event.target)) {
                            return;
                        }
                        cleanup();
                        resolve(true);
                    };
                    const cleanup = () => {
                        document.removeEventListener("keydown", onKey, true);
                        document.removeEventListener("pointerdown", onPointer, true);
                    };
                    requestAnimationFrame(() => {
                        document.addEventListener("keydown", onKey, true);
                        document.addEventListener("pointerdown", onPointer, true);
                    });
                });
                true
                "#,
            );

            let _ = eval.send(picker_id.to_string());
            if eval.join::<bool>().await.unwrap_or(false) {
                open.set(false);
            }
        });
    });
}

#[component]
pub fn SignalDatePicker(
    mut value: Signal<String>,
    #[props(default, into)] placeholder: String,
    #[props(default = true)] allow_clear: bool,
    #[props(default, into)] class: String,
) -> Element {
    let mut open = use_signal(|| false);
    let picker_id = use_hook(next_datepicker_id);
    use_datepicker_dismiss(open, picker_id);

    let selected = parse_ymd(&value());
    let (today_y, today_m, today_d) = today_fallback();

    let initial = selected.unwrap_or((today_y, today_m, today_d));
    let mut view_year = use_signal(|| initial.0);
    let mut view_month = use_signal(|| initial.1);

    let is_open = open();
    let year = view_year();
    let month = view_month();
    let month_label = MONTH_NAMES
        .get((month as usize).saturating_sub(1))
        .copied()
        .unwrap_or("?");
    let leading = weekday_monday_first(year, month, 1) as usize;
    let days = days_in_month(year, month);
    let display = selected
        .map(|(y, m, d)| format_display_date(&format_ymd(y, m, d)))
        .unwrap_or_else(|| {
            if placeholder.is_empty() {
                String::from("Select date")
            } else {
                placeholder.clone()
            }
        });
    let show_placeholder = selected.is_none();

    rsx! {
        div {
            class: if is_open { "ui-datepicker is-open {class}" } else { "ui-datepicker {class}" },
            "data-ui-datepicker": "{picker_id}",
            button {
                r#type: "button",
                class: if is_open { "ui-datepicker-trigger ui-squircle is-open" } else { "ui-datepicker-trigger ui-squircle" },
                "aria-haspopup": "dialog",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |_| {
                    if !*open.peek() {
                        let focus = parse_ymd(&value()).unwrap_or((today_y, today_m, today_d));
                        view_year.set(focus.0);
                        view_month.set(focus.1);
                    }
                    let next = !*open.peek();
                    open.set(next);
                },
                span { class: if show_placeholder { "ui-datepicker-value is-placeholder" } else { "ui-datepicker-value" },
                    "{display}"
                }
                span { class: "ui-datepicker-icon", "aria-hidden": "true",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 16 16",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        rect {
                            x: "2.5",
                            y: "3.5",
                            width: "11",
                            height: "10",
                            rx: "1.5",
                        }
                        path { d: "M5 2.5v2.5M11 2.5v2.5M2.5 7h11" }
                    }
                }
            }
            if is_open {
                div {
                    class: "ui-datepicker-panel ui-squircle",
                    role: "dialog",
                    "aria-label": "Choose date",
                    div { class: "ui-datepicker-header",
                        button {
                            r#type: "button",
                            class: "ui-datepicker-nav",
                            "aria-label": "Previous month",
                            onclick: move |_| {
                                let (y, m) = shift_month(view_year(), view_month(), -1);
                                view_year.set(y);
                                view_month.set(m);
                            },
                            "‹"
                        }
                        p { class: "ui-datepicker-month", "{month_label} {year}" }
                        button {
                            r#type: "button",
                            class: "ui-datepicker-nav",
                            "aria-label": "Next month",
                            onclick: move |_| {
                                let (y, m) = shift_month(view_year(), view_month(), 1);
                                view_year.set(y);
                                view_month.set(m);
                            },
                            "›"
                        }
                    }
                    div { class: "ui-datepicker-weekdays",
                        for label in WEEKDAY_LABELS {
                            span { "{label}" }
                        }
                    }
                    div { class: "ui-datepicker-grid",
                        for _ in 0..leading {
                            span { class: "ui-datepicker-day is-empty" }
                        }
                        for day in 1..=days {
                            {
                                let is_selected = selected == Some((year, month, day));
                                let is_today = (year, month, day) == (today_y, today_m, today_d);
                                let class_name = match (is_selected, is_today) {
                                    (true, true) => "ui-datepicker-day is-selected is-today",
                                    (true, false) => "ui-datepicker-day is-selected",
                                    (false, true) => "ui-datepicker-day is-today",
                                    (false, false) => "ui-datepicker-day",
                                };
                                rsx! {
                                    button {
                                        key: "{day}",
                                        r#type: "button",
                                        class: "{class_name}",
                                        "aria-label": "{day}",
                                        "aria-pressed": if is_selected { "true" } else { "false" },
                                        onclick: move |_| {
                                            value.set(format_ymd(year, month, day));
                                            open.set(false);
                                        },
                                        "{day}"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "ui-datepicker-footer",
                        button {
                            r#type: "button",
                            class: "ui-datepicker-action",
                            onclick: move |_| {
                                let today = format_ymd(today_y, today_m, today_d);
                                value.set(today);
                                view_year.set(today_y);
                                view_month.set(today_m);
                                open.set(false);
                            },
                            "Today"
                        }
                        if allow_clear && selected.is_some() {
                            button {
                                r#type: "button",
                                class: "ui-datepicker-action is-muted",
                                onclick: move |_| {
                                    value.set(String::new());
                                    open.set(false);
                                },
                                "Clear"
                            }
                        }
                    }
                }
            }
        }
    }
}
