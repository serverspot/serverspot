use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;

use crate::components::brand::BrandMark;
use crate::components::page::PoweredByFooter;
use crate::components::ui::*;
use crate::router::Route;

#[component]
pub fn Login() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut show_password = use_signal(|| false);

    rsx! {
        div { class: "login-page",
            div { class: "login-atmosphere", "aria-hidden": "true" }
            div { class: "login-grid", "aria-hidden": "true" }

            div { class: "login-stage",

                header { class: "login-hero",
                    BrandMark { class: "login-mark" }
                    h1 { class: "login-brand-name", { t!("brand-name") } }
                    p { class: "login-hero-line", { t!("login-admin-panel") } }
                }

                form {
                    class: "login-form",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        navigator.push(Route::LoginOtp {});
                    },

                    div { class: "login-field",
                        label { class: "login-label", r#for: "login-username", { t!("login-field-username") } }
                        input {
                            id: "login-username",
                            r#type: "text",
                            autocomplete: "username",
                            class: "login-input",
                            placeholder: "admin",
                            spellcheck: "false",
                        }
                    }

                    div { class: "login-field",
                        div { class: "login-field-top",
                            label { class: "login-label", r#for: "login-password", { t!("login-field-password") } }
                            button {
                                r#type: "button",
                                class: "login-link",
                                onclick: move |_| {
                                    navigator.push(Route::LoginReset {});
                                },
                                { t!("login-forgot-password") }
                            }
                        }
                        div { class: "login-password",
                            input {
                                id: "login-password",
                                r#type: if show_password() { "text" } else { "password" },
                                autocomplete: "current-password",
                                class: "login-input",
                                placeholder: "••••••••",
                            }
                            button {
                                r#type: "button",
                                class: "login-reveal",
                                "aria-label": if show_password() { t!("login-password-hide-aria") } else { t!("login-password-show-aria") },
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() {
                                    { t!("login-password-hide") }
                                } else {
                                    { t!("login-password-show") }
                                }
                            }
                        }
                    }

                    label { class: "login-remember",
                        input { r#type: "checkbox", class: "login-checkbox" }
                        span { { t!("login-remember-me") } }
                    }

                    Button {
                        full_width: true,
                        onclick: move |_| {
                            navigator.push(Route::LoginOtp {});
                        },
                        { t!("login-sign-in") }
                    }

                    div { class: "login-divider", { t!("login-divider-or") } }

                    button { r#type: "button", class: "login-oauth",
                        DiscordMark {}
                        { t!("login-oauth-discord") }
                    }
                }

                p { class: "login-foot", { t!("login-footer-need-access") } }
            }

            PoweredByFooter {}
        }
    }
}

#[component]
pub fn LoginOtp() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut code = use_signal(String::new);
    let ready = use_memo(move || code.read().len() == 6);

    rsx! {
        div { class: "login-page",
            div { class: "login-atmosphere", "aria-hidden": "true" }
            div { class: "login-grid", "aria-hidden": "true" }

            div { class: "login-stage",
                header { class: "login-hero",
                    BrandMark { class: "login-mark" }
                    h1 { class: "login-brand-name", { t!("login-otp-title") } }
                    p { class: "login-hero-line", { t!("login-otp-subtitle") } }
                }

                form {
                    class: "login-form",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        if code.read().len() == 6 {
                            navigator.push(Route::Dashboard {});
                        }
                    },

                    p { class: "login-otp-copy",
                        { t!("login-otp-instructions") }
                    }

                    label { class: "login-otp-field", r#for: "login-otp",
                        span { class: "sr-only", { t!("login-otp-field-aria") } }
                        div { class: "login-otp-cells", "aria-hidden": "true",
                            for i in 0..6 {
                                {
                                    let ch = code.read().chars().nth(i);
                                    let filled = ch.is_some();
                                    let active = code.read().len() == i;
                                    rsx! {
                                        span {
                                            class: if filled {
                                                "login-otp-cell is-filled"
                                            } else if active {
                                                "login-otp-cell is-active"
                                            } else {
                                                "login-otp-cell"
                                            },
                                            if let Some(digit) = ch {
                                                "{digit}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        input {
                            id: "login-otp",
                            class: "login-otp-input",
                            r#type: "text",
                            inputmode: "numeric",
                            autocomplete: "one-time-code",
                            maxlength: "6",
                            spellcheck: "false",
                            autofocus: true,
                            value: "{code}",
                            oninput: move |evt| {
                                let digits: String = evt
                                    .value()
                                    .chars()
                                    .filter(|c| c.is_ascii_digit())
                                    .take(6)
                                    .collect();
                                let done = digits.len() == 6;
                                code.set(digits);
                                if done {
                                    navigator.push(Route::Dashboard {});
                                }
                            },
                        }
                    }

                    Button {
                        full_width: true,
                        disabled: !ready(),
                        onclick: move |_| {
                            if code.read().len() == 6 {
                                navigator.push(Route::Dashboard {});
                            }
                        },
                        { t!("login-otp-verify") }
                    }

                    div { class: "login-otp-actions",
                        button { r#type: "button", class: "login-link", { t!("login-otp-resend") } }
                        button {
                            r#type: "button",
                            class: "login-link",
                            onclick: move |_| {
                                navigator.push(Route::Login {});
                            },
                            { t!("login-back-to-sign-in") }
                        }
                    }
                }

                p { class: "login-foot", { t!("login-otp-footer") } }
            }

            PoweredByFooter {}
        }
    }
}

#[component]
pub fn LoginReset() -> Element {
    let _lang = i18n();
    let navigator = use_navigator();
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut code = use_signal(String::new);
    let mut show_password = use_signal(|| false);
    let ready = use_memo(move || {
        !username.read().trim().is_empty()
            && code.read().len() == 6
            && !password.read().is_empty()
            && password() == confirm()
    });

    rsx! {
        div { class: "login-page",
            div { class: "login-atmosphere", "aria-hidden": "true" }
            div { class: "login-grid", "aria-hidden": "true" }

            div { class: "login-stage",
                header { class: "login-hero",
                    BrandMark { class: "login-mark" }
                    h1 { class: "login-brand-name", { t!("login-reset-title") } }
                    p { class: "login-hero-line", { t!("login-admin-panel") } }
                }

                form {
                    class: "login-form",
                    onsubmit: move |evt| {
                        evt.prevent_default();
                        if ready() {
                            navigator.push(Route::Login {});
                        }
                    },

                    p { class: "login-otp-copy",
                        { t!("login-reset-instructions") }
                    }

                    div { class: "login-field",
                        label { class: "login-label", r#for: "reset-username", { t!("login-field-username") } }
                        input {
                            id: "reset-username",
                            r#type: "text",
                            autocomplete: "username",
                            class: "login-input",
                            placeholder: "admin",
                            spellcheck: "false",
                            value: "{username}",
                            oninput: move |evt| username.set(evt.value()),
                        }
                    }

                    div { class: "login-field",
                        span { class: "login-label", { t!("login-reset-field-code") } }
                        label { class: "login-otp-field", r#for: "reset-otp",
                            div { class: "login-otp-cells", "aria-hidden": "true",
                                for i in 0..6 {
                                    {
                                        let ch = code.read().chars().nth(i);
                                        let filled = ch.is_some();
                                        let active = code.read().len() == i;
                                        rsx! {
                                            span {
                                                class: if filled {
                                                    "login-otp-cell is-filled"
                                                } else if active {
                                                    "login-otp-cell is-active"
                                                } else {
                                                    "login-otp-cell"
                                                },
                                                if let Some(digit) = ch {
                                                    "{digit}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            input {
                                id: "reset-otp",
                                class: "login-otp-input",
                                r#type: "text",
                                inputmode: "numeric",
                                autocomplete: "one-time-code",
                                maxlength: "6",
                                spellcheck: "false",
                                value: "{code}",
                                oninput: move |evt| {
                                    let digits: String = evt
                                        .value()
                                        .chars()
                                        .filter(|c| c.is_ascii_digit())
                                        .take(6)
                                        .collect();
                                    code.set(digits);
                                },
                            }
                        }
                    }

                    div { class: "login-field",
                        label { class: "login-label", r#for: "reset-password", { t!("login-reset-field-new-password") } }
                        div { class: "login-password",
                            input {
                                id: "reset-password",
                                r#type: if show_password() { "text" } else { "password" },
                                autocomplete: "new-password",
                                class: "login-input",
                                placeholder: "••••••••",
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                            button {
                                r#type: "button",
                                class: "login-reveal",
                                "aria-label": if show_password() { t!("login-password-hide-aria") } else { t!("login-password-show-aria") },
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() {
                                    { t!("login-password-hide") }
                                } else {
                                    { t!("login-password-show") }
                                }
                            }
                        }
                    }

                    div { class: "login-field",
                        label { class: "login-label", r#for: "reset-confirm", { t!("login-reset-field-confirm-password") } }
                        input {
                            id: "reset-confirm",
                            r#type: if show_password() { "text" } else { "password" },
                            autocomplete: "new-password",
                            class: "login-input",
                            placeholder: "••••••••",
                            value: "{confirm}",
                            oninput: move |evt| confirm.set(evt.value()),
                        }
                    }

                    Button {
                        full_width: true,
                        disabled: !ready(),
                        onclick: move |_| {
                            if ready() {
                                navigator.push(Route::Login {});
                            }
                        },
                        { t!("login-reset-submit") }
                    }

                    div { class: "login-otp-actions",
                        button { r#type: "button", class: "login-link", { t!("login-otp-resend") } }
                        button {
                            r#type: "button",
                            class: "login-link",
                            onclick: move |_| {
                                navigator.push(Route::Login {});
                            },
                            { t!("login-back-to-sign-in") }
                        }
                    }
                }

                p { class: "login-foot", { t!("login-reset-footer") } }
            }

            PoweredByFooter {}
        }
    }
}

#[component]
fn DiscordMark() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            "aria-hidden": "true",
            path { d: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.09.09 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.09 16.09 0 0 0-4.8 0c-.14-.34-.37-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.02.06.03.09.02c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02zM8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12zm6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12z" }
        }
    }
}
