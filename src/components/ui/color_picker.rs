use dioxus::prelude::*;

/// Shared accent palette used across admin colour pickers.
pub const DEFAULT_COLOR_PRESETS: &[&str] = &[
    "#69bdf2", "#5b9dff", "#3ecf8e", "#5eead4", "#f5c14a", "#f0a35e", "#f071a5", "#fb7185",
];

fn normalize_hex(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::from("#000000");
    }
    if trimmed.starts_with('#') {
        trimmed.to_string()
    } else {
        format!("#{trimmed}")
    }
}

/// Browser `<input type="color">` expects `#rrggbb`.
fn color_input_value(hex: &str) -> String {
    let normalized = normalize_hex(hex).to_ascii_lowercase();
    let digits = normalized.trim_start_matches('#');
    match digits.len() {
        3 => {
            let mut expanded = String::from("#");
            for ch in digits.chars() {
                expanded.push(ch);
                expanded.push(ch);
            }
            expanded
        }
        6 if digits.chars().all(|ch| ch.is_ascii_hexdigit()) => format!("#{digits}"),
        _ => String::from("#000000"),
    }
}

fn colors_match(a: &str, b: &str) -> bool {
    color_input_value(a).eq_ignore_ascii_case(&color_input_value(b))
}

#[component]
pub fn ColorPicker(
    mut value: Signal<String>,
    #[props(default = DEFAULT_COLOR_PRESETS)] presets: &'static [&'static str],
    #[props(default = true)] show_hex: bool,
) -> Element {
    let current = value();
    let picker_value = color_input_value(&current);
    let is_custom = !presets.iter().any(|preset| colors_match(&current, preset));

    rsx! {
        div {
            class: "space-y-2",
            div {
                class: "flex flex-wrap items-center gap-2",
                for preset in presets {
                    button {
                        r#type: "button",
                        class: if colors_match(&current, preset) {
                            "h-8 w-8 shrink-0 cursor-pointer rounded-squircle-sm outline outline-2 outline-offset-2 outline-white/70"
                        } else {
                            "h-8 w-8 shrink-0 cursor-pointer rounded-squircle-sm outline outline-1 outline-offset-1 outline-white/10 hover:outline-white/30"
                        },
                        style: "background: {preset};",
                        title: "{preset}",
                        "aria-label": "Select {preset}",
                        onclick: {
                            let color = (*preset).to_string();
                            move |_| value.set(color.clone())
                        },
                    }
                }
                label {
                    class: if is_custom {
                        "relative ml-0.5 inline-flex h-8 w-8 shrink-0 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm outline outline-2 outline-offset-2 outline-white/70"
                    } else {
                        "relative ml-0.5 inline-flex h-8 w-8 shrink-0 cursor-pointer items-center justify-center overflow-hidden rounded-squircle-sm border border-dashed border-border-subtle bg-surface/20 text-text-muted hover:border-border hover:text-text-secondary"
                    },
                    style: if is_custom {
                        format!("background: {picker_value};")
                    } else {
                        String::new()
                    },
                    title: "Custom colour",
                    "aria-label": "Custom colour",
                    if !is_custom {
                        span { class: "pointer-events-none text-sm font-semibold leading-none", "+" }
                    }
                    input {
                        r#type: "color",
                        class: "absolute inset-0 cursor-pointer opacity-0",
                        value: "{picker_value}",
                        oninput: move |evt: FormEvent| {
                            value.set(normalize_hex(&evt.value()));
                        },
                    }
                }
            }
            if show_hex {
                div {
                    class: "flex max-w-[11rem] items-center gap-2",
                    span {
                        class: "h-8 w-8 shrink-0 rounded-squircle-sm border border-border-subtle",
                        style: "background: {picker_value};",
                        "aria-hidden": "true",
                    }
                    input {
                        r#type: "text",
                        class: "ui-input ui-squircle h-8 min-w-0 flex-1 px-3 font-mono text-xs uppercase outline-none",
                        value: "{current}",
                        spellcheck: "false",
                        autocomplete: "off",
                        maxlength: "7",
                        "aria-label": "Hex colour",
                        oninput: move |evt: FormEvent| {
                            let next = evt.value();
                            if next.is_empty() || next.starts_with('#') {
                                value.set(next);
                            } else {
                                value.set(format!("#{next}"));
                            }
                        },
                        onblur: move |_| {
                            let normalized = color_input_value(&value());
                            value.set(normalized);
                        },
                    }
                }
            }
        }
    }
}
